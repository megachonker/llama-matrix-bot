use futures::future::join_all;
use matrix_sdk::{
    deserialized_responses::SyncResponse,
    ruma::events::room::message::{MessageType, RoomMessageEventContent},
    LoopCtrl,
};

use std::{sync::Arc, time::Duration};

use matrix_sdk::{
    config::SyncSettings,
    room::Room,
    ruma::{events::room::message::SyncRoomMessageEvent, RoomId},
    Client,
};
use tokio::{
    join, select,
    sync::mpsc::{self, Receiver},
    sync::{mpsc::Sender, Mutex},
};
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, MatrixConfig},
    worker::{profile::Profile, Worker},
};

pub struct Bot {
    enable: CancellationToken,
    //client
    rooms: Arc<Mutex<Vec<Arc<Mutex<BotRoom>>>>>,
    sheduler:Sheduler,
    login: Client,
}

struct BotRoom {
    //imply time out for worker to reject
    id: Box<RoomId>,
    message: Arc<Mutex<Vec<String>>>,
    worker: Arc<Mutex<Worker>>,
}

//permet de déplacer les event
#[derive(Debug)]
struct CtxEventRoom {
    ev: SyncRoomMessageEvent,
    room: Room,
}

enum ShedulingEvent {
    Killed,
}

struct Sheduler {
    pub tx_event: Sender<ShedulingEvent>,
    pub rx_worker: Receiver<Worker>,
}

impl Sheduler {
    pub async fn new(target: u8) -> Self {
        //create event channel
        let (tx_event, rx_event) = mpsc::channel::<ShedulingEvent>(10);

        //create worker channel
        let (tx_worker, rx_worker) = mpsc::channel::<Worker>(10);

        //lunch worker target
        let mut handles = Vec::new();

        for a in 0..target {
            let tx_worker = tx_worker.clone();
            handles.push(tokio::spawn(
                async move { 
                    println!("Start worker:{}",a);
                    tx_worker.send(Worker::new(Profile::Base).await).await.unwrap();
                    println!("--{:?} is started--",a);
                },
            ));
        }
        let _ = join_all(handles);
        println!("all worker started");

        //start sheduler
        Sheduler::event_router(tx_worker,rx_event);

        //pass back struct
        Self { rx_worker,tx_event }
    }

    fn event_router(tx_worker:Sender<Worker>, mut rx_event:Receiver<ShedulingEvent>){
        tokio::spawn(async move {
            loop {
                let event = rx_event.recv().await.unwrap();
                match event {
                    //when one VM is quit
                    ShedulingEvent::Killed => {
                        //can put somme rule here

                        //spwawn new one to remplace dead
                        println!("[Sheduler]: Spawn new worker:");
                        tx_worker.send(Worker::new(Profile::Base).await).await.unwrap();
                    }
                }
            }
        });
    }
}

impl Bot {
    pub async fn new() -> Self {
        //connect to the home server
        let matrixconf = Config::new("config_test.yaml".to_string()).matrix;
        let client = Self::login(matrixconf).await;

        //reating some worker
        let sheduler = Sheduler::new(4).await;

        //fill struct
        Bot {
            enable: CancellationToken::new(),
            rooms: Arc::new(Mutex::new(vec![])),
            login: client,
            sheduler, //,
        }
    }

    async fn handle_invitations(&self) {
        for room in self.login.invited_rooms().iter() {
            room.accept_invitation().await.expect("impossible to join");
        }
    }

