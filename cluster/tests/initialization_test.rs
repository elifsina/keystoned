use cluster::terraform::TerraformInitializer;
use cluster::ClusterInitializer;
use std::path::Path;

#[test]
fn initialize_terraform_module() {
    let initializer = TerraformInitializer::new(Path::new("tests/terraform"));
    assert!(initializer.and_then(|init| init.init()).is_ok());
}
