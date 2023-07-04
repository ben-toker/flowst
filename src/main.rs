use flowst::{Args, init_vars, parse_args, set_time, Action, TimeArgs};

fn main() {
    init_vars();

    let args = parse_args();

    match &args.command {
        Action::Set(arg) => set_time(&arg),
        Action::Add(_arg) => println!("add to be implemented"),
        Action::Reset => println!("0"),
    }

}

