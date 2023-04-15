use clap::Parser;
use clap::Subcommand;
use flate2::read::ZlibDecoder;
use sha1::{Digest, Sha1};

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    HashObject {
        #[clap(short = 'w')]
        actually_write: bool,
        file: PathBuf,
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
        SubCommands::HashObject {
            actually_write: _,
            file,
        } => {
            let file = File::open(file).expect("file not found");
            let buff = {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                reader.read_until(0, &mut buffer).unwrap();
                buffer
            };

            let header = format!("blob {}\x00", &buff.len());
            // concatenate header as byte and file contents
            let mut data = header.as_bytes().to_vec();
            data.extend_from_slice(&buff);
            let mut hasher = Sha1::new();
            hasher.update(&data);
            let hash = hasher.finalize();
            let hex_result = format!("{:#04x}", hash);

            let (dir, filename) = hex_result.split_at(2);
            let object_dir = objects_dir().join(dir);
            let object_file = object_dir.join(filename);
            if !object_dir.exists() {
                fs::create_dir(object_dir).expect("failed to create directory");
            }
            // compress using zlibencoder
            let mut encoder =
                flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(&data).expect("failed to compress");
            let compressed_data = encoder.finish().expect("failed to finish compression");
            fs::write(object_file, compressed_data).expect("failed to write file");
            io::stdout()
                .write_all(&hex_result.as_bytes())
                .expect("failed to write to stdout");
        }
    }
}

fn source_dir() -> PathBuf {
    Path::new(".git").to_path_buf()
}
fn objects_dir() -> PathBuf {
    source_dir().join("objects")
}
fn refs_dir() -> PathBuf {
    source_dir().join("refs")
}
