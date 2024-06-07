use std::sync::Arc;
use alfred_rs::connection::{Publisher, Subscriber};
use alfred_rs::error::Error;
use alfred_rs::message::Message;
use alfred_rs::module::Module;
use alfred_rs::tokio;
use alfred_rs::tokio::sync::Mutex;
use alfred_idroid01::Drivers;

const MODULE_NAME: &'static str = "idroid01";
const INPUT_TOPIC: &'static str = "idroid01";


async fn manage_input_topic(topic: String, message: &mut Message, drivers: &Arc<Mutex<Drivers>>, module: &mut Module) {
    let text = message.text.clone();
    let response = match text.as_str() {
        "head" => drivers.lock().await.head.get_status(),
        /*"base" => drivers.lock().await.base.get_status(),
        "arms" => drivers.lock().await.arms.get_status(),
        "hand" => drivers.lock().await.hand.get_status(),*/
        _ => format!("Unknown command {text}")
    };
    let response_topic = message.response_topics.pop_front().unwrap();
    message.text = response;
    module.publish(response_topic, &message).await.unwrap();

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut module = Module::new(MODULE_NAME.to_string()).await?;
    let drivers = Arc::new(Mutex::new(Drivers::new("/dev/i2c-1")));
    module.subscribe(INPUT_TOPIC.to_string()).await?;
    loop {
        let (topic, mut message) = module.get_message().await?;
        match topic.as_str() {
            INPUT_TOPIC => {
                manage_input_topic(topic, &mut message, &drivers, &mut module).await;
            },
            _ => {}
        }
    }

}
