pub mod terraform;

use std::borrow::Cow;

use anyhow::Result;
use thiserror::Error;

pub trait ClusterInitializer {
    fn init(&self) -> Result<()>;
}

#[derive(Error, Debug)]
pub enum ClusterError {
    #[error("Terraform init failed, module path {module_path:?}")]
    Init { module_path: Cow<'static, str> },
    #[error("Terraform apply failed, module path {module_path:?}")]
    Apply { module_path: Cow<'static, str> },
}
