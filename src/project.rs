use crate::model::{Project, ProjectConfig};
use crate::service::list_services;
use common_libs::*;
use std::path::Path;

pub fn list_projects(root_path: &Path) -> Result<Vec<Project>> {
    let mut projects = vec![];
    for entry in root_path.read_dir()? {
        let entry = entry?;
        let project_path = entry.path();
        if project_path.is_dir() {
            let project = load_project(&project_path)?;
            projects.push(project);
        }
    }
    Ok(projects)
}
pub fn load_project_config(path: &Path) -> eyre::Result<ProjectConfig> {
    let config = std::fs::read_to_string(&path)?;
    let config: Value = serde_json::from_str(&config)?;

    let config: ProjectConfig = serde_json::from_value(config)?;

    Ok(config)
}
pub fn load_project(path: &Path) -> Result<Project> {
    let project_name = path.file_name().unwrap().to_str().unwrap().to_string();
    Ok(Project {
        services: list_services(&project_name, &path)?,
        name: project_name,
        config: load_project_config(&path.join("config.json"))?,
    })
}
impl Project {
    pub fn print_project(&self) -> Result<()> {
        println!("Project: {}", self.name);
        for s in &self.services {
            println!("  Service: {}", s.service_name);
            println!("  Systemd: {}", s.gen_systemd_service(&self.config.user))
        }
        Ok(())
    }
}
