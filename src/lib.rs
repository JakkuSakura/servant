use crate::model::{Project, Service, ServiceConfig};
use common_libs::*;
use std::path::Path;

pub mod model;
pub mod systemd;

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
pub fn load_project(path: &Path) -> Result<Project> {
    let project_name = path.file_name().unwrap().to_str().unwrap().to_string();
    Ok(Project {
        services: list_services(&project_name, &path)?,
        name: project_name,
    })
}
pub fn list_services(project_name: &str, project_path: &Path) -> Result<Vec<Service>> {
    let mut services = vec![];
    for entry in project_path.read_dir()? {
        let entry = entry?;
        let service_path = entry.path();
        if service_path.is_dir() {
            let service = load_service(
                project_name,
                service_path.file_name().unwrap().to_str().unwrap(),
                &service_path,
            )?;
            services.push(service);
        }
    }
    Ok(services)
}
pub fn load_service_config(path: &Path) -> Result<ServiceConfig> {
    let config = std::fs::read_to_string(&path)?;
    let config: Value = serde_json::from_str(&config)?;

    let config: ServiceConfig = serde_json::from_value(config)?;

    Ok(config)
}
pub fn load_service(project_name: &str, service_name: &str, path: &Path) -> Result<Service> {
    let config = load_service_config(&path.join("config.json"))?;
    Ok(Service {
        project_name: project_name.to_owned(),
        service_name: service_name.to_owned(),
        config,
    })
}
