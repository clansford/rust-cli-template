use clap::Args;

#[derive(Args, Debug, PartialEq)]
pub struct Command1Args {
    #[arg(help = "help with command1 arg1")]
    pub arg1: String,
    #[arg(short, long, required = false, default_value = "false", help = "help with command1 arg2")]
    pub arg2: bool,
}

pub fn command1(args: Command1Args) {
    println!("Running command1");
    println!("command1 args:\n{:#?}", args);
}
