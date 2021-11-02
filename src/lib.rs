pub mod markov;

use std::fs;
use teloxide::prelude::*;

struct Config {
    pub filename: String,
    pub length: u32,
}

impl Config {
    pub fn new(params: Vec<String>) -> Result<Self, String> {
        match params.len() {
            1 => return Err("missing arguments".into()),
            2 => return Err("missing length".into()),
            _ => (),
        };

        let filename = params[1].clone();

        let length: u32 = match params[2].parse() {
            Ok(length) => length,
            _          => return Err(format!("'{}' is not a natural number", params[2]).into()),
        };

        if length < 2 {
            return Err("the integer must be greater than 1".into())
        };

        Ok(Config { filename, length })
    }
}

#[derive(Clone)]
pub enum Dialogue {
    Start,
    ReceiveCommand,
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
        Dialogue::Start => dialogue_start(cx).await,
        Dialogue::ReceiveCommand => dialogue_rec_cmd(cx).await,
    }
}

async fn dialogue_start(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> TransitionOut<Dialogue> {
    cx.answer("Welcome! This bot is able to generate some random text imitating a source text.\n\nThe available commands are:\n  /help:\n    print this message\n  /generate name n:\n    generate text, where 'name' indicates the source text and 'n' is the word count\n\nThe available source texts are:\n  hpmor:\n    the first chapter of 'Harry Potter and the Methods of Rationality'\n  angelo:\n    the automatically generated subtitles of the YouTube videos of an Italian crackpot")
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
        Some(text) => {
            let command: Vec<String> = text.split_whitespace().map(ToOwned::to_owned).collect();
            match &(command[0])[..] {
                "/help" | "/start" => next(Dialogue::Start),
                "/generate" => match cmd_to_text(command) {
                    Ok(text) => {
                        cx.answer(text)
                            .send()
                            .await?;
                        next(Dialogue::ReceiveCommand)
                    },
                    Err(err) => {
                        cx.answer(format!("Error: {}. Please try again.", err))
                            .send()
                            .await?;
                        next(Dialogue::ReceiveCommand)
                    },
                },
                _ => {
                    cx.answer("Error: could not parse input. Please try again.")
                        .send()
                        .await?;
                    next(Dialogue::ReceiveCommand)
                },
            }
        },
    }
}

fn cmd_to_text(command: Vec<String>) -> Result<String, String> {
    let config = Config::new(command)?;

    if let Ok(text) = fs::read_to_string(config.filename) {
        return Ok(markov::gen_text(&text, config.length));
    };

    Err("could not read the specified source".into())
}
