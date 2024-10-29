use anyhow::anyhow;
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::ClusterError;
use super::ClusterInitializer;

pub struct TerraformInitializer {
    module_dir: PathBuf,
}

impl TerraformInitializer {
    pub fn new(module_dir: &Path) -> Result<Self> {
        if !module_dir.is_dir() {
            return Err(anyhow!("path is not a directory"));
        }
        Ok(Self {
            module_dir: module_dir.to_path_buf(),
        })
    }
}

impl ClusterInitializer for TerraformInitializer {
    fn init(&self) -> Result<(), ClusterError> {
        Command::new("terraform")
            .arg("init")
            .current_dir(&self.module_dir)
            .status()
            .map(|_| ())
            .map_err(|err| ClusterError::Init(anyhow!(err)))
    }
}
