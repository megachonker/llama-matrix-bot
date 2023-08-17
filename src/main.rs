mod worker;
use worker::Worker;
use worker::profile;

// use std::{
//     fs::File,
//     io::{BufRead, BufReader, Read, Write},
//     process::{Child, ChildStdin, ChildStdout, Command, Stdio},
//     sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, collections::VecDeque,
// };

// use serde::Deserialize;

// use tokio::{
//     sync::mpsc,
//     task,
// };

// use matrix_sdk::{
//     config::SyncSettings,
//     room::{Joined, Room},
//     ruma::events::room::message::{RoomMessageEventContent, SyncRoomMessageEvent},
//     Client,
// };


// #[derive(Debug, Deserialize, Clone)]
// struct Config {
//     matrix: MatrixConfig,
//     path: String,
//     command_args: CommandArgs,
// }

// #[derive(Debug, Deserialize, Clone)]
// struct MatrixConfig {
//     username: String,
//     password: String,
//     homeserver_url: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// struct CommandArgs {
//     ctx_size: u32,
//     temp: f64,
//     top_k: u32,
//     top_p: f64,
//     repeat_last_n: u32,
//     batch_size: u32,
//     repeat_penalty: f64,
//     model: String,
//     threads: u32,
//     n_predict: u32,
//     interactive: bool,
//     reverse_prompt: String,
//     prompt: String,
// }

// //command
// // !help
// // !restart
// // !take
// // !give
// // !ftake //oom kill somone

// enum ChildAction {
//     Restart,
//     Ftake,
//     Take,
//     Give,
// }

// enum MasterAction {
//     Ack(Child),
//     Release,
// }

// //lunch
// struct Master {
//     session:Client,
//     chats: Vec<ShitChat>,
//     ressources: Vec<Child>,
// }

// impl Master {
//     fn new_runner(&mut self,command_args:&CommandArgs, path:&str){
//         self.ressources.push(lunch_LLM(command_args, path))
//     }
//     fn new_sc() {
//         //create pipe
//         //create shitchat
//         todo!()
//     }
// }

// struct ChildSC {
//     room: Joined,
//     to_sc: Sender<MasterAction>,
//     from_sc: Receiver<ChildAction>,
//     shitchat: Child,
// }

// //user are created when receive message
// struct ShitChat {
//     to_master: Sender<ChildAction>,
//     from_master: Receiver<MasterAction>,
//     room: Joined, //recognize room
//     chat_input: VecDeque<String>,
//     llama_instance: Child, //borrow a instance
// }

// //input room
// fn rooting_room() {
//     //search for a matching room | hashmap ?
//     //else
//     //create new child_sc
// }

// fn read_config_from_file() -> Result<Config, Box<dyn std::error::Error>> {
//     let mut file = File::open("config.yaml")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let config: Config = serde_yaml::from_str(&contents)?;

//     Ok(config)
// }

// fn lunch_LLM(command_args: &CommandArgs, program_executable: &str) -> Child {
//     let mut cmd = Command::new(program_executable);

//     cmd.arg("--ctx_size").arg(command_args.ctx_size.to_string());
//     cmd.arg("--temp").arg(command_args.temp.to_string());
//     cmd.arg("--top_k").arg(command_args.top_k.to_string());
//     cmd.arg("--top_p").arg(command_args.top_p.to_string());
//     cmd.arg("--repeat_last_n")
//         .arg(command_args.repeat_last_n.to_string());
//     cmd.arg("--batch_size")
//         .arg(command_args.batch_size.to_string());
//     cmd.arg("--repeat_penalty")
//         .arg(command_args.repeat_penalty.to_string());
//     cmd.arg("--model").arg(&command_args.model);
//     cmd.arg("--threads").arg(command_args.threads.to_string());
//     cmd.arg("--n_predict")
//         .arg(command_args.n_predict.to_string());
//     if command_args.interactive {
//         cmd.arg("--interactive");
//     }
//     cmd.arg("--reverse-prompt")
//         .arg(&command_args.reverse_prompt);
//     cmd.arg("--prompt").arg(&command_args.prompt);
//     cmd.stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("failed to lunch LLaMa")
// }

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

// fn to_llama(
//     ev: SyncRoomMessageEvent,
//     stdin: Arc<Mutex<std::process::ChildStdin>>,
//     restart_button_mv: Arc<Mutex<bool>>,
// ) {
//     match &ev.as_original().unwrap().content.msgtype {
//         matrix_sdk::ruma::events::room::message::MessageType::Text(m) => {
//             println!("SEND:{}", m.body);
//             if m.body.contains("!!!FuckMeDaddy!!!") {
//                 println!("EMERGENCY CUM CUMMED");
//                 *restart_button_mv.lock().unwrap() = true;
//             }
//             let azerazerazer = format!("{}\n", m.body);
//             let question = azerazerazer.as_bytes();
//             stdin.lock().unwrap().write_all(question).unwrap();
//         }
//         _ => println!("fuckyou"),
//     }
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

// #[tokio::main]
// async 
fn main() {
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


    let worker = Worker::new(profile::Profile::base);

        //  ::raw(String::from("azer")));
    let worker = Worker::new(Default::default());
    let worker = Worker::new(Default::default());

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
