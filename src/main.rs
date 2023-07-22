use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent};
use matrix_sdk::ruma::serde::duration;
use matrix_sdk::{config::SyncSettings, Client};
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::{thread, time};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

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

// fn llama(input : &String,path:String) -> Vec<u8>{
//     let azer = Command::new("cat -n")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())        .spawn()
//         .expect("ls command failed to start");
//     // let output = Command::new(format!("{}/main", path))
//     //     .arg("--model")
//     //     .arg(format!("{}/models/30B/ggml-model-f16-ggjt.bin", path))
//     //     .arg("--prompt")
//     //     .arg(input)
//     //     .output()
//     //     .expect("thats suck")
//         // .stdout;
//     return  output;
// }

// fn llama(input: String, enfant: Arc<Mutex<Child>>) -> Vec<u8> {
//     let mut process_guard = enfant.lock().unwrap();

    

    
//     stdin
//     .write_all(input.as_bytes())
//     .expect("Failed to write to stdin");
//     let mut buffer:[u8;10]= [0;10];
//     // stdout.read(&mut buffer).expect("error read bufferer");
//     return buffer.to_vec();
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut llama_process =
        Command::new("./prog")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("cat command failed to start");

    let stdin = Arc::new(Mutex::new(llama_process.stdin.take().expect("Failed to open stdin")));
    let stdout = Arc::new(Mutex::new(llama_process.stdout.take().expect("Failed to open stdout")));

    let poiu = read_conf();
    let client = login(poiu).await?;

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
                        let mut buf:[u8;100]= [0;100];

                        stdin.lock().unwrap().write(m.body.as_bytes()).unwrap();
                        println!("SEND:{}",m.body);

                        let sizer = stdout.lock().unwrap().read(&mut buf).unwrap();
                        let answer = String::from_utf8(buf.to_vec()).unwrap();
                        println!("READ: size: {sizer} buf:{:?}",answer);
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
