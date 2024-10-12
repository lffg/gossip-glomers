use std::{
    borrow::Cow,
    io::{self, BufWriter, Write},
    marker::PhantomData,
};

use serde::{Deserialize, Serialize};

/// A full message
#[derive(Serialize, Deserialize)]
pub struct Msg<'a, M> {
    pub src: Cow<'a, str>,
    #[serde(rename = "dest")]
    pub dst: Cow<'a, str>,
    pub body: Body<M>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Body<M> {
    #[serde(rename = "msg_id")]
    pub id: Option<u32>,
    #[serde(rename = "in_reply_to")]
    pub reply_to: Option<u32>,
    #[serde(flatten)]
    pub msg: M,
}

pub struct Ctx<'init, M> {
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

    _m: PhantomData<M>,
}

impl<'init, M> Ctx<'init, M>
where
    M: Serialize,
{
    fn update<'new_curr>(self, in_reply_to: Option<u32>, src: String) -> Ctx<'init, M> {
        Ctx {
            node_id: self.node_id,
            nodes_ids: self.nodes_ids,
            src,
            in_reply_to,
            writer: self.writer,
            count: self.count,
            _m: PhantomData,
        }
    }

    pub fn reply(&mut self, msg: M) {
        self.reply_any(msg);
    }

    pub fn send(&mut self, dst: &str, msg: M) {
        let body = Body {
            id: Some(self.count()),
            reply_to: None,
            msg,
        };
        write(
            &mut self.writer,
            Msg {
                src: Cow::Borrowed(&self.node_id),
                dst: Cow::Borrowed(dst),
                body,
            },
        );
    }

    fn reply_any<N>(&mut self, msg: N)
    where
        N: Serialize,
    {
        let body = Body {
            id: Some(self.count()),
            reply_to: self.in_reply_to,
            msg,
        };
        write(
            &mut self.writer,
            Msg {
                src: Cow::Borrowed(&self.node_id),
                dst: Cow::Borrowed(&self.src),
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
pub fn handle<F, M>(mut f: F)
where
    F: for<'a> FnMut(M, &'a mut Ctx<M>),
    M: Serialize + for<'de> Deserialize<'de>,
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
    ctx.reply_any(Init::InitOk);

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Init {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

fn init_ctx<'init, M>(w: BufWriter<io::Stdout>, msg: &'init Msg<Init>) -> Ctx<'init, M> {
    let Init::Init { node_id, node_ids } = &msg.body.msg else {
        panic!("expected `Init` msg, got {:?}", msg.body.msg);
    };
    Ctx {
        node_id: &node_id,
        nodes_ids: &node_ids,
        src: msg.src.to_string(), // Here we clone (just once)
        in_reply_to: msg.body.id,
        writer: w,
        count: 0,
        _m: PhantomData,
    }
}

fn decode<'de, M>(raw: &'de str) -> Msg<M>
where
    M: Deserialize<'de>,
{
    serde_json::from_str(raw).expect("should deserialize `FullMsg`")
}

#[inline]
fn write<W, M>(w: &mut BufWriter<W>, msg: Msg<M>)
where
    W: Write,
    M: Serialize,
{
    let json = serde_json::to_vec(&msg).expect("should serialize `FullMsg`");
    w.write_all(&json).expect("should write `FullMsg`");
    w.write_all(b"\n").expect("should write newline");
    w.flush().expect("should flush");
}
