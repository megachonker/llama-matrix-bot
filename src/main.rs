use matrix_sdk::room::{Joined, Room};
use matrix_sdk::ruma::events::call::answer;
use matrix_sdk::ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent};
use matrix_sdk::{config::SyncSettings, Client};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Stdout, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;
use tokio::task;
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

fn to_llama(ev: SyncRoomMessageEvent, stdin: Arc<Mutex<std::process::ChildStdin>>) {
    match &ev.as_original().unwrap().content.msgtype {
        matrix_sdk::ruma::events::room::message::MessageType::Text(m) => {
            println!("SEND:{}", &m.body);
            let azerazerazer = format!("{}\n",m.body);
            let question = azerazerazer.as_bytes();
            stdin.lock().unwrap().write_all(question).unwrap();
        }
        _ => println!("fuckyou"),
    }
}

async fn handlers(client: Client, stdout: ChildStdout, stdin: Arc<Mutex<ChildStdin>>) {
    let (tx,mut rx) = mpsc::channel(10);
    task::spawn(async move {
        let bufreader = BufReader::new(stdout);
        let lines = bufreader.lines();
        let room: Joined = rx.recv().await.unwrap();
        for line in lines  {
            let answer  = line.unwrap();
            println!("LLAMA-OUT:{}",answer);
            let llama_answer = RoomMessageEventContent::text_plain(answer);
            room.send(llama_answer, None).await.unwrap();
        }
    });

    client.add_event_handler({
        //TRIKS
        let stdin = stdin.clone();
        let tx = tx.clone();
        move |ev: SyncRoomMessageEvent, room: Room| {
            //TRIKS
            let tx = tx.clone();
            let stdin = stdin.clone();
            async move {
                //filtre les message envoyer a soit meme
                if room.client().user_id().unwrap() == ev.sender() {
                    return;
                }
                to_llama(ev, stdin);
                match room {
                    Room::Joined(room) => {
                        tx.send(room).await.unwrap();
                    }
                    _ => println!("event on unjoined room"),
                }
            }
        }
    });

    client.sync(SyncSettings::default()).await.unwrap(); // this essentially loops until we kill the bot
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration_var = read_conf();

    let mut llama_process = Command::new("/bin/bash")
    // let mut llama_process = Command::new("./prog")
        .arg(format!("{}/examples/chat-30B.sh", configuration_var.path))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .env("USER_NAME", "frank")
        .env("AI_NAME", "javis")
        .env("N_THREAD", "16")
        .env("N_PREDICTS", "2048")
        .spawn()
        .expect("Failed to lunch LLama");

    // Take Redirection
    let stdin = Arc::new(Mutex::new(
        llama_process.stdin.take().expect("Failed to open stdin"),
    ));
    let stdout = llama_process.stdout.take().expect("Failed to open stdout");

    let client = login(configuration_var).await?;

    handlers(client, stdout, stdin).await;

    Ok(())
}
