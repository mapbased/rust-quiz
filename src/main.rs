#![allow(
    clippy::let_underscore_untyped,
    clippy::match_bool,
    clippy::uninlined_format_args
)]

mod error;
mod render;
mod serve;

use crate::error::{Error, Result};
use clap::{Parser as ClapParser, Subcommand as ClapSubcommand};
use oqueue::{Color::Red, Sequencer};
use std::io::{self, Write};
use std::process;

#[derive(ClapParser, Debug)]
#[command(about = "Rust Quiz", version, author)]
struct Opt {
    #[clap(subcommand)]
    serve: Option<Subcommand>,
}

#[derive(ClapSubcommand, Debug)]
enum Subcommand {
    /// Serve website over http at localhost:8000
    Serve,
}

fn report(result: Result<()>) {
    if let Err(err) = result {
        let task = Sequencer::stderr().begin();
        task.bold_color(Red);
        write!(task, "ERROR");
        task.bold();
        writeln!(task, ": {}", err);
        task.reset_color();
        process::exit(1);
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    report(render::main());

    if opt.serve.is_some() {
        let _ = writeln!(io::stderr());
        report(serve::main().await);
    }
}

#[test]
fn test_cli() {
    <Opt as clap::CommandFactory>::command().debug_assert();
}
