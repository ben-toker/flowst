use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "flowst",
    author = "Ben Toker <btoker.dev>",
    version = "1.0",
    about = "Basic Pomodoro (Flow) timer in Rust.",
    long_about = None,
)]

struct Args {
    ///Command here; set, add, or reset
    command: String,
    ///work time frame
    work: u32,
    ///rest time frame
    rest: u32,
}

#[derive(Subcommand)]
enum Action {
    #[command(name = "set")]
    Set,
    #[command(name = "add")]
    Add,
    #[command(name = "reset")]
    Reset
}
fn main() {
    let args = Args::parse();

    let command = match &args.command[..] {
        "set" => Ok(Action::Set),
        "add" => Ok(Action::Add),
        "reset" => Ok(Action::Reset),
        _ => Err("No command given.")
    };
}

