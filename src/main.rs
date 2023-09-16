mod bot;
mod worker;
use std::thread;
use std::time::Duration;

use bot::Bot;
// use worker::Worker;
// use worker::profile::Profile;
pub(crate) mod config;

// //command
// // !help
// // !restart
// // !take
// // !give
// // !ftake //oom kill somone

#[tokio::main]
async fn main() {
    let instance = Bot::new().await;
    instance.start().await;

    loop {
        thread::sleep(Duration::from_secs(9999999));
    }
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
