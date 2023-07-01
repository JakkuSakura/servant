use crate::model::{Service, ServiceConfig};
use serde_json::Value;
use std::path::Path;

pub fn list_services(project_name: &str, project_path: &Path) -> eyre::Result<Vec<Service>> {
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

pub fn load_service_config(path: &Path) -> eyre::Result<ServiceConfig> {
    let config = std::fs::read_to_string(&path)?;
    let config: Value = serde_json::from_str(&config)?;

    let config: ServiceConfig = serde_json::from_value(config)?;

    Ok(config)
}

pub fn load_service(project_name: &str, service_name: &str, path: &Path) -> eyre::Result<Service> {
    let config = load_service_config(&path.join("config.json"))?;
    Ok(Service {
        project_name: project_name.to_owned(),
        service_name: service_name.to_owned(),
        config,
    })
}
