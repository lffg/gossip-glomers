use std::{
    borrow::Cow,
    collections::HashMap,
    io::{self, BufWriter, Write},
};

use serde::{Deserialize, Serialize};

/// A full message
#[derive(Serialize, Deserialize)]
pub struct FullMsg<'a> {
    pub src: Cow<'a, str>,
    #[serde(rename = "dest")]
    pub dst: Cow<'a, str>,
    pub body: Body,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<u32>,
    #[serde(rename = "in_reply_to")]
    pub reply_to: Option<u32>,
    #[serde(flatten)]
    pub msg: Msg,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Msg {
    #[default]
    Unknown,
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: String,
    },
    Broadcast {
        message: i32,
    },
    BroadcastOk,
    /// Our own
    Gossip {
        message: i32,
    },
    /// Our own
    GossipOk,
    Read,
    ReadOk {
        messages: Vec<i32>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

pub struct Ctx<'init> {
    /// The ID of the current node.
    pub node_id: &'init str,
    pub nodes_ids: &'init [String],

    /// The source of the message that is currently being handled.
    src: String,
    /// The message ID (if any) of the message that is currently being handled.
    in_reply_to: Option<u32>,

    writer: BufWriter<io::Stdout>,
    /// Counter of sent messages.
    count: u32,
}

impl<'init> Ctx<'init> {
    fn update<'new_curr>(self, in_reply_to: Option<u32>, src: String) -> Ctx<'init> {
        Ctx {
            node_id: self.node_id,
            nodes_ids: self.nodes_ids,
            src,
            in_reply_to,
            writer: self.writer,
            count: self.count,
        }
    }

    pub fn reply(&mut self, msg: Msg) {
        let body = Body {
            id: Some(self.count()),
            reply_to: self.in_reply_to,
            msg,
        };
        write(
            &mut self.writer,
            FullMsg {
                src: Cow::Borrowed(&self.node_id),
                dst: Cow::Borrowed(&self.src),
                body,
            },
        );
    }

    pub fn send(&mut self, dst: &str, msg: Msg) {
        let body = Body {
            id: Some(self.count()),
            reply_to: None,
            msg,
        };
        write(
            &mut self.writer,
            FullMsg {
                src: Cow::Borrowed(&self.node_id),
                dst: Cow::Borrowed(dst),
                body,
            },
        );
    }

    fn count(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}

/// Handles a message.
pub fn handle<F>(mut f: F)
where
    F: for<'a> FnMut(Msg, &'a mut Ctx),
{
    let stdout = BufWriter::new(io::stdout());
    let stdin = io::stdin();

    let mut lines = stdin.lines();

    // First line must be init message
    let init = lines
        .next()
        .expect("should receive `Init` message")
        .expect("should to read message");
    let init = decode(&init);

    let mut ctx = init_ctx(stdout, &init);
    ctx.reply(Msg::InitOk);

    for line in lines {
        let msg = line.expect("should read message");
        let msg = decode(&msg);
        let Cow::Owned(src) = msg.src else {
            unreachable!()
        };
        ctx = ctx.update(msg.body.id, src);
        f(msg.body.msg, &mut ctx);
    }
}

fn init_ctx<'init>(w: BufWriter<io::Stdout>, msg: &'init FullMsg) -> Ctx<'init> {
    let Msg::Init { node_id, node_ids } = &msg.body.msg else {
        panic!("expected `Init` msg, got {:?}", msg.body.msg);
    };
    Ctx {
        node_id: &node_id,
        nodes_ids: &node_ids,
        src: msg.src.to_string(), // Here we clone (just once)
        in_reply_to: msg.body.id,
        writer: w,
        count: 0,
    }
}

fn decode(raw: &str) -> FullMsg {
    serde_json::from_str(raw).expect("should deserialize `FullMsg`")
}

#[inline]
fn write<W>(w: &mut BufWriter<W>, msg: FullMsg)
where
    W: Write,
{
    let json = serde_json::to_vec(&msg).expect("should serialize `FullMsg`");
    w.write_all(&json).expect("should write `FullMsg`");
    w.write_all(b"\n").expect("should write newline");
    w.flush().expect("should flush");
}
