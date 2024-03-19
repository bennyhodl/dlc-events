use std::str::FromStr;

use clap::Parser;
use nostr_sdk::{PublicKey, Client, EventBuilder, Keys, Kind, Tag};

const RELAY_URL: &str = "ws://localhost:8081";

#[derive(Parser)]
pub struct Args {
    // Event kind to broadcast
    kind: u64,
    // The text content to broadcast.
    content: String,
    // Pubkey to send to.
    pubkey: Option<String>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Parser::parse();

    let keys = Keys::generate();

    // let pubkey = PublicKey::parse(args.pubkey).unwrap();
    let pubkey = if let Some(pubkey) = args.pubkey {
        PublicKey::from_str(&pubkey).unwrap()
    } else {
        keys.public_key()
    };

    let p_tag = Tag::public_key(pubkey);
    let event = EventBuilder::new(Kind::Custom(args.kind), args.content, vec![p_tag]).to_unsigned_event(keys.public_key().into());

    println!("Created an event {:?}", event);

    let signed_event = event.sign(&keys)?;

    let client = Client::new(keys);
    client.add_relay(RELAY_URL).await?;
    client.connect().await;
    let broadcast = client.send_event(signed_event).await?;
    client.disconnect().await?;

    println!("Sent event kind: {} w/ id: {:?}", args.kind, broadcast);

    Ok(())
}
