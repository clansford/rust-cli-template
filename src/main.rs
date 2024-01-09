use clap::builder::Styles;
use clap::{CommandFactory, Parser, Subcommand};

#[path = "./cmds/command1.rs"]
mod command1;
#[path = "./cmds/command2.rs"]
mod command2;
#[path = "./cmds/generate.rs"]
mod generate;

#[derive(Parser, Debug, PartialEq)]
// determines command name, plain style keeps that weird bold from printing
#[command(name = "<command name>", styles = Styles::plain())]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

//All commands that come after cli tool name
#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    #[clap(
        alias = "c1",
        about = "command1 about",
        long_about = "'command 1 long about'",
        before_long_help = "<command1 before long help message>",
        before_help = "<command1 before help message>",
        after_help = "<command1 after help message>"
    )]
    Command1(command1::Command1Args),
    #[clap(alias = "c2", about = "<command2 about>")]
    Command2(command2::Command2Args),
    #[clap(about = "generate autocomplete script")]
    Generate(generate::GenerateArgs),
}

fn main() {
    let cli = Cli::parse();
    handle_command(cli);
}

fn handle_command(cli: Cli) {
    //if no command provided print the help
    if cli.command.is_none() {
        Cli::command().styles(Styles::plain()).print_help().unwrap();
    } else {
        match cli.command.unwrap() {
            Commands::Command1(args) => {
                command1::command1(args);
            }
            Commands::Command2(args) => match args.subcommands {
                command2::Command2Subcommands::Subcommand1(args) => {
                    command2::subcommand1(args);
                }
                command2::Command2Subcommands::Subcommand2 => {
                    command2::subcommand2();
                }
            },
            Commands::Generate(args) => {
                generate::generate(args)
            }
        }
    };
}
