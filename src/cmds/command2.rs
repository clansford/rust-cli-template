use clap::{Args, Subcommand};

#[derive(Args, Debug, PartialEq)]
pub struct Command2Args {
    #[command(subcommand)]
    pub subcommands: Command2Subcommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command2Subcommands {
    #[command(
        visible_alias = "sc1",
        about = "command2 subcommand1",
        long_about = "command2 subcommand1 long about"
    )]
    Subcommand1(Subcommand1Args),
    #[command(visible_alias = "sc2", about = "command2 subcommand2")]
    Subcommand2,
}

#[derive(Args, Debug, PartialEq)]
pub struct Subcommand1Args {
    #[arg(long, required = false, default_value = "", hide_default_value = true)]
    arg1: String,
    #[arg(long, required = false, default_value = "0", hide_default_value = true)]
    arg2: i32,
    #[arg(long, required = false, default_value = "true")]
    arg3: bool,
}

pub fn subcommand1(args: Subcommand1Args) {
    println!("Running command2 subcommand1");
    println!("command1 args:\n{:#?}", args);
}

pub fn subcommand2() {
    println!("Running command2 subcommand2");
    println!("Doesn't take any args");
}
