use clap::Parser;
use clap::Subcommand;
use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        blob_sha: String,
    },
}

fn main() {
    let args = Arguments::parse();
    match args.subcommand {
        SubCommands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }
        SubCommands::CatFile {
            pretty_print: _,
            blob_sha,
        } => {
            let sha = blob_sha;
            let directory = &sha[0..2];
            let filename = &sha[2..];
            let mut data = String::new();
            let file = File::open(format!(".git/objects/{}/{}", directory, filename))
                .expect("file not found");

            let mut reader = ZlibDecoder::new(file);
            reader.read_to_string(&mut data).unwrap();
            let null_index = data.find("\x00").unwrap();
            let null_len = "\x00".as_bytes().len();
            print!("{}", &data[null_index + null_len..]);
        }
    }
}
