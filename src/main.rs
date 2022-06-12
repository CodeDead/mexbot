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
        match cmd.to_lowercase().as_str() {
            "/economics" => {
                let res = self.elrond_service.get_economics().await;
                match res {
                    Ok(response) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nMEX economics:\n{}",
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
                            &format!(
                                "<@{}>\nError retrieving economics: {:?}",
                                author.as_u64(),
                                error
                            ),
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
                                    transmit_message(
                                        &msg,
                                        &ctx,
                                        &format!("<@{}>\nDon't be silly", author.as_u64()),
                                    )
                                    .await;
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
                                &format!(
                                    "<@{}>\nCurrency {} could not be found",
                                    author.as_u64(),
                                    symbol_to_compare
                                ),
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nError retrieving economics: {:?}",
                                author.as_u64(),
                                error
                            ),
                        )
                        .await;
                    }
                }
            }
            "/tokens" => {
                let res = self.elrond_service.get_tokens().await;
                match res {
                    Ok(result) => {
                        if result.len() == 0 {
                            transmit_message(
                                &msg,
                                &ctx,
                                &format!("<@{}>\nNo MEX tokens listed", author.as_u64()),
                            )
                            .await;
                        } else {
                            let mut token_string =
                                format!("<@{}>\nMEX tokens:\n```", author.as_u64());
                            for token in result.iter() {
                                token_string
                                    .push_str(&format!("{} - {}\n", &token.symbol, &token.name));
                            }
                            token_string.push_str("```");
                            transmit_message(&msg, &ctx, &token_string).await;
                        }
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nError retrieving tokens: {:?}",
                                author.as_u64(),
                                error
                            ),
                        )
                        .await;
                    }
                }
            }
            "/farms" => {
                let response = self.elrond_service.get_farms().await;
                match response {
                    Ok(result) => {
                        if result.len() == 0 {
                            transmit_message(
                                &msg,
                                &ctx,
                                &format!("<@{}>\nNo farms active at the moment", author.as_u64()),
                            )
                            .await;
                        } else {
                            let farms_message = format!("<@{}>\nMEX farms:\n", author.as_u64());
                            transmit_message(&msg, &ctx, &farms_message).await;

                            for farm in result.iter() {
                                if farm.farm_type.to_lowercase() == "standard" {
                                    transmit_message(&msg, &ctx, &format!("```Name: {} | {}\nType: {}\nFarmed symbol: {}\nFarming price: {}\nFarmed price: {}```\n", farm.farming_symbol, farm.farming_name, farm.farm_type, farm.farmed_symbol, farm.farming_price, farm.farmed_price)).await;
                                }
                            }
                        }
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nError retrieving farms: {:?}",
                                author.as_u64(),
                                error
                            ),
                        )
                        .await;
                    }
                }
            }
            "/metastaking" => {
                let response = self.elrond_service.get_farms().await;
                match response {
                    Ok(result) => {
                        if result.len() == 0 {
                            transmit_message(
                                &msg,
                                &ctx,
                                &format!(
                                    "<@{}>\nNo metastaking farms active at the moment",
                                    author.as_u64()
                                ),
                            )
                            .await;
                        } else {
                            let farms_message =
                                format!("<@{}>\nMEX metastaking:\n", author.as_u64());
                            transmit_message(&msg, &ctx, &farms_message).await;

                            for farm in result.iter() {
                                if farm.farm_type.to_lowercase() == "metastaking" {
                                    transmit_message(&msg, &ctx, &format!("```Name: {} | {}\nType: {}\nFarmed symbol: {}\nFarming price: {}\nFarmed price: {}```\n", farm.farming_symbol, farm.farming_name, farm.farm_type, farm.farmed_symbol, farm.farming_price, farm.farmed_price)).await;
                                }
                            }
                        }
                    }
                    Err(error) => {
                        transmit_message(
                            &msg,
                            &ctx,
                            &format!(
                                "<@{}>\nError retrieving metastaking farms: {:?}",
                                author.as_u64(),
                                error
                            ),
                        )
                        .await;
                    }
                }
            }
            "/help" => {
                transmit_message(&msg, &ctx, &format!("<@{}>\nCommands:\n```/economics - Display the MEX economics\n/tokens - Display all the available tokens\n/price - Display the MEX price\n/price [TOKEN] - Display the price of the given token\n/farms - Display MEX standard farms information\n/metastaking - Display MEX metastaking farms information\n/about - Display about information```", author.as_u64())).await;
            }
            "/about" => {
                transmit_message(&msg, &ctx, &format!("<@{}>\nThis bot was created by CodeDead.\nWebsite: https://codedead.com/\nDonate: https://codedead.com/donate\nDonate EGLD: erd1rdc6w82ftjsyp5ethh0q56297fsef6w5ht75vyltcjh3ms220urqezdhd3\nData source: https://maiar.exchange/", author.as_u64())).await;
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
        "https://api.elrond.com/mex/farms",
    );

    let handler = Handler { elrond_service };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
