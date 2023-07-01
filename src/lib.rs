use crate::model::{Project, Service};
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
            let project_name = project_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            projects.push(Project {
                services: list_services(&project_name, &project_path)?,
                name: project_name,
            });
        }
    }
    Ok(projects)
}
fn list_services(project_name: &str, project_path: &Path) -> Result<Vec<Service>> {
    let mut services = vec![];
    for entry in project_path.read_dir()? {
        let entry = entry?;
        let service_path = entry.path();
        if service_path.is_dir() {
            let service_name = service_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            services.push(Service {
                project_name: project_name.to_owned(),
                service_name,
            });
        }
    }
    Ok(services)
}
