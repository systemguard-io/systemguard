use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct SecurityEvent { pub timestamp: u64, pub pid: u32, pub uid: u32, pub comm: String, pub syscall: u32, pub path: String }
#[derive(Debug)]
pub struct ScoredEvent { pub event: SecurityEvent, pub risk_score: u8, pub reasons: Vec<String> }
pub struct AnomalyDetector { baseline: HashMap<String, u64>, learning_mode: bool }
impl AnomalyDetector {
    pub fn new(_c: &super::config::AgentConfig) -> anyhow::Result<Self> { Ok(Self { baseline: HashMap::new(), learning_mode: true }) }
    pub fn should_send(&self, e: &SecurityEvent) -> bool {!e.path.starts_with("/proc/") }
    pub fn score(&mut self, e: SecurityEvent) -> ScoredEvent {
        let key = format!("{}:{}", e.comm, e.path);
        let cnt = self.baseline.entry(key).or_insert(0); *cnt += 1;
        let mut score = 0; let mut reasons = vec![];
        if (e.comm=="nginx"||e.comm=="apache2") && e.path.contains("/etc/shadow") { score=95; reasons.push("web server priv access".into()); }
        if e.syscall==59 && e.path.starts_with("/tmp/") { score=score.max(70); reasons.push("exec from /tmp".into()); }
        ScoredEvent { event: e, risk_score: score, reasons }
    }
}
