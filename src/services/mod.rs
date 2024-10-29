pub mod terraform;

pub trait ClusterInitializer {
    fn init(&self) -> anyhow::Result<()>;
}
