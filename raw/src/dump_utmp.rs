use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem;
use std::path::PathBuf;
use std::process;
use utmp_classic::utmp;
use zerocopy::LayoutVerified;

const SIZE: usize = mem::size_of::<utmp>();

#[repr(align(8))]
struct Buffer([u8; SIZE]);

fn main() -> io::Result<()> {
    let mut args = env::args_os();
    let program_name = PathBuf::from(args.next().unwrap());
    let path = match args.next() {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Usage: {} <path>", program_name.display());
            process::exit(2);
        }
    };

    let mut f = File::open(&path)?;
    let mut buffer = Buffer([0; SIZE]);
    while let Ok(()) = f.read_exact(&mut buffer.0) {
        let buffer = buffer.0.as_ref();
        let record = LayoutVerified::<_, utmp>::new(buffer).unwrap().into_ref();
        println!("{:#?}", record);
    }
    Ok(())
}
