use flowst::{parse_args, Action};

use self::timer::start_timer;

mod timer;

#[tokio::main(flavor="current_thread")]
async fn main() {
    let args = parse_args();

    match &args.command {
        Action::Start(arg)=> {
            start_timer(arg.work.into(), arg.rest.into()).await;
        },
    }
} 

