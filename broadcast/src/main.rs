//! This works both as a single-node and multi-node broadcast!

use std::collections::HashMap;

use proto::{handle, Ctx};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Msg {
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

fn broadcast(msg: Msg, ctx: &mut Ctx<Msg>, store: &mut Vec<i32>) {
    match msg {
        Msg::Broadcast { message } => {
            for node in ctx.nodes_ids {
                ctx.send(node, Msg::Gossip { message });
            }
            ctx.reply(Msg::BroadcastOk);
        }
        Msg::Gossip { message } => {
            store.push(message);
            ctx.reply(Msg::GossipOk);
        }
        Msg::GossipOk => (),
        Msg::Read => ctx.reply(Msg::ReadOk {
            messages: store.clone(),
        }),
        Msg::Topology { topology: _ } => ctx.reply(Msg::TopologyOk),
        unexpected => panic!("unexpected {unexpected:?}"),
    }
}

fn main() {
    let mut store = Vec::new();
    handle(|msg, context| broadcast(msg, context, &mut store));
}
