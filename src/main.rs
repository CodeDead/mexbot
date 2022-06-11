use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use services::elrond_service::ElrondService;

mod models;
mod services;

struct Handler {
    pub elrond_service: ElrondService,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        async fn transmit_message(msg: &Message, ctx: &Context, message_content: &str) {
            if let Err(why) = msg.channel_id.say(&ctx.http, message_content).await {
                println!("Error sending message: {:?}", why);
            }
        }

        let author = msg.author.id.clone();

        let mut command = msg.content.as_str().split_whitespace().peekable();
        let cmd = command.next().expect("Command expected!");
        match cmd {
            "/economics" => {
                let res = self.elrond_service.get_economics().await;
                match res {
                    Ok(response) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nMEX economics:{}",
                                author.as_u64(),
                                response.to_string()
                            ),
                        )
                        .await;
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!("Error retrieving economics: {:?}", error),
                        )
                        .await;
                    }
                }
            }
            "/price" => {
                let res = self.elrond_service.get_tokens().await;
                let mut symbol_to_compare = "MEX";
                if command.peek().is_some() {
                    symbol_to_compare = command.next().expect("Symbol was empty!");
                }

                let not_found = symbol_to_compare.clone();
                match res {
                    Ok(response) => {
                        let mex = response
                            .iter()
                            .filter(|token| {
                                token.symbol.to_lowercase() == symbol_to_compare.to_lowercase()
                            })
                            .nth(0);
                        let default_currency = response
                            .iter()
                            .filter(move |token| {
                                let price = match token.price.as_f64() {
                                    Some(p) => p,
                                    None => 0f64,
                                };

                                price == 1.0
                            })
                            .nth(0);

                        if let Some(x) = mex {
                            let price = match x.price.as_f64() {
                                Some(d) => d,
                                None => 0.0,
                            };
                            if let Some(y) = default_currency {
                                if x.symbol == y.symbol {
                                    transmit_message(&msg, &ctx, "Don't be silly").await;
                                } else {
                                    transmit_message(
                                        &msg,
                                        &ctx,
                                        &format!(
                                            "<@{}>\n1 {} = {} {}",
                                            author.as_u64(),
                                            x.symbol,
                                            price,
                                            y.symbol
                                        ),
                                    )
                                    .await;
                                }
                            }
                        } else {
                            transmit_message(
                                &msg,
                                &ctx,
                                &format!("Currency {} could not be found", not_found),
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!("Error retrieving economics: {:?}", error),
                        )
                        .await;
                    }
                }
            }
            "/help" => {
                transmit_message(&msg, &ctx, &format!("<@{}>\nCommands:```/economics - Display the Maiar exchange economics\n/price - Display the MEX price\n/price [TOKEN] - Display the price of the given token\n/about - Display about information```", author.as_u64())).await;
            }
            "/about" => {
                transmit_message(&msg, &ctx, &format!("<@{}>\nThis bot was created by CodeDead.\nWebsite: https://codedead.com/\nDonate: https://codedead.com/donate\nData source: https://maiar.exchange/", author.as_u64())).await;
            }
            _ => {
                // Do nothing
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = String::from("YOUR TOKEN HERE");

    let elrond_service = ElrondService::new(
        "https://api.elrond.com/mex/economics",
        "https://api.elrond.com/mex/tokens",
    );

    let handler = Handler { elrond_service };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
