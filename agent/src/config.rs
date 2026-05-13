use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct AgentConfig {
    pub collector_url: String,
    pub host_id: String,
    pub learning_mode_days: u32,
    pub batch_size: usize,
}
impl Default for AgentConfig {
    fn default() -> Self {
        Self { collector_url: "http://localhost:9090".into(), host_id: "localhost".into(), learning_mode_days: 7, batch_size: 100 }
    }
}
