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
use tokio::{select, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};
pub struct Bot {
    enable: CancellationToken,
    //client
    data: Arc<Mutex<RACE>>,
    worker_list: Vec<Worker>,
    login: Client,
    // context:EvHandlerContext<'a>,
}

#[derive(Clone, Copy)]
struct RACE {
    data: u8,
}

impl RACE {
    fn suce(&mut self) {
        self.data += 1;
        println!("{}", self.data);
    }
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
            data: Arc::new(Mutex::new(RACE { data: 1 })),
            login: client,
            worker_list: vec![], //vec![worker_a, worker_b],
        }
    }

    pub async fn start(mut self) {
        self.enable = CancellationToken::new(); //to be sure
                                                // self.login.add_event_handler_context(self.data);
        let data = self.data.clone();

        self.login.add_event_handler({
            let data = self.data.clone();
            move |ev: SyncRoomMessageEvent, room: Room| {
                let data = data.clone();
                async move {
                    data.clone().lock().unwrap().suce();
                    println!("EVENT!");
                }
            }
        });

        // //if i desire to cancel
        // let enable = self.enable.clone();
        // tokio::spawn(async move {
        //     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        //     enable.cancel();
        // });
        self.sync_start_stop().await;
    }

    async fn sync_start_stop(&mut self) -> tokio::task::JoinHandle<()> {
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
