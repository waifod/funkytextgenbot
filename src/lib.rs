pub mod markov;

use std::fs;
use teloxide::prelude::*;

struct Config {
    pub length: u32,
    pub filename: String,
}

impl Config {
    pub fn new(message: String) -> Result<Self, String> {
        let mut args = message.split_whitespace();

        let length: u32 = match args.next() {
            Some(length) => match length.parse() {
                Ok(length) => length,
                _          => return Err(format!("'{}' is not a natural number", length).into()),
            },
            None => return Err("missing arguments".into()),
        };

        let filename = match args.next() {
            Some(filename) => filename.to_string(),
            None => return Err("missing source".into()),
        };

        Ok(Config { length, filename })
    }
}

#[derive(Clone)]
pub enum Dialogue {
    Start,
    ReceiveCommand,
    ReceiveConfig,
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start
    }
}

pub async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match dialogue {
        Dialogue::Start => Ok(dialogue_start(cx).await?),
        Dialogue::ReceiveCommand => Ok(dialogue_rec_cmd(cx).await?),
        Dialogue::ReceiveConfig => Ok(dialogue_rec_cfg(cx).await?),
    }
}

async fn dialogue_start(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> TransitionOut<Dialogue> {
    cx.answer("Welcome! This bot is able to generate some random text based upon other text. You can indicate the source to be imitated and the length of the output. The available commands are:\n/help: print this message\n/generate: generate text")
        .send()
        .await?;
    next(Dialogue::ReceiveCommand)
}

async fn dialogue_rec_cmd(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Oh, please, send me a text message!")
                .send()
                .await?;
            next(Dialogue::ReceiveCommand)
        },
        Some(text) => match text.trim() {
            "/help" | "/start" => {
                cx.answer("Welcome! This bot is able to generate some random text based upon other text. You can indicate the length of the output and the source to be imitated.\nThe available commands are:\n/help: print this message\n/generate: generate text")
                    .send()
                    .await?;
                next(Dialogue::ReceiveCommand)
            },
            "/generate" => {
                cx.answer("Please input length and source text.\nThe texts currently available are:\n-'angelo': subtitles taken from the YouTube videos of an Italian crackpot\n-'hpmor': the first chapter of 'Harry Potter and the Methods of Rationality'") 
                    .send()
                    .await?;
                next(Dialogue::ReceiveConfig)
            },
            _ => {
                cx.answer("Error: can not parse input. Please try again.")
                    .send()
                    .await?;
                next(Dialogue::ReceiveCommand)
            },
        },
    }
}

async fn dialogue_rec_cfg(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> TransitionOut<Dialogue> {
    match cx.update.text() {
        Some(input) => {
            match Config::new(input.to_string()) {
                Ok(config) => {
                    if let Ok(text) = fs::read_to_string(config.filename) {
                        cx.answer(markov::gen_text(&text, config.length))
                            .send()
                            .await?;
                        cx.answer("Waiting for the next command.")
                            .send()
                            .await?;
                        next(Dialogue::ReceiveCommand)
                    } else {
                        cx.answer("Error: could not read the specified source. Please try again.")
                            .send()
                            .await?;
                        next(Dialogue::ReceiveConfig)
                    }
                },
                Err(err) => {
                    cx.answer(format!("Error: {}. Please try again.", err))
                        .send()
                        .await?;
                    next(Dialogue::ReceiveConfig)
                },
            }
        },
        None => {
            cx.answer("Oh, please, send me a text message!")
                .send()
                .await?;
            next(Dialogue::ReceiveConfig)
        },
    }
}