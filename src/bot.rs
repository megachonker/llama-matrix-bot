use std::{
    sync::{Arc, Mutex}, time::Duration,
};

use matrix_sdk::{
    config::SyncSettings,
    room::{Joined, Room},
    ruma::{events::room::message::SyncRoomMessageEvent, RoomId,},
    BaseRoom, Client,
};
use tokio::{select, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};

pub struct Bot {
    enable: CancellationToken,
    //client
    rooms: Vec<room>,
    worker_list: Vec<Worker>,
    login: Client,
    // context:EvHandlerContext<'a>,
}

struct room {
    id: Box<RoomId>,
    message:String,
    answer:String,
}
#[derive(Debug)]
struct ctx{
    ev: SyncRoomMessageEvent,
    room: Room
}



impl Bot {
    pub async fn new() -> Self {
        //connect to the home server
        let matrixconf = Config::new("config_test.yaml".to_string()).matrix;
        let client = Self::login(matrixconf).await;

        //reating some worker
        // let worker_a = Worker::new(Profile::base).await;
        // let worker_b = Worker::new(Profile::base).await;

        Bot {
            enable: CancellationToken::new(),
            rooms: vec![],
            login: client,
            worker_list: vec![], //vec![worker_a, worker_b],
        }
    }

    pub async fn start(mut self) {
        //call disable to cancel sync
        self.enable = CancellationToken::new(); //to be sure token enable

        //i choose to use channel than context because after data was piped i can do 
        //EVERYTHING, in the context the data inside the struct are STUCK like a CUCK
        let (tx, mut rx) = mpsc::channel::<ctx>(10);

        //on message receive
        self.login.add_event_handler({
            move |ev: SyncRoomMessageEvent, room: Room| {
                let tx = tx.clone();
                async move {
                    //FUCK YOU add_handler_context !!! 
                    let playload = ctx{ev:ev.clone(),room:room.clone()};
                    tx.send(playload).await.expect("Sending error");
                }
            }
        });

        //handle new message in rooter async
        //FUCK YOU add_handler_context !!! 
        tokio::spawn(async move{
            loop {
                let value = rx.recv().await.unwrap();
                Bot::router(value);
            }
        });

        //start sync can be cancel by sync stop or can by async without await
        Bot::sync_start(&self.login,&self.enable).await;
    }


    fn router(bundle:ctx){
        //unwrap context
        let ev = bundle.ev;
        let room = bundle.room;


        println!("new event:");
        println!("{:?}",bundle);
    }

    async fn sync_stop(&self,delay:Duration) -> tokio::task::JoinHandle<()>{
        let enable = self.enable.clone();
        return  tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            enable.cancel();
        });
    }

    async fn sync_start(login:&Client,enable:&CancellationToken) -> tokio::task::JoinHandle<()> {
        let login = login.clone();
        let token = enable.clone();

        return tokio::spawn(async move {
            select! {
                _ = token.cancelled() => {println!("Sync Off")}
                _ = login.sync(SyncSettings::default()) => {}
            }
        });
    }

    async fn login(conf: MatrixConfig) -> Client {
        let client = Client::builder()
            .homeserver_url(conf.homeserver_url)
            .build()
            .await
            .expect("failed to build connextion");

        client
            .login_username(&conf.username, &conf.password)
            .initial_device_display_name("getting started bot")
            .send()
            .await
            .expect("failed to login");

        println!("logged in as {}", conf.username);
        client
    }
}
