use std::fs::File;
use std::io::Read;
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
    save_path: Option<PathBuf>, // Don't make optional on release

    #[arg(short, long, value_name = "NAME OR DATE AND TIME")]
    find: Option<String>,

    #[arg(short, long, value_name = "REGEX STRING")]
    regex_contents: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands
{
    Write
    {
        #[clap(short, long)]
        encrypt: bool,

        #[clap()]
        contents: String,
    },
}

struct Line
{
    index: u32,
    name: String,
    contents: String,
    date: Option<NaiveDateTime>,
}

const SAVE_FILE_DIR: &str = "./save/"; // Later let user set directory with save_path
const SAVE_FILE_NAME: &str = "journal.csv";

fn main() -> ()
{
    let cli: Cli = Cli::parse();
    if File::open(SAVE_FILE_DIR.to_owned() + SAVE_FILE_NAME).is_ok()
    {
        read_save();
    }
    else
    {
        create_save();
    }
}

fn read_save() -> ()
{
        
}

fn create_save() -> ()
{
    
}
