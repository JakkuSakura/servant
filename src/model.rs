use common_libs::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceConfig {
    #[serde(default)]
    pub addr: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Service {
    pub project_name: String,
    pub service_name: String,
    pub config: ServiceConfig,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub services: Vec<Service>,
}
