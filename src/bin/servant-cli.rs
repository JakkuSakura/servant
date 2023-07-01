use common_libs::*;
use servant::list_projects;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LogLevel::Info)?;
    let projects = list_projects(Path::new("etc"))?;
    info!("Projects: {:#?}", projects);
    Ok(())
}
