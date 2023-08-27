mod worker;
mod bot;
use std::thread;
use std::time::Duration;

use bot::Bot;
use worker::Worker;
use worker::profile::Profile;
pub(crate) mod config;

// use std::{
//     fs::File,
//     io::{BufRead, BufReader, Read, Write},
//     process::{Child, ChildStdin, ChildStdout, Command, Stdio},
//     sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, collections::VecDeque,
// };

// use serde::Deserialize;


// use matrix_sdk::{
//     config::SyncSettings,
//     room::{Joined, Room},
//     ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent},
//     Client,
// };



// //command
// // !help
// // !restart
// // !take
// // !give
// // !ftake //oom kill somone

// async fn login(conf: MatrixConfig) -> anyhow::Result<Client> {
//     let client = Client::builder()
//         .homeserver_url(conf.homeserver_url)
//         .build()
//         .await?;

//     client
//         .login_username(&conf.username, &conf.password)
//         .initial_device_display_name("getting started bot")
//         .send()
//         .await?;

//     println!("logged in as {}", conf.username);
//     return Ok(client);
// }


// async fn handlers(
//     mut token: String,
//     client: Client,
//     stdout: ChildStdout,
//     stdin: Arc<Mutex<ChildStdin>>,
// ) -> String {
//     let (tx, mut rx) = mpsc::channel(30);
//     let restart_button = Arc::new(Mutex::new(false));
//     let restart_button_mv = restart_button.clone();
//     task::spawn(async move {
//         let bufreader = BufReader::new(stdout);
//         let lines = bufreader.lines();
//         let room: Joined = rx.recv().await.unwrap();
//         for line in lines {
//             let answer = line.unwrap();
//             println!("LLAMA-OUT:{}", answer);
//             let llama_answer = RoomMessageEventContent::text_plain(answer);
//             room.send(llama_answer, None).await.unwrap();
//         }
//     });

//     let handle = client.add_event_handler({
//         //TRIKS
//         let restart_button_mv = restart_button_mv.clone();
//         let stdin: Arc<Mutex<ChildStdin>> = stdin.clone();
//         let tx = tx.clone();
//         move |ev: SyncRoomMessageEvent, room: Room| {
//             //TRIKS
//             let tx = tx.clone();
//             let stdin = stdin.clone();
//             let restart_button_mv = restart_button_mv.clone();
//             async move {
//                 //filtre les message envoyer a soit meme
//                 if room.client().user_id().unwrap() == ev.sender() {
//                     return;
//                 }
//                 to_llama(ev, stdin, restart_button_mv);
//                 match room {
//                     Room::Joined(room) => {
//                         //data a send to user
//                         tx.send(room).await.unwrap();
//                     }
//                     _ => println!("event on unjoined room"),
//                 }
//             }
//         }
//     });

//     //handler to accept new people

//     while !*restart_button.lock().unwrap() {
//         token = client
//             .sync_once(SyncSettings::default().token(token))
//             .await
//             .unwrap()
//             .next_batch;
//     }
//     println!("!!!FuckMeDaddy!!! --- EXIT --- !!!FuckMeDady!!!");
//     client.remove_event_handler(handle);
//     token
// }

#[tokio::main]
async fn main() {
// -> anyhow::Result<()> {
    // // Read conf file
    // let conf = read_config_from_file().expect("failed to read conf file");

    // // generate client and login
    // let client = login(conf.matrix).await?;

    // //get first token
    // let mut token = client
    //     .sync_once(SyncSettings::default())
    //     .await
    //     .unwrap()
    //     .next_batch;


    // //.create
    // let mut my_master = Master{session:client, chats:vec![],ressources:vec![]};
    // my_master.new_runner(&conf.command_args, &conf.path);
    // //start new LLM instance
    // //.new
    // //.new

    

    // let mut azer = Vec::<String>::new();
    // let mut work = Worker::new(Profile::base).await;
    // azer.push(work.interaction("what color of a orange ?").await);
    // azer.push(work.interaction("what is the color of a apple ?").await);
    // work.quit().await;

    

    let instance = Bot::new().await;
    instance.start().await;
    // println!("Sync1");
    // instance.sync_start_stop().await;
loop {
    
}
    // Worker::new(Profile::from_config(Config::default()));
    // Worker::new(Profile::raw(vec!["ls","-l","fucked"].iter().map(ToString::to_string).collect()));
    // Worker::new(Default::default());
    // //.switch rooting

    // //main loop
    // loop {
    //     //lunch LLM
    //     let mut llm_proc = lunch_LLM(&conf.command_args, &conf.path);

    //     // Take Redirection
    //     let stdin = Arc::new(Mutex::new(
    //         llm_proc.stdin.take().expect("Failed to open stdin"),
    //     ));
    //     let stdout = llm_proc.stdout.take().expect("Failed to open stdout");

    //     //MAIN HANDLER
    //     token = handlers(token, client.clone(), stdout, stdin).await;

    //     //clean process
    //     llm_proc.kill().unwrap();
    //     llm_proc.wait().unwrap();
    // }
    
}
// pub async fn handle_event<'a>(mut args: &mut Split<'a, char>, room: Room, ev: &SyncRoomMessageEvent) {
//     let target = match args.next() {
//         None => {
//             room.send(RoomMessageEventContent::text_plain("no target"), None).await.expect("error sending message");
//             return;
//         }
//         Some(str) => str
//     };
//     let content = RoomMessageEventContent::text_plain(
//         format!("*{} pats {}*", ev.sender().to_string(), target)
//     );
//     room.send(content, None).await.expect("error sending message");
// }


// pub fn register(client: &Client){
//     client.add_event_handler(
//         |ev: SyncRoomMessageEvent, room: Room| {
//             async move {
//                 let original = ev.as_original().unwrap();
//                 let content = original.content.body();

//                 if content.starts_with('!') {
//                     let mut result = content.split(' ');
//                     match result.next() {
//                         Some(str) => {
//                             match str {
//                                 "!help" => {
//                                     help::handle_event(room).await;
//                                 }
//                                 "!pat" => {
//                                     affection::handle_event("pats", &mut result, room, &ev).await;
//                                 }
//                                 "!boops" => {
//                                     affection::handle_event("boops", &mut result, room, &ev).await;
//                                 }
//                                 "!boops" => {
//                                     affection::handle_event("hugs", &mut result, room, &ev).await;
//                                 }
//                                 _ => {
//                                     let content = RoomMessageEventContent::text_plain("Invalid command");
//                                     room.send(content, None).await.expect("error sending message");
//                                     return;
//                                 }
//                             }
//                         }
//                         None => return
//                     }
//                 } else {
//                     return;
//                 }

//             }
//         },
//     );
// }