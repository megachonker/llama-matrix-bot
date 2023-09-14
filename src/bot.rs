use matrix_sdk::{
    deserialized_responses::SyncResponse, ruma::events::room::message::MessageType, LoopCtrl,
};

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{events::room::message::SyncRoomMessageEvent, RoomId},
    Client,
};
use tokio::{
    join, select,
    sync::mpsc::{self, Receiver},
};
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};

pub struct Bot {
    enable: CancellationToken,
    //client
    rooms: Vec<Arc<Mutex<BotRoom>>>,
    worker_list: Arc<Mutex<Vec<Worker>>>,
    login: Client,
}

struct BotRoom {
    id: Box<RoomId>,
    message: Arc<Mutex<Vec<String>>>,
    worker: Worker,
}

//permet de déplacer les event
#[derive(Debug)]
struct CtxEventRoom {
    ev: SyncRoomMessageEvent,
    room: Room,
}

impl Bot {
    pub async fn new() -> Self {
        //connect to the home server
        let matrixconf = Config::new("config_test.yaml".to_string()).matrix;
        let client = Self::login(matrixconf).await;

        //reating some worker
        let worker_a = Worker::new(Profile::Base).await;
        let worker_b = Worker::new(Profile::Base).await;

        Bot {
            enable: CancellationToken::new(),
            rooms: vec![],
            login: client,
            worker_list: Arc::new(Mutex::new(vec![worker_a, worker_b])), //,
        }
    }

    async fn handle_invitations(&self) {
        for room in self.login.invited_rooms().iter() {
            room.accept_invitation().await.expect("impossible to join");
        }
    }

    //consume all
    pub async fn start(mut self) {
        //call disable to cancel sync
        self.enable = CancellationToken::new(); //to be sure token enable

        //accept all invite
        self.login.sync_once(Default::default()).await.unwrap();
        self.handle_invitations().await;

        //i choose to use channel than context because after data was piped i can do
        //EVERYTHING, in the context the data inside the struct are STUCK like a CUCK
        let (tx, mut rx) = mpsc::channel::<CtxEventRoom>(10);

        //on message receive
        self.login.add_event_handler({
            move |ev: SyncRoomMessageEvent, room: Room| {
                let tx = tx.clone();
                async move {
                    //FUCK YOU add_handler_context !!!
                    let playload = CtxEventRoom {
                        ev: ev.clone(),
                        room: room.clone(),
                    };
                    tx.send(playload).await.expect("Sending error");
                }
            }
        });

        //handle new message in rooter async
        //FUCK YOU add_handler_context !!!
        tokio::spawn(async move {
            let vcs = self.worker_list;
            loop {
                let value = rx.recv().await.expect("nobody sending ?");
                let cloned_data = *vcs.clone().lock().unwrap();

                Bot::route_event(value, &self.rooms, cloned_data).await;
            }
        });

        //start sync can be cancel by sync stop or can by async without await
        Bot::sync_start(&self.login, &self.enable, self.rooms).await;
    }

    //tout ce qui est recus par server
    //crée les room use by the bot
    async fn route_sync(mut rx: Receiver<SyncResponse>) {
        // mesrooms.push(room {..Default::default()});
        loop {
            let data = rx.recv().await.expect("errr recv route data");
            data; //<= to use after
            // println!("-------------------------------------");
            // println!("{:?}", data.presence.events);
        }
    }

    //tout ce qui est émit ou recus par une room
    async fn route_event(
        bundle: CtxEventRoom,
        rooms: &Vec<Arc<std::sync::Mutex<BotRoom>>>,
        workerlist: Vec<Worker>,
    ) {
        //unwrap context
        let ev = bundle.ev;
        let room = bundle.room;

        let roomid = room.room_id();
        let selected_room: &mut BotRoom;

        //create new room if needed
        {
            let mut roomlist_LOCKED = rooms.lock().unwrap();
            if !roomlist_LOCKED.iter().any(|obj| obj.id == roomid) {
                let str = Box::new("azer".to_string());
                // let var = Vec::new(str);

                let mut locked = workerlist;
                // .lock().unwrap();
                let azer = locked.remove(0);
                roomlist_LOCKED.push(BotRoom {
                    id: roomid.into(),
                    message: Arc::new(Mutex::new(vec![])),
                    worker: azer,
                });
                // ,answer:vec![].into(),message:vec![].into(),worker:Worker::new(Profile::base).await.into()});
                println!("new room handled")
                //need to attash worker
            }
            //assign room
            selected_room = roomlist_LOCKED
                .iter_mut()
                .find(|obj| obj.id == roomid)
                .unwrap();
        }

        match &ev.as_original().unwrap().content.msgtype {
            // matrix_sdk::ruma::events::room::message::MessageType::Notice()
            MessageType::Text(message) => {
                let msg = message.clone();
                let message = match msg.formatted {
                    Some(data) => data.body,
                    None => msg.body,
                };
                println!("{}", message);
                if message == "cum" {
                    let hist = selected_room.message.lock().unwrap();
                    println!("HISTORIQUE:{:?}", hist);
                    selected_room.worker.interaction(hist.last().unwrap()).await;
                } else {
                    selected_room.message.lock().unwrap().push(message.into());
                }
                //find Client with id
                //append message
            }
            MessageType::Emote(emo) => {
                println!("/me {}?", emo.body)
            }
            _ => {
                eprintln!("fuckyou router dumb")
            }
        }
    }

    //get deleted when sync stoped
    async fn sync(login: Client, rooms: Arc<std::sync::Mutex<Vec<BotRoom>>>) {
        let (tx, rx) = mpsc::channel::<SyncResponse>(10);
        let sync_channel = &tx;
        let f1 = Bot::route_sync(rx, rooms);
        let f2 = login //fuckyou sinc setting
            .sync_with_callback(SyncSettings::default(), |response| async move {
                let tx = sync_channel;
                tx.send(response).await.expect("err send");
                LoopCtrl::Continue
            });
        join!(f1, f2);
    }

    async fn sync_stop(&self, delay: Duration) -> tokio::task::JoinHandle<()> {
        let enable: CancellationToken = self.enable.clone();
        return tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            enable.cancel();
        });
    }

    async fn sync_start(
        login: &Client,
        enable: &CancellationToken,
        rooms: Arc<std::sync::Mutex<Vec<BotRoom>>>,
    ) -> tokio::task::JoinHandle<()> {
        let login = login.clone();
        let rooms = rooms.clone();
        let token = enable.clone();

        return tokio::spawn(async move {
            select! {
                _ = token.cancelled() => {println!("Sync Off")}
                _ = Bot::sync(login,rooms) => {}
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
