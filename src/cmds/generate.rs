use clap::{Args, Command, CommandFactory};
use clap_complete::{Generator, Shell};
use std::io;
use crate::Cli;

#[derive(Args, Debug, PartialEq)]
pub struct GenerateArgs {
    #[arg(help = "shell for which to generate autocompletions")]
    pub shell: Shell,
}

pub fn generate(args: GenerateArgs) {
    let mut cmd = Cli::command();
    let shell = args.shell;
    eprintln!("Generating completion file for {shell:?}...");
    print_completions(shell, &mut cmd);
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
        clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

