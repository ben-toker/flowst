use flowst::{init_vars, parse_args, set_time, Action};

use self::timer::start_timer;
mod timer;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    init_vars();

    let args = parse_args();

    match &args.command {
        Action::Set(arg) => set_time(&arg),
        Action::Start(_) => start_timer().await,
        Action::Add(_arg) => println!("add to be implemented"),
        Action::Reset => println!("0"),
    }

}

