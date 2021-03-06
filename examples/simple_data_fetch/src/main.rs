use boticordrs::{BoticordClient};

#[tokio::main]
async fn main() {
    let client = BoticordClient::new("".to_string(), 2).expect("failed client");

    let bot_id: String = "724663360934772797".to_string();

    match client.get_bot_info(bot_id).await {
        Ok(res) => {
            println!("Short Description: {}", res.information.short_description.unwrap())
        },
        Err(e) => eprintln!("{}", e),
    }
}