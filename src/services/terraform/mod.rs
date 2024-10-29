use std::{path::Path, process::Command};

pub struct TerraformInitializer {
    module_dir: Path,
}

impl TerraformInitializer {
    pub fn new(module_dir: Path) -> anyhow::Result<Self> {
        if !module_dir.is_dir() {
            return Err(anyhow!("path is not a directory"));
        }
        Ok(Self { module_dir })
    }
}

impl ClusterInitializer for TerraformInitializer {
    fn init(&self) -> anyhow::Result<()> {
        let mut cmd = Command::new("terraform")
            .arg("init")
            .current_dir(self.module_dir);
        Ok(cmd
            .status()?
            .success()
            .then_some(())
            .ok_or_else(|| anyhow!("failed to initialize terraform"))?)
    }
}
