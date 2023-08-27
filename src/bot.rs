use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::{Joined, Room},
    ruma::{events::room::message::SyncRoomMessageEvent, RoomId},
    BaseRoom, Client,
};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};

pub struct Bot {
    enable: CancellationToken,
    //client
    worker_list: Vec<Worker>,
    login: Client,
    // context:EvHandlerContext<'a>,
}
// #[derive(Clone)]
// struct EvHandlerContext<'a>{
//     room_list:Vec<&'a RoomId>
// }

// async fn test(
//     ev: SyncRoomMessageEvent,
//     _room: Room,
//     _client: Client,
//     context: Ctx<EvHandlerContext<'_>>,
// ){
//     ;
// }

impl Bot {
    pub async fn new() -> Self {
        //connect to the home server
        let matrixconf = Config::new("config_test.yaml".to_string()).matrix;
        let client = Self::login(matrixconf).await;

        //reating some worker
        // let worker_a = Worker::new(Profile::base).await;
        // let worker_b = Worker::new(Profile::base).await;

        Bot {
            // context: EvHandlerContext{room_list:vec![]} ,
            enable: CancellationToken::new(),
            login: client,
            worker_list: vec![], //vec![worker_a, worker_b],
        }
    }

    pub fn start(&mut self) {
        self.enable = CancellationToken::new(); //to be sure

        let list = Arc::new(Mutex::new(Vec::<String>::new()));

        //ajoute les salon
        self.login.add_event_handler({
            let list = list.clone();
            move |ev: SyncRoomMessageEvent, room: Room, client: Client| {
                let list = list.clone();
                async move {
                    let room_id = room.room_id().to_string();
                    println!("{}",room_id);
                    list.lock().unwrap().to_vec().push(room_id);
                }
            }
        });

        self.login.add_event_handler({
            let list = list.clone();
            move |ev: SyncRoomMessageEvent| {
                let list = list.clone();
                async move {
                    let data = list.clone();
                    let data = data.lock().unwrap();
                    println!("{}", data.join(", "));
                }
            }
        });

        // //if i desire to cancel
        // let enable = self.enable.clone();
        // tokio::spawn(async move {
        //     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        //     enable.cancel();
        // });
    }

    pub async fn sync_start_stop(&self) -> tokio::task::JoinHandle<()> {
        let login = self.login.clone();
        let token = self.enable.clone();

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
