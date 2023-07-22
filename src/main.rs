use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent};
use matrix_sdk::{config::SyncSettings, Client};
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(Debug, Deserialize)]
struct ConfigData {
    username: String,
    password: String,
    homeserver_url: String,
    path: String,
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
    let configuration_var = read_conf();

    let mut llama_process = Command::new("/bin/bash")
        .arg(format!("{}/examples/chat-30B.sh", configuration_var.path))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .env("USER_NAME", "frank")
        .env("AI_NAME", "javis")
        .env("N_THREAD", "3")
        .env("N_PREDICTS", "2048")
        .spawn()
        .expect("Failed to lunch LLama");

    let stdin = Arc::new(Mutex::new(
        llama_process.stdin.take().expect("Failed to open stdin"),
    ));
    let stdout = Arc::new(Mutex::new(
        llama_process.stdout.take().expect("Failed to open stdout"),
    ));
    let client = login(configuration_var).await?;

    client.add_event_handler({
        let stdin = stdin.clone();
        let stdout = stdout.clone();
        move |ev: SyncRoomMessageEvent, room: Room| {
            let stdin = stdin.clone();
            let stdout = stdout.clone();

            async move {
                if room.client().user_id().unwrap() == ev.sender() {
                    return;
                }
                match &ev.as_original().unwrap().content.msgtype {
                    matrix_sdk::ruma::events::room::message::MessageType::Text(m) => {
                        let mut buf = String::new();

                        stdin.lock().unwrap().write(m.body.as_bytes()).unwrap();
                        println!("SEND:{}", m.body);

                        let sizer = stdout.lock().unwrap().read_to_string(&mut buf).unwrap();
                        let answer = buf
                            .as_str()
                            .lines()
                            .last()
                            .expect("last line imposible to get");
                        println!("READ: size: {sizer} buf:{}", answer);
                        let fuck = RoomMessageEventContent::text_plain(answer); // prépare le message
                        match room {
                            Room::Joined(raclure) => {
                                raclure.send(fuck, None).await.unwrap(); //answer
                            }
                            _ => println!("LA FE%% CE A F"),
                        }
                    }
                    _ => println!("fuckyou"),
                }
            }
        }
    });
    client.sync(SyncSettings::default()).await?; // this essentially loops until we kill the bot
    Ok(())
}
