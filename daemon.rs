use ::libc;
extern "C" {
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: off_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub static mut daemon_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pid_file_handle: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn kill_daemon() {
    close(pid_file_handle);
    remove(b"/tmp/dogcom.pid\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn signal_handler(mut signal: libc::c_int) {
    match signal {
        15 => {
            kill_daemon();
            exit(0 as libc::c_int);
        }
        1 | 2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn daemonise() {
    let mut pid: pid_t = 0;
    let mut sig_action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sigset: sigset_t = __sigset_t { __val: [0; 16] };
    pid = fork();
    if pid < 0 as libc::c_int {
        printf(b"Fork failed!\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else if pid > 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    if setsid() < 0 as libc::c_int {
        printf(b"Setsid failed!\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    sigemptyset(&mut sigset);
    sigaddset(&mut sigset, 17 as libc::c_int);
    sigaddset(&mut sigset, 20 as libc::c_int);
    sigaddset(&mut sigset, 22 as libc::c_int);
    sigaddset(&mut sigset, 21 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut sigset, 0 as *mut sigset_t);
    sig_action
        .__sigaction_handler
        .sa_handler = Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ());
    sigemptyset(&mut sig_action.sa_mask);
    sig_action.sa_flags = 0 as libc::c_int;
    sigaction(1 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    pid = fork();
    if pid < 0 as libc::c_int {
        printf(b"Fork failed!\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else if pid > 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    chdir(b"/tmp/\0" as *const u8 as *const libc::c_char);
    umask(0o27 as libc::c_int as __mode_t);
    close(0 as libc::c_int);
    close(1 as libc::c_int);
    close(2 as libc::c_int);
    open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0o1 as libc::c_int);
    open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0o2 as libc::c_int);
    pid_file_handle = open(
        b"/tmp/dogcom.pid\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int | 0o100 as libc::c_int,
        0o600 as libc::c_int,
    );
    if pid_file_handle < 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    if lockf(pid_file_handle, 2 as libc::c_int, 0 as libc::c_int as off_t)
        < 0 as libc::c_int
    {
        exit(1 as libc::c_int);
    }
    let mut spid: [libc::c_char; 10] = [0; 10];
    sprintf(spid.as_mut_ptr(), b"%d\n\0" as *const u8 as *const libc::c_char, getpid());
    write(
        pid_file_handle,
        spid.as_mut_ptr() as *const libc::c_void,
        strlen(spid.as_mut_ptr()),
    );
}
