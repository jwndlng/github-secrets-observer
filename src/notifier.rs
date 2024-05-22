
pub struct Notifier;


// Notifier is the component that notifies the user about the expiration of the secrets
// It will support multiple mechanism to do so.
// Planned mechanisms:
// - Slack Webhook
// - Simple console output
// - GitHub Log Annotation

impl Notifier {
    pub fn new() -> Notifier {
        Notifier
    }

    pub async fn notify(&mut self) {
        println!("Notifying...");
    }
}