use anyhow::{Result, bail};
use clap::Parser;
use phoxal_core_engine::DriverRuntimeArgs;
use phoxal_infra_helpers::init_tracing;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;
    let args = DriverRuntimeArgs::parse();
    let robot = args.runtime.robot()?;
    let binding = robot.driver_binding(&args.component_id)?;

    error!(
        component_id = %binding.component_id,
        "OAK-D Lite physical component driver is not implemented"
    );
    bail!(
        "OAK-D Lite physical component driver is not implemented for component '{}'",
        binding.component_id
    )
}
