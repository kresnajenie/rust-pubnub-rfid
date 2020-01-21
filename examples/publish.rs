#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

use futures_util::stream::StreamExt;
use pubnub::{json::object, Error, PubNub};

#[tokio::main]
async fn publish_pubnub(rfid) -> Result<(), Error> {
    let mut pubnub = PubNub::new("pub-c-2848a4b4-ed53-4b26-84ba-009c384d8b0c", "sub-c-f660257a-ecae-11e9-bdee-36080f78eb20");

    let message = object! {
        "username" => "JoeBob",
        "content" => rfid,
    };

    // let mut stream = pubnub.subscribe("my-channel").await;
    let timetoken = pubnub.publish("my-channel", message).await?;
    println!("timetoken = {:?}", timetoken);

    // let received = stream.next().await;
    // println!("received = {:?}", received);

    Ok(())
}
