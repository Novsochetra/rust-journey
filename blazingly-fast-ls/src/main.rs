use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let args: Vec<String> = std::env::args().collect();
    let entries: io::Result<fs::ReadDir>;

    if args.len() == 1 {
        entries = fs::read_dir(current_dir);
    } else if args.len() == 2 {
        let args_path = Path::new(&args[1]);
        entries = fs::read_dir(args_path);
    } else {
        entries = Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Too many arguments",
        ));
    }

    let mut folders: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];

    match entries {
        Ok(r) => {
            for entry in r {
                let path = entry.unwrap().path();
                let filename = path.file_name().unwrap().to_str().unwrap();
                // let is_hidden_file = &filename[0..1] == ".";

                if path.is_dir() {
                    folders.push(String::from(filename));
                } else if path.is_file() {
                    files.push(String::from(filename));
                }
            }

            folders.sort();
            files.sort();

            for folder in folders {
                println!("{}/", folder);
            }

            for file in files {
                println!("{}", file);
            }
        }
        Err(_) => {
            // TODO
        }
    }
}
