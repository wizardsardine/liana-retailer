import logging
import os
import shutil

from bip32.utils import coincurve
from bip380.descriptors import Descriptor
from bip380.miniscript import SatisfactionMaterial
from test_framework.utils import (
    UnixDomainSocketRpc,
    TailableProc,
    VERBOSE,
    LOG_LEVEL,
    MINISAFED_PATH,
    wait_for,
)
from test_framework.serializations import (
    PSBT,
    sighash_all_witness,
    CTxInWitness,
    CScriptWitness,
    PSBT_IN_BIP32_DERIVATION,
    PSBT_IN_WITNESS_SCRIPT,
    PSBT_IN_PARTIAL_SIG,
    PSBT_IN_FINAL_SCRIPTWITNESS,
)


class Minisafed(TailableProc):
    def __init__(
        self,
        datadir,
        owner_hd,
        multi_desc,
        bitcoind_rpc_port,
        bitcoind_cookie_path,
    ):
        TailableProc.__init__(self, datadir, verbose=VERBOSE)

        self.datadir = datadir
        self.prefix = os.path.split(datadir)[-1]

        self.owner_hd = owner_hd
        self.multi_desc = multi_desc
        self.receive_desc, self.change_desc = multi_desc.singlepath_descriptors()

        self.conf_file = os.path.join(datadir, "config.toml")
        self.cmd_line = [MINISAFED_PATH, "--conf", f"{self.conf_file}"]
        socket_path = os.path.join(os.path.join(datadir, "regtest"), "minisafed_rpc")
        self.rpc = UnixDomainSocketRpc(socket_path)

        with open(self.conf_file, "w") as f:
            f.write(f"data_dir = '{datadir}'\n")
            f.write("daemon = false\n")
            f.write(f"log_level = '{LOG_LEVEL}'\n")

            f.write(f'main_descriptor = "{multi_desc}"\n')

            f.write("[bitcoin_config]\n")
            f.write('network = "regtest"\n')
            f.write("poll_interval_secs = 1\n")

            f.write("[bitcoind_config]\n")
            f.write(f"cookie_path = '{bitcoind_cookie_path}'\n")
            f.write(f"addr = '127.0.0.1:{bitcoind_rpc_port}'\n")

    def sign_psbt(self, psbt):
        """Sign a transaction using the owner's key.
        This will fill the 'partial_sigs' field of all inputs.

        :param psbt: PSBT of the transaction to be signed.
        :returns: PSBT with a signature in each input for the owner's key.
        """
        assert isinstance(psbt, PSBT)

        # Sign each input.
        for i, psbt_in in enumerate(psbt.i):
            # First, gather the needed information from the PSBT input.
            # 'hd_keypaths' is of the form {pubkey: (fingerprint (4 bytes), derivation path (n * 4 bytes))}
            fing_der = next(iter(psbt_in.map[PSBT_IN_BIP32_DERIVATION].values()))
            raw_der_path = fing_der[4:]
            der_path = [
                int.from_bytes(raw_der_path[i : i + 4], byteorder="little", signed=True)
                for i in range(0, len(raw_der_path), 4)
            ]
            script_code = psbt_in.map[PSBT_IN_WITNESS_SCRIPT]

            # Now sign the transaction with the key of the "owner" (the participant that
            # can sign immediately without a timelock)
            sighash = sighash_all_witness(script_code, psbt, i)
            privkey = coincurve.PrivateKey(
                self.owner_hd.get_privkey_from_path(der_path)
            )
            pubkey = privkey.public_key.format()
            assert pubkey in psbt_in.map[PSBT_IN_BIP32_DERIVATION].keys(), (
                der_path,
                fing_der,
                pubkey,
                psbt_in.map[PSBT_IN_BIP32_DERIVATION].keys(),
            )
            sig = privkey.sign(sighash, hasher=None) + b"\x01"
            logging.debug(
                f"Adding signature {sig.hex()} for pubkey {pubkey.hex()} (path {der_path})"
            )
            assert PSBT_IN_PARTIAL_SIG not in psbt_in.map
            psbt_in.map[PSBT_IN_PARTIAL_SIG] = {pubkey: sig}

        return psbt

    def finalize_psbt(self, psbt):
        """Create a valid witness for all inputs in the PSBT.
        This will fail if the PSBT input does not contain enough material.

        :param psbt: PSBT of the transaction to be finalized.
        :returns: PSBT with finalized inputs.
        """
        assert isinstance(psbt, PSBT)

        # Create a witness for each input of the transaction.
        for i, psbt_in in enumerate(psbt.i):
            # First, gather the needed information from the PSBT input.
            # 'hd_keypaths' is of the form {pubkey: (fingerprint, derivation index)}
            fing_der = next(iter(psbt_in.map[PSBT_IN_BIP32_DERIVATION].values()))
            raw_der_path = fing_der[4:]
            der_path = [
                int.from_bytes(raw_der_path[i : i + 4], byteorder="little", signed=True)
                for i in range(0, len(raw_der_path), 4)
            ]
            assert len(der_path) == 2

            # Create a copy of the descriptor to derive it at the index used in this input.
            # Then create a satisfaction for it using the signature we just created.
            desc = Descriptor.from_str(
                str(self.receive_desc if der_path[0] == 0 else self.change_desc)
            )
            desc.derive(der_path[1])
            sat_material = SatisfactionMaterial(
                signatures=psbt_in.map[PSBT_IN_PARTIAL_SIG],
            )
            stack = desc.satisfy(sat_material)
            logging.debug(f"Satisfaction for {desc} is {[e.hex() for e in stack]}")

            # Update the transaction inside the PSBT directly.
            assert stack is not None
            psbt_in.map[PSBT_IN_FINAL_SCRIPTWITNESS] = CTxInWitness(
                CScriptWitness(stack)
            )
            psbt.tx.wit.vtxinwit.append(psbt_in.map[PSBT_IN_FINAL_SCRIPTWITNESS])

        return psbt

    def restart_fresh(self, bitcoind):
        """Delete the internal state of the wallet and restart."""
        self.stop()
        dir_path = os.path.join(self.datadir, "regtest")
        shutil.rmtree(dir_path)
        wallet_path = os.path.join(dir_path, "minisafed_watchonly_wallet")
        bitcoind.node_rpc.unloadwallet(wallet_path)
        self.start()
        wait_for(
            lambda: self.rpc.getinfo()["blockheight"] == bitcoind.rpc.getblockcount()
        )

    def start(self):
        TailableProc.start(self)
        self.wait_for_logs(
            [
                "Database initialized and checked",
                "Connection to bitcoind established and checked.",
                "JSONRPC server started.",
            ]
        )

    def stop(self, timeout=5):
        try:
            self.rpc.stop()
            self.wait_for_log(
                "Stopping the minisafe daemon.",
            )
            self.proc.wait(timeout)
        except Exception as e:
            logging.error(f"{self.prefix} : error when calling stop: '{e}'")
        return TailableProc.stop(self)

    def cleanup(self):
        try:
            self.stop()
        except Exception:
            self.proc.kill()
