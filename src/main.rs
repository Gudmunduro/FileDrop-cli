use std::path::PathBuf;
use structopt::StructOpt;
use crate::api::FiledropApiClient;

pub mod models;
pub mod api;
pub mod commands;

#[derive(StructOpt)]
struct Options {
    command: String,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args: Options = Options::from_args();
    
    commands::execute_command(&args.command, &args.files);
}
