use crosstown_bus::{CrosstownBus, MessageHandler, HandleError, QueueProperties};
use borsh::{BorshDeserialize, BorshSerialize};
use std::time::Duration;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedEventHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedEventHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler: {:?}", message);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let listener = CrosstownBus::new_subscriber("amqp://guest:guest@localhost:5672".to_owned()).unwrap();

    let _ = listener.subscribe(
        "user_created".to_owned(),
        UserCreatedEventHandler{},
        QueueProperties::default()
    );

    println!("Subscriber sudah siap dan sedang mendengarkan antrean 'user_created'...");

    let ten_millis = Duration::from_millis(10);

    loop {
        // std::thread::sleep(ten_millis);
    }
}