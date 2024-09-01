#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::c2rust_out::*;
extern "C" {
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dogcom(try_times: libc::c_int) -> libc::c_int;
    static mut drcom_config: config;
    static mut verbose_flag: libc::c_int;
    static mut logging_flag: libc::c_int;
    static mut eapol_flag: libc::c_int;
    static mut eternal_flag: libc::c_int;
    static mut log_path: *mut libc::c_char;
    static mut mode: [libc::c_char; 10];
    static mut bind_ip: [libc::c_char; 20];
    fn config_parse(filepath: *mut libc::c_char) -> libc::c_int;
    fn daemonise();
    static mut daemon_flag: libc::c_int;
    fn getall_ifs(ifs: *mut iflist_t, cnt: *mut libc::c_int) -> libc::c_int;
    fn eaplogin(uname: *const libc::c_char, pwd: *const libc::c_char) -> libc::c_int;
    fn setifname(ifname: *const libc::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub server: [libc::c_char; 20],
    pub username: [libc::c_char; 36],
    pub password: [libc::c_char; 20],
    pub CONTROLCHECKSTATUS: libc::c_uchar,
    pub ADAPTERNUM: libc::c_uchar,
    pub host_ip: [libc::c_char; 20],
    pub IPDOG: libc::c_uchar,
    pub host_name: [libc::c_char; 20],
    pub PRIMARY_DNS: [libc::c_char; 20],
    pub dhcp_server: [libc::c_char; 20],
    pub AUTH_VERSION: [libc::c_uchar; 2],
    pub mac: [libc::c_uchar; 6],
    pub host_os: [libc::c_char; 20],
    pub KEEP_ALIVE_VERSION: [libc::c_uchar; 2],
    pub ror_version: libc::c_int,
    pub keepalive1_mod: libc::c_int,
    pub pppoe_flag: libc::c_uchar,
    pub keep_alive2_flag: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iflist_t {
    pub name: [libc::c_char; 16],
}
static mut default_bind_ip: [libc::c_char; 20] = unsafe {
    *::core::mem::transmute::<
        &[u8; 20],
        &[libc::c_char; 20],
    >(b"0.0.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0")
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc == 1 as libc::c_int {
        print_help(1 as libc::c_int);
    }
    let mut file_path: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        static mut long_options: [option; 10] = [
            {
                let mut init = option {
                    name: b"mode\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'm' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"conf\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'c' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"bindip\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'b' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"log\0" as *const u8 as *const libc::c_char,
                    has_arg: 1 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'l' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"daemon\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'd' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"802.1x\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'x' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"eternal\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'e' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"verbose\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'v' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"help\0" as *const u8 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'h' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: 0 as *const libc::c_char,
                    has_arg: 0 as libc::c_int,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 0 as libc::c_int,
                };
                init
            },
        ];
        let mut c: libc::c_int = 0;
        let mut option_index: libc::c_int = 0 as libc::c_int;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"m:c:b:l:dxevh\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            109 => {
                if strcmp(optarg, b"dhcp\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    strcpy(mode.as_mut_ptr(), optarg);
                } else if strcmp(optarg, b"pppoe\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    strcpy(mode.as_mut_ptr(), optarg);
                } else {
                    printf(b"unknown mode\n\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            99 => {
                if !mode.as_mut_ptr().is_null() {
                    let mut path_c: [libc::c_char; 4096] = [0; 4096];
                    realpath(optarg, path_c.as_mut_ptr());
                    file_path = strdup(path_c.as_mut_ptr());
                }
            }
            98 => {
                strcpy(bind_ip.as_mut_ptr(), optarg);
            }
            108 => {
                if !mode.as_mut_ptr().is_null() {
                    let mut path_l: [libc::c_char; 4096] = [0; 4096];
                    realpath(optarg, path_l.as_mut_ptr());
                    log_path = strdup(path_l.as_mut_ptr());
                    logging_flag = 1 as libc::c_int;
                }
            }
            100 => {
                daemon_flag = 1 as libc::c_int;
            }
            120 => {
                eapol_flag = 1 as libc::c_int;
            }
            101 => {
                eternal_flag = 1 as libc::c_int;
            }
            118 => {
                verbose_flag = 1 as libc::c_int;
            }
            104 => {
                print_help(0 as libc::c_int);
            }
            63 => {
                print_help(1 as libc::c_int);
            }
            _ => {}
        }
    }
    if !mode.as_mut_ptr().is_null() && !file_path.is_null() {
        if daemon_flag != 0 {
            daemonise();
        }
        if config_parse(file_path) == 0 {
            if eapol_flag != 0 {
                if 0 as libc::c_int != try_smart_eaplogin() {
                    printf(
                        b"Can't finish 802.1x authorization!\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            }
            if strlen(bind_ip.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong {
                memcpy(
                    bind_ip.as_mut_ptr() as *mut libc::c_void,
                    default_bind_ip.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                );
            }
            dogcom(5 as libc::c_int);
        } else {
            return 1 as libc::c_int
        }
    } else {
        printf(b"Need more options!\n\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_help(mut exval: libc::c_int) {
    printf(
        b"\nDrcom-generic implementation in C.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"Version: %s\n\n\0" as *const u8 as *const libc::c_char,
        b"1.6.2\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Usage:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\tdogcom -m <dhcp/pppoe> -c <FILEPATH> [options <argument>]...\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"\t--mode <dhcp/pppoe>, -m <dhcp/pppoe>  set your dogcom mode \n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--conf <FILEPATH>, -c <FILEPATH>      import configuration file\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"\t--bindip <IPADDR>, -b <IPADDR>        bind your ip address(default is 0.0.0.0)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"\t--log <LOGPATH>, -l <LOGPATH>         specify log file\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--daemon, -d                          set daemon flag\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--802.1x, -x                          enable 802.1x\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--eternal, -e                         set eternal flag\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--verbose, -v                         set verbose flag\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\t--help, -h                            display this help\n\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(exval);
}
#[no_mangle]
pub unsafe extern "C" fn try_smart_eaplogin() -> libc::c_int {
    let mut ifcnt: libc::c_int = 64 as libc::c_int;
    let mut ifs: [iflist_t; 64] = [iflist_t { name: [0; 16] }; 64];
    if 0 as libc::c_int > getall_ifs(ifs.as_mut_ptr(), &mut ifcnt) {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ifcnt {
        setifname((ifs[i as usize].name).as_mut_ptr());
        if 0 as libc::c_int
            == eaplogin(
                (drcom_config.username).as_mut_ptr(),
                (drcom_config.password).as_mut_ptr(),
            )
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
