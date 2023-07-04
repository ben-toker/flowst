use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[clap(
    name = "flowst",
    author = "Ben Toker <btoker.dev>",
    version = "1.0",
    about = r#"                                                            
                                                              
          ,--,                                        ___     
  .--., ,--.'|                                      ,--.'|_   
,--.'  \|  | :     ,---.           .---.            |  | :,'  
|  | /\/:  : '    '   ,'\         /. ./|  .--.--.   :  : ' :  
:  : :  |  ' |   /   /   |     .-'-. ' | /  /    '.;__,'  /   
:  | |-,'  | |  .   ; ,. :    /___/ \: ||  :  /`./|  |   |    
|  : :/||  | :  '   | |: : .-'.. '   ' .|  :  ;_  :__,'| :    
|  |  .''  : |__'   | .; :/___/ \:     ' \  \    `. '  : |__  
'  : '  |  | '.'|   :    |.   \  ' .\     `----.   \|  | '.'| 
|  | |  ;  :    ;\   \  /  \   \   ' \ | /  /`--'  /;  :    ; 
|  : \  |  ,   /  `----'    \   \  |--" '--'.     / |  ,   /  
|  |,'   ---`-'              \   \ |      `--'---'   ---`-'   
`--'                          '---"                           

    Basic Pomodoro (Flow) timer in Rust."#,
    long_about = None,
)]
pub struct Args {
   #[clap(subcommand)]
    pub command: Action
}

#[derive(Parser, Debug)]
pub enum Action {
    Set(TimeArgs),
    Add(TimeArgs),
    Reset,
}

#[derive(Parser,Debug)]
pub struct TimeArgs {
    work: u32,

    rest: u32,
}

pub fn parse_args() -> Args {
    Args::parse()
}

//checkes if environmental variables are initiated, initiates if not.
pub fn init_vars() {
    match env::var("work") {
        Err(_) => env::set_var("work", "25"),
        _ => {},
    }

    match env::var("rest") {
        Err(_) => env::set_var("rest", "5"),
        _ => {},
    }
}

pub fn set_time(args: &TimeArgs) {
    env::set_var("work", args.work.to_string());
    env::set_var("rest", args.rest.to_string());
    println!("Work is now: {}, Rest is now: {}", args.work, args.rest);

}
