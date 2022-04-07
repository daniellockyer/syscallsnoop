use libc::{c_char, dirent, size_t, ssize_t};

macro_rules! syscall_hook {
    ($name: ident, $hook_name: ident, $path: ident, $rettype: ident, ($($y:ident: $ytype:ty),+)) => {
        redhook::hook! {
            unsafe fn $name($($y: $ytype),+) -> $rettype => $hook_name {
                if let Ok(printvar) = std::str::from_utf8(std::ffi::CStr::from_ptr($path).to_bytes()) {
                    println!("{}({:?})", stringify!($name), printvar);
                } else {
                    println!("{}(..) {:?}", stringify!($name), $path);
                }

                redhook::real!($name)($($y),+)
            }
        }
    };
}

syscall_hook!(
    open,
    openhook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char, bufsiz: size_t)
);

syscall_hook!(
    stat,
    stathook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char)
);

syscall_hook!(
    stat64,
    stat64hook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char)
);

syscall_hook!(
    lstat,
    lstathook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char)
);

syscall_hook!(
    lstat64,
    lstat64hook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char)
);

syscall_hook!(
    readlink,
    readlinkhook,
    path,
    ssize_t,
    (path: *const c_char, buf: *mut c_char, bufsiz: size_t)
);

syscall_hook!(
    opendir,
    opendirhook,
    dirname,
    dirent,
    (dirname: *const c_char)
);