    //consume all
    pub async fn start(mut self) {
        // shedul_event_router

        //call disable to cancel sync
        self.enable = CancellationToken::new(); //to be sure token enable

        //accept all invite
        self.login.sync_once(Default::default()).await.unwrap();
        self.handle_invitations().await;

        //i choose to use channel than context because after data was piped i can do
        //EVERYTHING, in the context the data inside the struct are STUCK like a CUCK
        let (tx_ctx, mut rx_ctx) = mpsc::channel::<CtxEventRoom>(10);

        //on message receive
        self.login.add_event_handler({
            move |ev: SyncRoomMessageEvent, room: Room| {
                let tx = tx_ctx.clone();
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
            loop {
                let value = rx_ctx.recv().await.expect("nobody sending ?");
                Bot::route_event(value, &self.rooms, &mut self.sheduler).await;
            }
        });

        //start sync can be cancel by sync stop or can by async without await
        Bot::sync_start(&self.login, &self.enable).await;
    }

    //tout ce qui est recus par server
    //crée les room use by the bot
    async fn route_sync(mut rx: Receiver<SyncResponse>) {
        // mesrooms.push(room {..Default::default()});
        loop {
            let data = rx.recv().await.expect("errr recv route data");
            drop(data); //<= to use after
            // data; //<= to use after
                  // println!("-------------------------------------");
                  // println!("{:?}", data.presence.events);
        }
    }

    //tout ce qui est émit ou recus par une room
    async fn route_event(
        bundle: CtxEventRoom,
        bot_rooms: &Arc<Mutex<Vec<Arc<Mutex<BotRoom>>>>>,
        sheduler: &mut Sheduler,
    ) {
        if bundle.room.client().user_id().unwrap() == bundle.ev.sender() {
            return;
        }

        //unwrap context

        let roomid = bundle.room.room_id();
        let bot_room;
        //create new room if needed
        {
            let mut roomlist_locked = bot_rooms.lock().await;
            let mut current_room_arc = None;

            //find the event room to boot room
            for room in roomlist_locked.iter() {
                if room.lock().await.id == roomid {
                    current_room_arc = Some(Arc::clone(room)); //Stay lock from her ?
                    break;
                }
            }

            //if room trouver
            if let Some(room) = current_room_arc {
                bot_room = room;
            } 
            //sinon on en crée une nouvel est on lui donne un worker par def
            else {
                let selected_worker = sheduler.rx_worker.recv().await.unwrap();

                bot_room = Arc::new(Mutex::new(BotRoom {
                    id: roomid.into(),
                    message: Arc::new(Mutex::new(vec![])),
                    worker: Arc::new(Mutex::new(selected_worker)),
                }));

                roomlist_locked.push(bot_room.clone());
                println!("new room handled");
            }
        }

        match &bundle.ev.as_original().unwrap().content.msgtype {
            // matrix_sdk::ruma::events::room::message::MessageType::Notice()
            MessageType::Text(message) => {
                let msg = message.clone();
                let message = match msg.formatted {
                    Some(data) => data.body,
                    None => msg.body,
                };
                println!("{}", message);
                if message == "!reset" {
                    Bot::rcv_reset(bot_room, sheduler).await;
                } else {
                    Bot::rcv_message(bundle, bot_room, message);
                }
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

    //BOOOT cmd

    //nomal
    fn rcv_message(bundle: CtxEventRoom, bot_room: Arc<Mutex<BotRoom>>, message: String) {
        tokio::spawn(async move {
            let bot_room = bot_room.clone();
            let current_room = bot_room.lock().await;

            let mut hist = current_room.message.lock().await.clone();
            hist.push(message);
            println!("HISTORIQUE:{:?}", hist);

            let worker = current_room.worker.clone();
            drop(current_room);

            let mut worker = worker.lock().await;
            let answer = worker.interaction(hist.last().unwrap()).await;
            match bundle.room {
                Room::Invited(_) => {}
                Room::Joined(room) => {
                    let llama_answer = RoomMessageEventContent::text_plain(answer);
                    room.send(llama_answer, None).await.unwrap();
                }
                Room::Left(_) => {}
            }
        });
    }

    async fn rcv_reset(bot_room: Arc<Mutex<BotRoom>>, sheduler: &mut Sheduler) {
        let mut room_lock = bot_room.lock().await;
        room_lock.worker.lock().await.quit().await;
        
        println!("SEND KILL");
        sheduler.tx_event.send(ShedulingEvent::Killed).await.unwrap();

        println!("PICK worker");
        let selected_worker = sheduler.rx_worker.recv().await.unwrap();
        room_lock.worker = Arc::new(Mutex::new(selected_worker));
    }

    // async fn rcv_reset(bot_room:Arc<Mutex<BotRoom>>,workers:&Arc<Mutex<Vec<Worker>>>){
    //     let selected_worker = workers.lock().await.remove(0);
    //     let mut room_lock = bot_room.lock().await;
    //     room_lock.worker.lock().await.quit().await;
    //     room_lock.worker = Arc::new(Mutex::new(selected_worker));
    // }

    //get deleted when sync stoped
    async fn sync(login: Client) {
        let (tx, rx) = mpsc::channel::<SyncResponse>(10);
        let sync_channel = &tx;
        let f1 = Bot::route_sync(rx);
        let f2 = login //fuckyou sinc setting
            .sync_with_callback(SyncSettings::default(), |response| async move {
                let tx = sync_channel;
                tx.send(response).await.expect("err send");
                LoopCtrl::Continue
            });
        let _ = join!(f1, f2);
    }

    async fn sync_stop(&self, delay: Duration) -> tokio::task::JoinHandle<()> {
        let enable: CancellationToken = self.enable.clone();
        return tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            enable.cancel();
        });
    }

    async fn sync_start(login: &Client, enable: &CancellationToken) -> tokio::task::JoinHandle<()> {
        let login = login.clone();
        let token = enable.clone();

        return tokio::spawn(async move {
            select! {
                _ = token.cancelled() => {println!("Sync Off")}
                _ = Bot::sync(login) => {}
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
