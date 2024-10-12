use proto::{handle, Ctx, Msg};

fn echo(msg: Msg, ctx: &mut Ctx) {
    let Msg::Echo { echo } = msg else {
        panic!("unexpected {msg:?}");
    };

    ctx.reply(Msg::EchoOk { echo });
}

fn main() {
    handle(echo);
}
