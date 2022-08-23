use std::fmt::Debug;

use log::{error, info};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod jsonrpc;

use super::{model::*, Daemon, DaemonError};

pub trait Client {
    type Error: Into<DaemonError> + Debug;
    fn request<S: Serialize + Debug, D: DeserializeOwned + Debug>(
        &self,
        method: &str,
        params: Option<S>,
    ) -> Result<D, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Minisafed<C: Client> {
    client: C,
}

impl<C: Client> Minisafed<C> {
    pub fn new(client: C) -> Minisafed<C> {
        Minisafed { client }
    }

    /// Generic call function for RPC calls.
    fn call<T: Serialize + Debug, U: DeserializeOwned + Debug>(
        &self,
        method: &str,
        input: Option<T>,
    ) -> Result<U, DaemonError> {
        info!("{}", method);
        self.client.request(method, input).map_err(|e| {
            error!("method {} failed: {:?}", method, e);
            e.into()
        })
    }
}

impl<C: Client + Debug> Daemon for Minisafed<C> {
    fn is_external(&self) -> bool {
        true
    }

    fn stop(&mut self) -> Result<(), DaemonError> {
        let _res: serde_json::value::Value = self.call("stop", Option::<Request>::None)?;
        Ok(())
    }

    fn get_info(&self) -> Result<GetInfoResult, DaemonError> {
        self.call("getinfo", Option::<Request>::None)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}