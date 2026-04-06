/// WhatsApp integration — no DB access, pure service.
///
/// Runs as a tokio background task inside the Axum process.
/// Uses the pairing code flow: user enters an 8-digit code in WhatsApp → Linked Devices.
pub struct WhatsAppService;

impl WhatsAppService {
    pub fn new() -> Self {
        Self
    }

    /// Start the WhatsApp listener as a background task.
    /// Returns the 8-digit pairing code the user must enter in WhatsApp.
    pub async fn start(&self) -> anyhow::Result<String> {
        // TODO: integrate whatsapp-rust crate
        // 1. generate pairing code
        // 2. spawn tokio task that listens for incoming messages
        // 3. on each message, check for Spotify URLs and emit an event
        todo!("WhatsApp listener not yet implemented")
    }
}
