use super::{cstr_from_bytes, UT_HOSTSIZE, UT_LINESIZE, UT_NAMESIZE};
use std::fmt;
use zerocopy::{FromZeroes, FromBytes};

#[repr(C)]
#[derive(Clone, Copy, Debug, FromZeroes, FromBytes)]
pub struct timeval {
    /// Seconds
    pub tv_sec: i64,
    /// Microseconds
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy, FromZeroes, FromBytes)]
pub struct utmp {
    /// Device name of tty - `"/dev/"`
    pub ut_line: [u8; UT_LINESIZE],
    /// Username
    pub ut_user: [u8; UT_NAMESIZE],
    /// Hostname for remote login, or kernel version for run-level message
    pub ut_host: [u8; UT_HOSTSIZE],
    /// Time entry was made
    pub ut_tv: timeval,
}

impl fmt::Debug for utmp {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("utmp")
            .field("ut_line", &cstr_from_bytes(&self.ut_line))
            .field("ut_user", &cstr_from_bytes(&self.ut_user))
            .field("ut_host", &cstr_from_bytes(&self.ut_host))
            .field("ut_tv", &self.ut_tv)
            .finish()
    }
}

#[test]
fn test_size_of_utmp_x64() {
    assert_eq!(std::mem::size_of::<utmp>(), 312);
}
