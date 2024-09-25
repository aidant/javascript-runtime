#[macro_export]
macro_rules! stdio_to_logcat {
    () => {
        use android_log_sys::{LogPriority, __android_log_write};
        use std::{
            ffi::{CStr, CString},
            fs::File,
            io::{BufRead, BufReader},
            os::{
                fd::{FromRawFd, RawFd},
                raw::c_int,
            },
            thread::Builder,
        };

        let mut stdio: [RawFd; 2] = Default::default();
        unsafe {
            libc::pipe(stdio.as_mut_ptr());
            libc::dup2(stdio[1], libc::STDOUT_FILENO);
            libc::dup2(stdio[1], libc::STDERR_FILENO);
        }

        Builder::new()
            .name("stdio_to_logcat".into())
            .spawn(move || {
                let tag = CStr::from_bytes_with_nul(b"javascript_runtime\0").unwrap();
                let file = unsafe { File::from_raw_fd(stdio[0]) };
                let mut reader = BufReader::new(file);
                let mut buffer = String::new();
                loop {
                    buffer.clear();
                    if let Ok(len) = reader.read_line(&mut buffer) {
                        if len == 0 {
                            break;
                        } else if let Ok(msg) = CString::new(buffer.clone()) {
                            unsafe {
                                __android_log_write(
                                    LogPriority::INFO as c_int,
                                    tag.as_ptr(),
                                    msg.as_ptr(),
                                );
                            }
                        }
                    }
                }
            })
            .map_err(anyhow::Error::msg)?;
    };
}
