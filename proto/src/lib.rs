use std::{
    io::{self, BufWriter, Write},
    mem,
};

use serde::{Deserialize, Serialize};

/// A full message
#[derive(Serialize, Deserialize)]
pub struct FullMsg {
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

    fn take(&mut self) -> Msg {
        mem::take(&mut self.body)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Msg {
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
    Generate {
        msg_id: u32,
    },
    GenerateOk {
        msg_id: u32,
        in_reply_to: u32,
        id: String,
    },
}

pub struct Context {
    /// The ID of the current node.
    pub node_id: String,
    pub nodes_ids: Vec<String>,
}

/// Handles a message.
pub fn handle<F>(mut f: F)
where
    F: for<'a> FnMut(Msg, &'a Context) -> Msg,
{
    let mut s = ();
    handle_s(&mut s, |m, c, _| f(m, c));
}

/// Handles a message with a custom state.
pub fn handle_s<S, F>(state: &mut S, mut f: F)
where
    F: for<'a> FnMut(Msg, &'a Context, &mut S) -> Msg,
{
    let mut stdout = BufWriter::new(io::stdout());
    let stdin = io::stdin();

    let mut lines = stdin.lines();

    // First line must be init message
    let init = lines
        .next()
        .expect("should receive `Init` message")
        .expect("failed to read message");
    let mut init = decode(&init);

    let init_msg = init.take();
    let (init_reply, context) = handle_init(init_msg);
    let init_reply = init.reply(init_reply);
    write(&mut stdout, init_reply);

    for line in lines {
        let mut msg = decode(&line.expect("failed to read message"));
        let body = msg.take();
        let reply = f(body, &context, state);
        let reply = msg.reply(reply);
        write(&mut stdout, reply);
    }
}

fn handle_init(msg: Msg) -> (Msg, Context) {
    let Msg::Init {
        msg_id,
        node_id,
        node_ids,
    } = msg
    else {
        panic!("unexpected {msg:?}");
    };
    let reply = Msg::InitOk {
        in_reply_to: msg_id,
    };
    let context = Context {
        node_id,
        nodes_ids: node_ids,
    };
    (reply, context)
}

fn decode(raw: &str) -> FullMsg {
    serde_json::from_str(raw).expect("should deserialize `FullMsg`")
}

fn write<W>(w: &mut BufWriter<W>, msg: FullMsg)
where
    W: Write,
{
    let json = serde_json::to_vec(&msg).expect("should serialize `FullMsg`");
    w.write_all(&json).expect("should write `FullMsg`");
    w.write_all(b"\n").expect("should write newline");
    w.flush().expect("should flush");
}
