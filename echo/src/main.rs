use std::{
    io::{self, Write},
    mem,
};

use serde::{Deserialize, Serialize};

/// A full message
#[derive(Serialize, Deserialize)]
struct FullMsg {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Msg,
}

impl FullMsg {
    pub fn reply(self, body: Msg) -> FullMsg {
        FullMsg {
            src: self.dst,
            dst: self.src,
            body,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Msg {
    #[default]
    Unknown,
    Init {
        msg_id: u32,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u32,
    },
    Echo {
        msg_id: u32,
        echo: String,
    },
    EchoOk {
        msg_id: u32,
        in_reply_to: u32,
        echo: String,
    },
}

fn echo(msg: Msg) -> Msg {
    let Msg::Echo { msg_id, echo } = msg else {
        panic!("unexpected");
    };
    Msg::EchoOk {
        msg_id,
        in_reply_to: msg_id,
        echo,
    }
}

fn main() {
    let mut stdout = io::stdout();
    let lines = io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        let mut msg: FullMsg = serde_json::from_str(&line).expect("valid json");
        let body = mem::take(&mut msg.body);

        let reply_msg = if let Msg::Init { msg_id, .. } = body {
            Msg::InitOk {
                in_reply_to: msg_id,
            }
        } else {
            echo(body)
        };

        let reply = msg.reply(reply_msg);

        let json = serde_json::to_vec(&reply).expect("should serialize");
        stdout.write_all(json.as_slice()).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}
