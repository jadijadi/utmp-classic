#![allow(non_camel_case_types)]

use cfg_if::cfg_if;
use std::ffi::CStr;
use std::os::raw::c_short;

pub mod x32;
pub mod x64;

/// Record does not contain valid info (formerly known as `UT_UNKNOWN` on Linux)
pub const EMPTY: c_short = 0;
/// Change in system run-level (see `init(8)`)
pub const RUN_LVL: c_short = 1;
/// Time of system boot (in `ut_tv`)
pub const BOOT_TIME: c_short = 2;
/// Time after system clock change (in `ut_tv`)
pub const NEW_TIME: c_short = 3;
/// Time before system clock change (in `ut_tv`)
pub const OLD_TIME: c_short = 4;
/// Process spawned by `init(8)`
pub const INIT_PROCESS: c_short = 5;
/// Session leader process for user login
pub const LOGIN_PROCESS: c_short = 6;
/// Normal process
pub const USER_PROCESS: c_short = 7;
/// Terminated process
pub const DEAD_PROCESS: c_short = 8;
/// Not implemented
pub const ACCOUNTING: c_short = 9;
pub const UTMP: c_short = 10;

pub const UT_LINESIZE: usize = 8;
pub const UT_NAMESIZE: usize = 32;
pub const UT_HOSTSIZE: usize = 256;

/// Type for `ut_exit`, below
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct exit_status {
    /// Process termination status
    pub e_termination: c_short,
    /// Process exit status
    pub e_exit: c_short,
}

cfg_if! {
    if #[cfg(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "sparc",
        target_arch = "sparc64",
        target_arch = "aarch64",
        target_arch = "loongarch64",
    ))] {
        pub use x32::*;
    } else if #[cfg(any(
        target_arch = "s390x",
    ))] {
        pub use x64::*;
    } else {
        compile_error!("The target platform is not supported, please help us add it.");
    }
}

fn cstr_from_bytes(bytes: &[u8]) -> &CStr {
    match bytes.iter().position(|b| *b == 0) {
        // This is safe because we manually located the first zero byte above.
        Some(pos) => unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[..=pos]) },
        // This is safe because we manually generated this string.
        None => unsafe { CStr::from_bytes_with_nul_unchecked("???\0".as_bytes()) },
    }
}


