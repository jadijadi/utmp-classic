use std::ffi::CStr;
use std::os::raw::c_short;
use thiserror::Error;
use time::OffsetDateTime;
use utmp_classic_raw::x32::utmp as utmp32;
use utmp_classic_raw::x64::{timeval as timeval64, utmp as utmp64};

/// Parsed utmp entry.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UtmpEntry {
    UTMP {
        /// Device name of tty
        line: String,
        /// Username
        user: String,
        /// Hostname for remote login
        host: String,
        /// Session ID (`getsid(2)`)
        time: OffsetDateTime,
        // TODO: Figure out the correct byte order to parse the address
    },    
}

impl<'a> TryFrom<&'a utmp32> for UtmpEntry {
    type Error = UtmpError;

    fn try_from(from: &utmp32) -> Result<Self, UtmpError> {
        UtmpEntry::try_from(&utmp64 {
            ut_line: from.ut_line,
            ut_user: from.ut_user,
            ut_host: from.ut_host,
            ut_tv: timeval64 {
                tv_sec: i64::from(from.ut_tv.tv_sec),
                tv_usec: i64::from(from.ut_tv.tv_usec),
            },
        })
    }
}

impl<'a> TryFrom<&'a utmp64> for UtmpEntry {
    type Error = UtmpError;

    fn try_from(from: &utmp64) -> Result<Self, UtmpError> {
        Ok(UtmpEntry::UTMP {
                line: string_from_bytes(&from.ut_line).map_err(UtmpError::InvalidLine)?,
                user: string_from_bytes(&from.ut_user).map_err(UtmpError::InvalidUser)?,
                host: string_from_bytes(&from.ut_host).map_err(UtmpError::InvalidHost)?,
                time: time_from_tv(from.ut_tv)?,
        })
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum UtmpError {
    #[error("unknown type {0}")]
    UnknownType(c_short),
    #[error("invalid time value {0:?}")]
    InvalidTime(timeval64),
    #[error("invalid line value `{0:?}`")]
    InvalidLine(Box<[u8]>),
    #[error("invalid user value `{0:?}`")]
    InvalidUser(Box<[u8]>),
    #[error("invalid host value `{0:?}`")]
    InvalidHost(Box<[u8]>),
}

fn time_from_tv(tv: timeval64) -> Result<OffsetDateTime, UtmpError> {
    let timeval64 { tv_sec, tv_usec } = tv;
    if tv_usec < 0 {
        return Err(UtmpError::InvalidTime(tv));
    }
    let usec = i128::from(tv_sec) * 1_000_000 + i128::from(tv_usec);
    OffsetDateTime::from_unix_timestamp_nanos(usec * 1000).map_err(|_| UtmpError::InvalidTime(tv))
}

fn string_from_bytes(bytes: &[u8]) -> Result<String, Box<[u8]>> {
    bytes
        .iter()
        .position(|b| *b == 0)
        .and_then(|pos| {
            // This is safe because we manually located the first zero byte above.
            let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..=pos]) };
            Some(cstr.to_str().ok()?.to_string())
        })
        .ok_or_else(|| bytes.to_owned().into_boxed_slice())
}
