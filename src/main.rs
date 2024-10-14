use std::collections::HashMap;
use std::string::ToString;
use std::sync::Arc;
use std::time::Duration;
use alfred_rs::connection::{Receiver, Sender};
use alfred_rs::error::Error;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::{debug, error, warn};
use alfred_rs::message::{Message, MessageType};
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
            let (response_topic, response) = message.reply(result.clone(), MessageType::TEXT)?;
            publisher.lock().await.send(&response_topic, &response).await.inspect_err(|err| error!("{err}")).unwrap();
        },
        _ => {}
    }
    Ok(())
}

async fn manage_device_events(publisher: &Arc<Mutex<AlfredPublisher>>, drivers: &Arc<Mutex<Drivers>>, watcher_commands: Vec<String>, devices_statuses: &mut HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut found_difference = false;
    let mut found_error = false;
    for command in watcher_commands {
        let command_str = command.as_str();
        match drivers.lock().await.get_command(command_str.to_string()) {
            Err(err) => {
                found_error = true;
                warn!("Error on event {}: {}", command_str, err);
            },
            Ok(result) => {
                //debug!("Result on event {}: {}", command_str, result);
                let previous = devices_statuses.insert(command_str.to_string(), result.clone())
                    .unwrap_or(result.clone()).clone();
                if result != previous {
                    found_difference = true;
                    debug!("{command_str}: {result} (previous: {previous})");
                    let mut message = Message::empty();
                    message.text = result;
                    publisher.lock().await.send_event(MODULE_NAME, command_str.replace(" ", "_").as_str(), &message).await?;
                }
            }
        }
    }
    let delay = if found_error { 1000 } else { if found_difference { 500 } else { 10 } };
    tokio::time::sleep(Duration::from_millis(delay)).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let module = InterfaceModule::new(MODULE_NAME).await?;
    let subscriber = Arc::new(Mutex::new(module.connection.subscriber));
    let publisher1 = Arc::new(Mutex::new(module.connection.publisher));
    let publisher2 = publisher1.clone();

    let drivers = Arc::new(Mutex::new(Drivers::new("/dev/i2c-1")));
    let drivers2 = drivers.clone();
    subscriber.lock().await.listen(INPUT_TOPIC).await?;
    // TODO: load from config.toml file
    let watcher_commands = vec![
        "head touch".to_string(),
        //"motherboard kbd".to_string()
    ];

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
