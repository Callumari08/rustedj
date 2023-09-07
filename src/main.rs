use std::path::PathBuf;
use chrono::NaiveDateTime;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli
{
    username: String,
    password: String,

    #[arg(short, long, value_name = "DIRECTORY")]
    save_path: Option<PathBuf>,

    #[arg(short, long, value_name = "NAME OR DATE AND TIME")]
    find: Option<String>,

    #[arg(short, long, value_name = "REGEX STRING")]
    regex_contents: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands
{
    #[arg(short, long)]
    Write
    {
        contents: String,
    },
}

struct Line
{
    index: u32,
    name: String,
    date: Option<NaiveDateTime>,
    contents: String,
}

fn main() 
{
    let cli: Cli = Cli::parse();
    

    
}
