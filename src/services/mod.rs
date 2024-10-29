pub mod terraform;

use anyhow::Result;

pub trait ClusterInitializer {
    fn init(&self) -> Result<(), ClusterError>;
}

pub enum ClusterError {
    Init(anyhow::Error),
}
