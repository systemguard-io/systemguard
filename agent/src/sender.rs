use super::detector::ScoredEvent;
pub struct EventSender { url: String, client: reqwest::Client }
impl EventSender {
    pub fn new(url: String) -> anyhow::Result<Self> { Ok(Self{url, client: reqwest::Client::new()}) }
    pub async fn send(&self, e: ScoredEvent) -> anyhow::Result<()> { if e.risk_score>50 { tracing::warn!("High risk {} {}", e.event.comm, e.risk_score); } Ok(()) }
}
