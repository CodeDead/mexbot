# MexBot - Discord Bot

![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/mexbot)

`MexBot` is a simple Discord bot that can be used to retrieve Maiar.exchange information such as the economics and token values.

## Commands

* `/economics` - Will display an overview of the MEX economics
* `/tokens` - Displays all the available tokens
* `/price` - Displays the current MEX price
* `/price [TOKEN]` - Displays the price of the requested token
* `/help` - Displays helpful information
* `/about` - Displays information about the MexBot

## Usage

You can add the official MexBot to your server using the following link:  
https://discord.com/api/oauth2/authorize?client_id=985236966817812571&permissions=67584&scope=applications.commands%20bot

## Dependencies

* [reqwest](https://crates.io/crates/reqwest)
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [serenity](https://crates.io/crates/serenity)
* [tokio](https://crates.io/crates/tokio)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2022 CodeDead
