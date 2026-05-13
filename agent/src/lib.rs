//! SystemGuard Agent
pub mod config;
pub mod detector;
pub mod ebpf;
pub mod sender;

use anyhow::{Context, Result};
use tracing::{info, warn};

pub struct Agent {
    config: config::AgentConfig,
    ebpf_manager: ebpf::EbpfManager,
    detector: detector::AnomalyDetector,
    sender: sender::EventSender,
}

impl Agent {
    pub async fn new(config: config::AgentConfig) -> Result<Self> {
        info!("Initializing Agent v{}", env!("CARGO_PKG_VERSION"));
        Ok(Self {
            ebpf_manager: ebpf::EbpfManager::load().context("eBPF load failed")?,
            detector: detector::AnomalyDetector::new(&config)?,
            sender: sender::EventSender::new(config.collector_url.clone())?,
            config,
        })
    }
}
