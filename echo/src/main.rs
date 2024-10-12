use proto::{handle, Context, Msg};

fn echo(msg: Msg, _: &Context) -> Msg {
    let Msg::Echo { msg_id, echo } = msg else {
        panic!("unexpected {msg:?}");
    };
    Msg::EchoOk {
        msg_id,
        in_reply_to: msg_id,
        echo,
    }
}

fn main() {
    handle(echo);
}
