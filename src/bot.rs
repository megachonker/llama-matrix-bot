use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use matrix_sdk::{config::SyncSettings, Client, LoopCtrl};
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
            login: client,
            worker_list: vec![], //vec![worker_a, worker_b],
        }
    }

    pub async fn start_stop(&self) {
        let login = self.login.clone(); 
        let token = self.enable.clone(); 
        let tt = self.enable.clone(); 

        let join_handle = tokio::spawn(async move {

            select! {
                _ = token.cancelled() => {
                    println!("CANCEL");
                    5
                }
                _ = login.sync(SyncSettings::default()) => {
                    println!("Start");
                    99
                }
            }
        });
        
        
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            tt.clone().cancel();
        });
    
        assert_eq!(5, join_handle.await.unwrap());

        
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
