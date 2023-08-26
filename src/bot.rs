use std::time::Duration;

use matrix_sdk::{config::SyncSettings, Client, LoopCtrl};

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};

pub struct Bot {
    enable: LoopCtrl,
    //client
    worker_list: Vec<Worker>,
    login: Client,
}

impl Bot {
    pub async fn new() -> Self {
        let enable = LoopCtrl::Continue;
        //connect to the home server
        let matrixconf = Config::new("config_test.yaml".to_string()).matrix;
        let client = Self::login(matrixconf).await;

        //reating some worker
        // let worker_a = Worker::new(Profile::base).await;
        // let worker_b = Worker::new(Profile::base).await;

        //syncronize with home server
        let sync_settings = SyncSettings::new().timeout(Duration::from_secs(30));
        client.sync_with_callback(sync_settings, |response| async move {
                for (room_id, room) in response.rooms.join {
                    for event in room.timeline.events {
                        println!("Get:event!");
                    }
                }
                enable
            })
            .await.expect("Imposible de start la syncro server");

        Bot {
            enable,
            login: client,
            worker_list: vec![], //vec![worker_a, worker_b],
        }
    }

    pub async fn start(mut self) {
        self.enable = LoopCtrl::Break;
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
