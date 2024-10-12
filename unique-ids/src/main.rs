use proto::{handle, Context, Msg};

fn unique_ids(msg: Msg, ctx: &Context, counter: &mut u64) -> Msg {
    let Msg::Generate { msg_id } = msg else {
        panic!("unexpected {msg:?}");
    };

    let id = format!("{}:{}", &ctx.node_id, *counter);
    *counter += 1;

    Msg::GenerateOk {
        msg_id,
        in_reply_to: msg_id,
        id,
    }
}

fn main() {
    let mut counter = 0;
    handle(|msg, context| unique_ids(msg, context, &mut counter));
}
