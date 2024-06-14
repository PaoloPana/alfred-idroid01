use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use alfred_rs::connection::{Receiver, Sender};
use alfred_rs::error::Error;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::{debug, error, warn};
use alfred_rs::message::Message;
use alfred_rs::pubsub_connection::{AlfredPublisher, AlfredSubscriber};
use alfred_rs::tokio;
use alfred_rs::tokio::sync::Mutex;
use alfred_idroid01::Drivers;

const MODULE_NAME: &'static str = "idroid01";
const INPUT_TOPIC: &'static str = "idroid01";

async fn manage_input_messages(publisher: &Arc<Mutex<AlfredPublisher>>, subscriber: &Arc<Mutex<AlfredSubscriber>>, drivers: &Arc<Mutex<Drivers>>) -> Result<(), Error> {
    let (topic, message) = subscriber.lock().await.receive().await?;
    let drivers = drivers.clone();
    match topic.as_str() {
        INPUT_TOPIC => {
            let result = drivers.lock().await.get_command(message.text.clone()).unwrap_or(format!("Unknown command {}", message.text));
            debug!("{}", result);
            let mut response = message.clone();
            response.text = result;
            if response.response_topics.len() == 0 {
                warn!("No response topics found!");
            }
            let response_topic = response.response_topics.pop_front().unwrap(); // TODO check validity
            publisher.lock().await.send(response_topic, &response).await.inspect_err(|err| error!("{err}")).unwrap();
        },
        _ => {}
    }
    Ok(())
}

async fn manage_device_events(publisher: &Arc<Mutex<AlfredPublisher>>, drivers: &Arc<Mutex<Drivers>>, watcher_commands: Vec<String>, devices_statuses: &mut HashMap<String, String>) -> Result<(), Error> {
    for command in watcher_commands {
        let result = drivers.lock().await.get_command(command.clone()).unwrap();
        let previous = devices_statuses.insert(command, result.clone()).unwrap_or(result.clone()).clone();
        if result != previous {
            let mut message = Message::empty();
            message.text = result;
            // TODO: move event topic to config.toml file
            publisher.lock().await.send("events.idroid01".to_string(), &message).await.expect("Error on send message");
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let module = InterfaceModule::new(MODULE_NAME.to_string()).await?;
    let subscriber = Arc::new(Mutex::new(module.connection.subscriber));
    let publisher1 = Arc::new(Mutex::new(module.connection.publisher));
    let publisher2 = publisher1.clone();

    let drivers = Arc::new(Mutex::new(Drivers::new("/dev/i2c-1")));
    let drivers2 = drivers.clone();
    subscriber.lock().await.listen(INPUT_TOPIC.to_string()).await?;
    // TODO: load from config.toml file
    let watcher_commands = vec!["head touch".to_string()];

    tokio::spawn(async move {
        let drivers = drivers2.clone();
        async move {
            let mut devices_statuses = HashMap::new();
            loop {
                manage_device_events(&publisher1, &drivers, watcher_commands.clone(), &mut devices_statuses).await.unwrap();
            }
        }.await;
    });

    loop {
        manage_input_messages(&publisher2, &subscriber, &drivers.clone()).await.unwrap();
    }
}
