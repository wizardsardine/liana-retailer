pub mod app;
pub mod daemon;
pub mod datadir;
pub mod download;
pub mod hw;
pub mod installer;
pub mod launcher;
pub mod lianalite;
pub mod loader;
pub mod logger;
pub mod node;
pub mod signer;
pub mod utils;

use liana::Version;

pub const VERSION: Version = Version {
    major: 7,
    minor: 0,
    patch: 0,
};

const RETAILER_NAME: &str = "21st Capital";

#[derive(Debug, Clone)]
pub struct RetailerVersion(Version);

impl std::fmt::Display for RetailerVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.0, RETAILER_NAME)
    }
}

pub const RETAILER_VERSION: RetailerVersion = RetailerVersion(VERSION);
