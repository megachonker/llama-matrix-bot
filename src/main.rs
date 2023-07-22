use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent};
use matrix_sdk::{config::SyncSettings, Client};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use std::process::Command;

#[derive(Debug, Deserialize)]
struct ConfigData {
    username: String,
    password: String,
    homeserver_url: String,
    path: String
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

fn llama(input : &String,path:String) -> Vec<u8>{
    let output = Command::new(format!("{}/main", path))
        .arg("--model")
        .arg(format!("{}/models/30B/ggml-model-f16-ggjt.bin", path))
        .arg("--prompt")
        .arg(input)
        .output()
        .expect("thats suck")
        .stdout;
    return  output;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let poiu = read_conf();
    let client = login(poiu).await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent, room: Room| async move {
        let path: String = read_conf().path; //ugly

        if room.client().user_id().unwrap() == ev.sender(){
            return ;
        }
        match &ev.as_original().unwrap().content.msgtype {
            matrix_sdk::ruma::events::room::message::MessageType::Text(m) => {
                println!("{}", m.body); //read message
                let answer = String::from_utf8(llama(&m.body,path)).unwrap();
                let fuck = RoomMessageEventContent::text_plain(answer);  // prépare le message
                match room {
                    Room::Joined(raclure) => {
                        raclure.send(fuck, None).await.unwrap();//answer
                    }
                    _ => println!("LA FE%% CE A F"),
                }
            }
            _ => println!("fuckyou"),
        }
    });
    client.sync(SyncSettings::default()).await?; // this essentially loops until we kill the bot
    Ok(())
}
