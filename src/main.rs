#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use flate2::read::ZlibDecoder;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => match args[2].as_str() {
            "-p" => {
                let sha = &args[3];
                let directory = &sha[0..2];
                let filename = &sha[2..];
                let mut data = String::new();
                let file = File::open(format!(".git/objects/{}/{}", directory, filename))
                    .expect("file not found");

                let mut reader = ZlibDecoder::new(file);
                reader.read_to_string(&mut data).unwrap();
                let null_index = data.find("\x00").unwrap();
                let null_len = "\x00".as_bytes().len();
                // let data = &data[null_index + 1..];
                print!("{}", &data[null_index + null_len..]);
                // print!("{}", &data);
            }
            _ => {
                panic!("unknown command: {}", args[2])
            }
        },
        _ => panic!("unknown command: {}", args[1]),
    }
}
