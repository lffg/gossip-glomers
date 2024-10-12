use proto::{handle, Ctx};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Msg {
    Echo { echo: String },
    EchoOk { echo: String },
}

fn echo(msg: Msg, ctx: &mut Ctx<Msg>) {
    let Msg::Echo { echo } = msg else {
        panic!("unexpected {msg:?}");
    };

    ctx.reply(Msg::EchoOk { echo });
}

fn main() {
    handle(echo);
}
