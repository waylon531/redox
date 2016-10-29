/// Convert syscall types to libc types
extern crate syscall;

use core::{slice, str};

use super::{c_char, c_int, c_void, mode_t, off_t, pid_t, gid_t, uid_t, sa_family_t, size_t, ssize_t, strlen};

use self::syscall::data::{Stat, TimeSpec};

pub use self::syscall::error::*;
pub use self::syscall::flag::*;
pub use self::syscall::{
    clock_gettime, clone, execve as exec, exit, futex, kill, nanosleep, setgid, setuid, waitpid,
    chdir, getcwd, open, mkdir, rmdir, unlink, dup, pipe2,
    read, write, fpath, fstat, fsync, ftruncate, lseek, close
};

//TODO: Thread local
pub static mut errno: c_int = 0;

pub type stat = Stat;
pub type timespec = TimeSpec;

pub const AF_INET: sa_family_t = 1;
pub const AF_INET6: sa_family_t = 2;

pub const STDIN_FILENO: usize = 0;
pub const STDOUT_FILENO: usize = 1;
pub const STDERR_FILENO: usize = 2;

fn cvt(result: syscall::Result<usize>) -> c_int {
    match result {
        Ok(res) => res as c_int,
        Err(err) => {
            unsafe { errno = err.errno };
            -1
        }
    }
}

// ralloc shims {
/// Cooperatively gives up a timeslice to the OS scheduler.
#[no_mangle]
pub unsafe extern "C" fn sched_yield() -> c_int {
    cvt(syscall::sched_yield())
}

/// Increment data segment of this process by some, _n_, return a pointer to the new data segment
/// start.
///
/// This uses the system call BRK as backend.
///
/// This is unsafe for multiple reasons. Most importantly, it can create an inconsistent state,
/// because it is not atomic. Thus, it can be used to create Undefined Behavior.
#[no_mangle]
pub extern "C" fn sbrk(n: isize) -> *mut u8 {
    let orig_seg_end = match unsafe { syscall::brk(0) } {
        Ok(end) => end,
        Err(_) => return !0 as *mut u8
    };

    if n == 0 {
        return orig_seg_end as *mut u8;
    }

    let expected_end = match orig_seg_end.checked_add(n as usize) {
        Some(end) => end,
        None => return !0 as *mut u8
    };

    let new_seg_end = match unsafe { syscall::brk(expected_end) } {
        Ok(end) => end,
        Err(_) => return !0 as *mut u8
    };

    if new_seg_end != expected_end {
        // Reset the break.
        let _ = unsafe { syscall::brk(orig_seg_end) };

        !0 as *mut u8
    } else {
        orig_seg_end as *mut u8
    }
}
// } ralloc shims

pub unsafe fn fcntl(fd: usize, cmd: c_int, arg: c_int) -> c_int {
    unimplemented!();
}
