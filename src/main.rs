use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::{SyncRoomMessageEvent,RoomMessageEventContent};
use matrix_sdk::{config::SyncSettings, Client};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ConfigData {
    username: String,
    password: String,
    homeserver_url: String,
}

fn read_conf() -> ConfigData {
    let mut config_file = File::open("config.yaml").expect("Failed to open configuration file.");
    let mut config_content = String::new();

    config_file
        .read_to_string(&mut config_content)
        .expect("Failed to read configuration file.");

    return serde_yaml::from_str(&config_content).expect("Failed to parse YAML.");
}

async fn login(conf: ConfigData) -> anyhow::Result<Client> {
    let client = Client::builder()
        .homeserver_url(conf.homeserver_url)
        .build()
        .await?;

    client
        .login_username(&conf.username, &conf.password)
        .initial_device_display_name("getting started bot")
        .send()
        .await?;

    println!("logged in as {}", conf.username);
    return Ok(client);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = login(read_conf()).await?;
    client.add_event_handler(|ev: SyncRoomMessageEvent, room: Room| async move {
        match &ev.as_original().unwrap().content.msgtype {
            matrix_sdk::ruma::events::room::message::MessageType::Text(m) => {
                println!("{}", m.body);
                let fuck = RoomMessageEventContent::text_plain("MEur MEru MEur");
                match room {
                    Room::Joined(raclure) => {raclure.send(fuck, None).await.unwrap();},
                    _ => println!("LA FE%% CE A F"),
                }
            }
            _ => println!("fuckyou"),
        }
        // room
        //ned answer fuck yourtself
    });
    client.sync(SyncSettings::default()).await?; // this essentially loops until we kill the bot
    Ok(())
}
