use futures::Stream;
pub struct EbpfManager;
pub struct RawEvent { pub timestamp: u64, pub pid: u32, pub uid: u32, pub comm: String, pub syscall_id: u32, pub path: String }
impl EbpfManager { pub fn load() -> anyhow::Result<Self> { Ok(Self) } pub fn event_stream(&self) -> impl Stream<Item=RawEvent> { futures::stream::empty() } }
