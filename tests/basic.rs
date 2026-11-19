use anyhow::Result;
use once_cell::sync::Lazy;
use std::io::{self, Read};
use std::path::PathBuf;
use time::OffsetDateTime;
use utmp_classic::{Utmp32Parser, UtmpEntry};

static SAMPLES_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "tests", "samples"]));

fn timestamp(nanos: i128) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp_nanos(nanos).unwrap()
}

fn get_basic_expected() -> Vec<UtmpEntry> {
    vec![UtmpEntry::UTMP {
        line: "ttyC3".to_owned(),
        user: "jadi".to_owned(),
        host: "".to_owned(),
        time: timestamp(1_714_663_553_000_000_000),
    }]
}
#[cfg(target_endian = "little")]
#[test]
fn parse_basic32() -> Result<()> {
    let path = SAMPLES_PATH.join("basic.utmp");
    let actual = Utmp32Parser::from_path(&path)?.collect::<Result<Vec<_>, _>>()?;
    let expected = get_basic_expected();
    Ok(assert_eq!(actual[5], expected[0]))
}

#[allow(dead_code)]
struct ByteReader<R>(R);

impl<R: Read> Read for ByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            self.0.read(buf)
        } else {
            self.0.read(&mut buf[..1])
        }
    }
}
