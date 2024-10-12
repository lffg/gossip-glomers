use proto::{handle, Ctx};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Msg {
    Generate,
    GenerateOk { id: String },
}

fn unique_ids(msg: Msg, ctx: &mut Ctx, counter: &mut u64) {
    let Msg::Generate = msg else {
        panic!("unexpected {msg:?}");
    };

    let id = format!("{}:{}", &ctx.node_id, *counter);
    *counter += 1;

    ctx.reply(Msg::GenerateOk { id });
}

fn main() {
    let mut counter = 0;
    handle(|msg, context| unique_ids(msg, context, &mut counter));
}
