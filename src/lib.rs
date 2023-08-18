//! Signal process when parent terminates
//!
//! A subprocess whose parent exits will be re-parented to init (PID 1)
//! and continue to run. `pdeathsigexec` sets the process to have a signal
//! sent if the parent process terminates.
//!
//! The "signal on parent termination" behaviour applies
//! to the executed process only and not descendents
//! ([prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html)):
//!
//! ```
//! The parent-death signal setting is cleared for the child of a fork(2).
//! It is also (since Linux 2.4.36 / 2.6.23) cleared when  executing  a
//! set-user-ID or set-group-ID binary, or a binary that has associated
//! capabilities (see capabilities(7)); otherwise, this value is preserved
//! across execve(2).
//! ```
use libc::c_char;
use std::ffi::CString;

#[cfg(target_os = "linux")]
use libc::{__errno_location, prctl, PR_SET_PDEATHSIG};

#[cfg(target_os = "freebsd")]
use libc::{__error, c_int, c_void, procctl, P_PID, PROC_PDEATHSIG_CTL};

/// Retrieve the last error number of a system or library call.
pub fn errno() -> i32 {
    #[cfg(target_os = "linux")]
    {
        unsafe { *__errno_location() }
    }

    #[cfg(target_os = "freebsd")]
    {
        unsafe { *__error() }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        libc::ENOSYS
    }
}

/// Signal process when parent exits.
pub fn signal(sig: i32) -> Result<(), i32> {
    #[cfg(target_os = "linux")]
    {
        match unsafe { prctl(PR_SET_PDEATHSIG, sig, 0, 0, 0) } {
            0 => Ok(()),
            _ => Err(errno()),
        }
    }

    #[cfg(target_os = "freebsd")]
    {
        let mut data: c_int = sig;
        let p_data = &mut data as *mut c_int as *mut c_void;

        match unsafe { procctl(P_PID, 0, PROC_PDEATHSIG_CTL, p_data) } {
            0 => Ok(()),
            _ => Err(errno()),
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        Err(errno())
    }
}

/// Replace the current process image with the new process image specified by path and
/// arguments.
pub fn execvp(argv: Vec<CString>) -> i32 {
    let mut p_argv: Vec<_> = argv.iter().map(|arg| arg.as_ptr()).collect();

    p_argv.push(std::ptr::null());

    let p: *const *const c_char = p_argv.as_ptr();

    unsafe {
        libc::execvp(p_argv[0], p);
    };

    errno()
}
