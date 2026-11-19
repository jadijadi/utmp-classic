use anyhow::Result;
use std::path::PathBuf;
use std::{env, process};

fn main() -> Result<()> {
    let mut args = env::args_os();
    let program_name = PathBuf::from(args.next().unwrap());
    let path = match args.next() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Usage: {} <path>", program_name.display());
            process::exit(2);
        }
    };

    let entries = utmp_classic::parse_from_path(&path)?;
    for entry in entries {
        println!("{:?}", entry);
    }
    Ok(())
}
