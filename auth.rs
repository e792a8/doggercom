use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    static mut drcom_config: config;
    static mut verbose_flag: libc::c_int;
    static mut logging_flag: libc::c_int;
    static mut eternal_flag: libc::c_int;
    static mut log_path: *mut libc::c_char;
    static mut mode: [libc::c_char; 10];
    static mut bind_ip: [libc::c_char; 20];
    fn keepalive_1(
        sockfd: libc::c_int,
        addr: sockaddr_in,
        seed: *mut libc::c_uchar,
        auth_information: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn keepalive_2(
        sockfd: libc::c_int,
        addr: sockaddr_in,
        keepalive_counter: *mut libc::c_int,
        first: *mut libc::c_int,
        encrypt_type: *mut libc::c_int,
    ) -> libc::c_int;
    fn gen_crc(
        seed: *mut libc::c_uchar,
        encrypt_type: libc::c_int,
        crc: *mut libc::c_uchar,
    );
    fn MD5(data: *const libc::c_void, size: libc::c_ulong, result: *mut libc::c_uchar);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type socklen_t = __socklen_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed = 262;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed = 143;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_L2TP: C2RustUnnamed = 115;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MUST_USE_DHCP: C2RustUnnamed_0 = 23;
pub const NOT_ON_THIS_IP_MAC: C2RustUnnamed_0 = 22;
pub const UPDATE_CLIENT: C2RustUnnamed_0 = 21;
pub const TOO_MUCH_IP: C2RustUnnamed_0 = 20;
pub const NOT_ON_THIS_MAC: C2RustUnnamed_0 = 11;
pub const NOT_ON_THIS_IP: C2RustUnnamed_0 = 7;
pub const FREEZE_UP: C2RustUnnamed_0 = 5;
pub const NOT_ENOUGH: C2RustUnnamed_0 = 4;
pub const WRONG_PASS: C2RustUnnamed_0 = 3;
pub const SERVER_BUSY: C2RustUnnamed_0 = 2;
pub const CHECK_MAC: C2RustUnnamed_0 = 1;
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
#[no_mangle]
pub unsafe extern "C" fn dhcp_challenge(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut seed: *mut libc::c_uchar,
) -> libc::c_int {
    let mut challenge_packet: [libc::c_uchar; 20] = [0; 20];
    let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
    memset(
        challenge_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        20 as libc::c_int as libc::c_ulong,
    );
    challenge_packet[0 as libc::c_int as usize] = 0x1 as libc::c_int as libc::c_uchar;
    challenge_packet[1 as libc::c_int as usize] = 0x2 as libc::c_int as libc::c_uchar;
    challenge_packet[2 as libc::c_int
        as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
    challenge_packet[3 as libc::c_int
        as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
    challenge_packet[4 as libc::c_int
        as usize] = drcom_config.AUTH_VERSION[0 as libc::c_int as usize];
    sendto(
        sockfd,
        challenge_packet.as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[Challenge sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            challenge_packet.as_mut_ptr(),
            20 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Challenge sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            challenge_packet.as_mut_ptr(),
            20 as libc::c_int,
        );
    }
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) < 0 as libc::c_int as libc::c_long
    {
        perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if verbose_flag != 0 {
        print_packet(
            b"[Challenge recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            76 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Challenge recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            76 as libc::c_int,
        );
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int != 0x2 as libc::c_int {
        printf(
            b"Bad challenge response received.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    memcpy(
        seed as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_login(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut seed: *mut libc::c_uchar,
    mut auth_information: *mut libc::c_uchar,
    mut try_JLUversion: libc::c_int,
) -> libc::c_int {
    let mut login_packet_size: libc::c_uint = 0;
    let mut length_padding: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut JLU_padding: libc::c_int = 0 as libc::c_int;
    if strlen((drcom_config.password).as_mut_ptr()) > 8 as libc::c_int as libc::c_ulong {
        length_padding = (strlen((drcom_config.password).as_mut_ptr()))
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                length_padding.wrapping_rem(2 as libc::c_int as libc::c_uint)
                    as libc::c_ulong,
            ) as libc::c_uint;
        if try_JLUversion != 0 {
            printf(b"Start JLU mode.\n\0" as *const u8 as *const libc::c_char);
            if logging_flag != 0 {
                logging(
                    b"Start JLU mode.\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_uchar,
                    0 as libc::c_int,
                );
            }
            if strlen((drcom_config.password).as_mut_ptr())
                != 16 as libc::c_int as libc::c_ulong
            {
                JLU_padding = (strlen((drcom_config.password).as_mut_ptr()))
                    .wrapping_div(4 as libc::c_int as libc::c_ulong) as libc::c_int;
            }
            length_padding = (28 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (strlen((drcom_config.password).as_mut_ptr()))
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(JLU_padding as libc::c_ulong) as libc::c_uint;
        }
    }
    if drcom_config.ror_version != 0 {
        login_packet_size = (338 as libc::c_int as libc::c_uint)
            .wrapping_add(length_padding);
    } else {
        login_packet_size = 330 as libc::c_int as libc::c_uint;
    }
    let vla = login_packet_size as usize;
    let mut login_packet: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
    let mut MD5A: [libc::c_uchar; 16] = [0; 16];
    let mut MACxorMD5A: [libc::c_uchar; 6] = [0; 6];
    let mut MD5B: [libc::c_uchar; 16] = [0; 16];
    let mut checksum1: [libc::c_uchar; 8] = [0; 8];
    let mut checksum2: [libc::c_uchar; 4] = [0; 4];
    memset(
        login_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        login_packet_size as libc::c_ulong,
    );
    memset(
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        100 as libc::c_int as libc::c_ulong,
    );
    *login_packet
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 0x3 as libc::c_int as libc::c_uchar;
    *login_packet
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    *login_packet
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    *login_packet
        .as_mut_ptr()
        .offset(
            3 as libc::c_int as isize,
        ) = (strlen((drcom_config.username).as_mut_ptr()))
        .wrapping_add(20 as libc::c_int as libc::c_ulong) as libc::c_uchar;
    let mut MD5A_len: libc::c_int = (6 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((drcom_config.password).as_mut_ptr())) as libc::c_int;
    let vla_0 = MD5A_len as usize;
    let mut MD5A_str: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla_0);
    *MD5A_str
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 0x3 as libc::c_int as libc::c_uchar;
    *MD5A_str
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    memcpy(
        MD5A_str.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_void,
        seed as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        MD5A_str.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut libc::c_void,
        (drcom_config.password).as_mut_ptr() as *const libc::c_void,
        strlen((drcom_config.password).as_mut_ptr()),
    );
    MD5(
        MD5A_str.as_mut_ptr() as *const libc::c_void,
        MD5A_len as libc::c_ulong,
        MD5A.as_mut_ptr(),
    );
    memcpy(
        login_packet.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
        MD5A.as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(20 as libc::c_int as isize)
            as *mut libc::c_void,
        (drcom_config.username).as_mut_ptr() as *const libc::c_void,
        strlen((drcom_config.username).as_mut_ptr()),
    );
    memcpy(
        login_packet.as_mut_ptr().offset(56 as libc::c_int as isize)
            as *mut libc::c_void,
        &mut drcom_config.CONTROLCHECKSTATUS as *mut libc::c_uchar
            as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(57 as libc::c_int as isize)
            as *mut libc::c_void,
        &mut drcom_config.ADAPTERNUM as *mut libc::c_uchar as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut mac: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        sum = (MD5A[i as usize] as libc::c_int as libc::c_ulong)
            .wrapping_add(sum.wrapping_mul(256 as libc::c_int as libc::c_ulong));
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 6 as libc::c_int {
        mac = (drcom_config.mac[i_0 as usize] as libc::c_int as libc::c_ulong)
            .wrapping_add(mac.wrapping_mul(256 as libc::c_int as libc::c_ulong));
        i_0 += 1;
        i_0;
    }
    sum ^= mac;
    let mut i_1: libc::c_int = 6 as libc::c_int;
    while i_1 > 0 as libc::c_int {
        MACxorMD5A[(i_1 - 1 as libc::c_int)
            as usize] = sum.wrapping_rem(256 as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        sum = (sum as libc::c_ulong).wrapping_div(256 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        i_1 -= 1;
        i_1;
    }
    memcpy(
        login_packet.as_mut_ptr().offset(58 as libc::c_int as isize)
            as *mut libc::c_void,
        MACxorMD5A.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 6]>() as libc::c_ulong,
    );
    let mut MD5B_len: libc::c_int = (9 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((drcom_config.password).as_mut_ptr())) as libc::c_int;
    let vla_1 = MD5B_len as usize;
    let mut MD5B_str: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla_1);
    memset(
        MD5B_str.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        MD5B_len as libc::c_ulong,
    );
    *MD5B_str
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    memcpy(
        MD5B_str.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (drcom_config.password).as_mut_ptr() as *const libc::c_void,
        strlen((drcom_config.password).as_mut_ptr()),
    );
    memcpy(
        MD5B_str
            .as_mut_ptr()
            .offset(strlen((drcom_config.password).as_mut_ptr()) as isize)
            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
        seed as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    MD5(
        MD5B_str.as_mut_ptr() as *const libc::c_void,
        MD5B_len as libc::c_ulong,
        MD5B.as_mut_ptr(),
    );
    memcpy(
        login_packet.as_mut_ptr().offset(64 as libc::c_int as isize)
            as *mut libc::c_void,
        MD5B.as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    *login_packet
        .as_mut_ptr()
        .offset(80 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    let mut host_ip: [libc::c_uchar; 4] = [0; 4];
    sscanf(
        (drcom_config.host_ip).as_mut_ptr(),
        b"%hhd.%hhd.%hhd.%hhd\0" as *const u8 as *const libc::c_char,
        &mut *host_ip.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *host_ip.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *host_ip.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *host_ip.as_mut_ptr().offset(3 as libc::c_int as isize)
            as *mut libc::c_uchar,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(81 as libc::c_int as isize)
            as *mut libc::c_void,
        host_ip.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    let mut checksum1_str: [libc::c_uchar; 101] = [0; 101];
    let mut checksum1_tmp: [libc::c_uchar; 4] = [
        0x14 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x7 as libc::c_int as libc::c_uchar,
        0xb as libc::c_int as libc::c_uchar,
    ];
    memcpy(
        checksum1_str.as_mut_ptr() as *mut libc::c_void,
        login_packet.as_mut_ptr() as *const libc::c_void,
        97 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        checksum1_str.as_mut_ptr().offset(97 as libc::c_int as isize)
            as *mut libc::c_void,
        checksum1_tmp.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    MD5(
        checksum1_str.as_mut_ptr() as *const libc::c_void,
        101 as libc::c_int as libc::c_ulong,
        checksum1.as_mut_ptr(),
    );
    memcpy(
        login_packet.as_mut_ptr().offset(97 as libc::c_int as isize)
            as *mut libc::c_void,
        checksum1.as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(105 as libc::c_int as isize)
            as *mut libc::c_void,
        &mut drcom_config.IPDOG as *mut libc::c_uchar as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(110 as libc::c_int as isize)
            as *mut libc::c_void,
        &mut drcom_config.host_name as *mut [libc::c_char; 20] as *const libc::c_void,
        strlen((drcom_config.host_name).as_mut_ptr()),
    );
    let mut PRIMARY_DNS: [libc::c_uchar; 4] = [0; 4];
    sscanf(
        (drcom_config.PRIMARY_DNS).as_mut_ptr(),
        b"%hhd.%hhd.%hhd.%hhd\0" as *const u8 as *const libc::c_char,
        &mut *PRIMARY_DNS.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *PRIMARY_DNS.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *PRIMARY_DNS.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *PRIMARY_DNS.as_mut_ptr().offset(3 as libc::c_int as isize)
            as *mut libc::c_uchar,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(142 as libc::c_int as isize)
            as *mut libc::c_void,
        PRIMARY_DNS.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    let mut dhcp_server: [libc::c_uchar; 4] = [0; 4];
    sscanf(
        (drcom_config.dhcp_server).as_mut_ptr(),
        b"%hhd.%hhd.%hhd.%hhd\0" as *const u8 as *const libc::c_char,
        &mut *dhcp_server.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *dhcp_server.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *dhcp_server.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut libc::c_uchar,
        &mut *dhcp_server.as_mut_ptr().offset(3 as libc::c_int as isize)
            as *mut libc::c_uchar,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(146 as libc::c_int as isize)
            as *mut libc::c_void,
        dhcp_server.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    let mut OSVersionInfoSize: [libc::c_uchar; 4] = [
        0x94 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
    ];
    let mut OSMajor: [libc::c_uchar; 4] = [0x5 as libc::c_int as libc::c_uchar, 0, 0, 0];
    let mut OSMinor: [libc::c_uchar; 4] = [0x1 as libc::c_int as libc::c_uchar, 0, 0, 0];
    let mut OSBuild: [libc::c_uchar; 4] = [
        0x28 as libc::c_int as libc::c_uchar,
        0xa as libc::c_int as libc::c_uchar,
        0,
        0,
    ];
    let mut PlatformID: [libc::c_uchar; 4] = [
        0x2 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
    ];
    if try_JLUversion != 0 {
        OSVersionInfoSize[0 as libc::c_int
            as usize] = 0x94 as libc::c_int as libc::c_uchar;
        OSMajor[0 as libc::c_int as usize] = 0x6 as libc::c_int as libc::c_uchar;
        OSMinor[0 as libc::c_int as usize] = 0x2 as libc::c_int as libc::c_uchar;
        OSBuild[0 as libc::c_int as usize] = 0xf0 as libc::c_int as libc::c_uchar;
        OSBuild[1 as libc::c_int as usize] = 0x23 as libc::c_int as libc::c_uchar;
        PlatformID[0 as libc::c_int as usize] = 0x2 as libc::c_int as libc::c_uchar;
        let mut ServicePack: [libc::c_uchar; 40] = [
            0x33 as libc::c_int as libc::c_uchar,
            0x64 as libc::c_int as libc::c_uchar,
            0x63 as libc::c_int as libc::c_uchar,
            0x37 as libc::c_int as libc::c_uchar,
            0x39 as libc::c_int as libc::c_uchar,
            0x66 as libc::c_int as libc::c_uchar,
            0x35 as libc::c_int as libc::c_uchar,
            0x32 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
            0x32 as libc::c_int as libc::c_uchar,
            0x65 as libc::c_int as libc::c_uchar,
            0x38 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
            0x37 as libc::c_int as libc::c_uchar,
            0x30 as libc::c_int as libc::c_uchar,
            0x61 as libc::c_int as libc::c_uchar,
            0x63 as libc::c_int as libc::c_uchar,
            0x66 as libc::c_int as libc::c_uchar,
            0x61 as libc::c_int as libc::c_uchar,
            0x39 as libc::c_int as libc::c_uchar,
            0x65 as libc::c_int as libc::c_uchar,
            0x63 as libc::c_int as libc::c_uchar,
            0x39 as libc::c_int as libc::c_uchar,
            0x35 as libc::c_int as libc::c_uchar,
            0x66 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
            0x64 as libc::c_int as libc::c_uchar,
            0x37 as libc::c_int as libc::c_uchar,
            0x34 as libc::c_int as libc::c_uchar,
            0x39 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
            0x36 as libc::c_int as libc::c_uchar,
            0x35 as libc::c_int as libc::c_uchar,
            0x34 as libc::c_int as libc::c_uchar,
            0x32 as libc::c_int as libc::c_uchar,
            0x62 as libc::c_int as libc::c_uchar,
            0x65 as libc::c_int as libc::c_uchar,
            0x37 as libc::c_int as libc::c_uchar,
            0x62 as libc::c_int as libc::c_uchar,
            0x31 as libc::c_int as libc::c_uchar,
        ];
        let mut hostname: [libc::c_uchar; 9] = [
            0x44 as libc::c_int as libc::c_uchar,
            0x72 as libc::c_int as libc::c_uchar,
            0x43 as libc::c_int as libc::c_uchar,
            0x4f as libc::c_int as libc::c_uchar,
            0x4d as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0xcf as libc::c_int as libc::c_uchar,
            0x7 as libc::c_int as libc::c_uchar,
            0x68 as libc::c_int as libc::c_uchar,
        ];
        memcpy(
            login_packet.as_mut_ptr().offset(182 as libc::c_int as isize)
                as *mut libc::c_void,
            hostname.as_mut_ptr() as *const libc::c_void,
            9 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            login_packet.as_mut_ptr().offset(246 as libc::c_int as isize)
                as *mut libc::c_void,
            ServicePack.as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
    }
    memcpy(
        login_packet.as_mut_ptr().offset(162 as libc::c_int as isize)
            as *mut libc::c_void,
        OSVersionInfoSize.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(166 as libc::c_int as isize)
            as *mut libc::c_void,
        OSMajor.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(170 as libc::c_int as isize)
            as *mut libc::c_void,
        OSMinor.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(174 as libc::c_int as isize)
            as *mut libc::c_void,
        OSBuild.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(178 as libc::c_int as isize)
            as *mut libc::c_void,
        PlatformID.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    if try_JLUversion == 0 {
        memcpy(
            login_packet.as_mut_ptr().offset(182 as libc::c_int as isize)
                as *mut libc::c_void,
            &mut drcom_config.host_os as *mut [libc::c_char; 20] as *const libc::c_void,
            strlen((drcom_config.host_os).as_mut_ptr()),
        );
    }
    memcpy(
        login_packet.as_mut_ptr().offset(310 as libc::c_int as isize)
            as *mut libc::c_void,
        (drcom_config.AUTH_VERSION).as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    let mut counter: libc::c_int = 312 as libc::c_int;
    let mut ror_padding: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if strlen((drcom_config.password).as_mut_ptr()) <= 8 as libc::c_int as libc::c_ulong
    {
        ror_padding = (8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(strlen((drcom_config.password).as_mut_ptr())) as libc::c_uint;
    } else {
        if (strlen((drcom_config.password).as_mut_ptr()))
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_rem(2 as libc::c_int as libc::c_ulong) != 0
        {
            ror_padding = 1 as libc::c_int as libc::c_uint;
        }
        if try_JLUversion != 0 {
            ror_padding = JLU_padding as libc::c_uint;
        }
    }
    if drcom_config.ror_version != 0 {
        MD5(
            MD5A_str.as_mut_ptr() as *const libc::c_void,
            MD5A_len as libc::c_ulong,
            MD5A.as_mut_ptr(),
        );
        *login_packet
            .as_mut_ptr()
            .offset(
                (counter + 1 as libc::c_int) as isize,
            ) = strlen((drcom_config.password).as_mut_ptr()) as libc::c_uchar;
        counter += 2 as libc::c_int;
        let mut i_2: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while (i_2 as libc::c_ulong) < strlen((drcom_config.password).as_mut_ptr()) {
            x = MD5A[i_2 as usize] as libc::c_int
                ^ drcom_config.password[i_2 as usize] as libc::c_int;
            *login_packet
                .as_mut_ptr()
                .offset(
                    (counter + i_2) as isize,
                ) = ((x << 3 as libc::c_int & 0xff as libc::c_int)
                + (x >> 5 as libc::c_int)) as libc::c_uchar;
            i_2 += 1;
            i_2;
        }
        counter = (counter as libc::c_ulong)
            .wrapping_add(strlen((drcom_config.password).as_mut_ptr())) as libc::c_int
            as libc::c_int;
    } else {
        ror_padding = 2 as libc::c_int as libc::c_uint;
    }
    *login_packet
        .as_mut_ptr()
        .offset(counter as isize) = 0x2 as libc::c_int as libc::c_uchar;
    *login_packet
        .as_mut_ptr()
        .offset(
            (counter + 1 as libc::c_int) as isize,
        ) = 0xc as libc::c_int as libc::c_uchar;
    let vla_2 = (counter + 18 as libc::c_int) as usize;
    let mut checksum2_str: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla_2);
    memset(
        checksum2_str.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (counter + 18 as libc::c_int) as libc::c_ulong,
    );
    let mut checksum2_tmp: [libc::c_uchar; 6] = [
        0x1 as libc::c_int as libc::c_uchar,
        0x26 as libc::c_int as libc::c_uchar,
        0x7 as libc::c_int as libc::c_uchar,
        0x11 as libc::c_int as libc::c_uchar,
        0,
        0,
    ];
    memcpy(
        checksum2_str.as_mut_ptr() as *mut libc::c_void,
        login_packet.as_mut_ptr() as *const libc::c_void,
        (counter + 2 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        checksum2_str
            .as_mut_ptr()
            .offset(counter as isize)
            .offset(2 as libc::c_int as isize) as *mut libc::c_void,
        checksum2_tmp.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        checksum2_str
            .as_mut_ptr()
            .offset(counter as isize)
            .offset(8 as libc::c_int as isize) as *mut libc::c_void,
        (drcom_config.mac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    sum = 1234 as libc::c_int as uint64_t;
    let mut ret: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < counter + 14 as libc::c_int {
        ret = 0 as libc::c_int as uint64_t;
        let mut j: libc::c_int = 4 as libc::c_int;
        while j > 0 as libc::c_int {
            ret = ret
                .wrapping_mul(256 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    *checksum2_str
                        .as_mut_ptr()
                        .offset((i_3 + j - 1 as libc::c_int) as isize) as libc::c_int
                        as libc::c_ulong,
                );
            j -= 1;
            j;
        }
        sum ^= ret;
        i_3 += 4 as libc::c_int;
    }
    sum = (1968 as libc::c_int as libc::c_ulong).wrapping_mul(sum)
        & 0xffffffff as libc::c_uint as libc::c_ulong;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < 4 as libc::c_int {
        checksum2[j_0
            as usize] = (sum >> j_0 * 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        j_0 += 1;
        j_0;
    }
    memcpy(
        login_packet
            .as_mut_ptr()
            .offset(counter as isize)
            .offset(2 as libc::c_int as isize) as *mut libc::c_void,
        checksum2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet
            .as_mut_ptr()
            .offset(counter as isize)
            .offset(8 as libc::c_int as isize) as *mut libc::c_void,
        (drcom_config.mac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    *login_packet
        .as_mut_ptr()
        .offset(
            (counter as libc::c_uint)
                .wrapping_add(ror_padding)
                .wrapping_add(14 as libc::c_int as libc::c_uint) as isize,
        ) = 0xe9 as libc::c_int as libc::c_uchar;
    *login_packet
        .as_mut_ptr()
        .offset(
            (counter as libc::c_uint)
                .wrapping_add(ror_padding)
                .wrapping_add(15 as libc::c_int as libc::c_uint) as isize,
        ) = 0x13 as libc::c_int as libc::c_uchar;
    if try_JLUversion != 0 {
        *login_packet
            .as_mut_ptr()
            .offset(
                (counter as libc::c_uint)
                    .wrapping_add(ror_padding)
                    .wrapping_add(14 as libc::c_int as libc::c_uint) as isize,
            ) = 0x60 as libc::c_int as libc::c_uchar;
        *login_packet
            .as_mut_ptr()
            .offset(
                (counter as libc::c_uint)
                    .wrapping_add(ror_padding)
                    .wrapping_add(15 as libc::c_int as libc::c_uint) as isize,
            ) = 0xa2 as libc::c_int as libc::c_uchar;
    }
    sendto(
        sockfd,
        login_packet.as_mut_ptr() as *const libc::c_void,
        (vla * ::core::mem::size_of::<libc::c_uchar>()) as libc::c_ulong,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[Login sent] \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            login_packet.as_mut_ptr(),
            (vla * ::core::mem::size_of::<libc::c_uchar>()) as libc::c_ulong
                as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Login sent] \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            login_packet.as_mut_ptr(),
            (vla * ::core::mem::size_of::<libc::c_uchar>()) as libc::c_ulong
                as libc::c_int,
        );
    }
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) < 0 as libc::c_int as libc::c_long
    {
        perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int != 0x4 as libc::c_int {
        if verbose_flag != 0 {
            print_packet(
                b"[login recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                100 as libc::c_int,
            );
        }
        printf(b"<<< Login failed >>>\n\0" as *const u8 as *const libc::c_char);
        if logging_flag != 0 {
            logging(
                b"[login recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                100 as libc::c_int,
            );
            logging(
                b"<<< Login failed >>>\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int,
            );
        }
        let mut err_msg: [libc::c_char; 256] = [0; 256];
        if recv_packet[0 as libc::c_int as usize] as libc::c_int == 0x5 as libc::c_int {
            match recv_packet[4 as libc::c_int as usize] as libc::c_int {
                1 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] Someone is using this account with wired.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                2 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] The server is busy, please log back in again.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                3 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] Account and password not match.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                4 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] The cumulative time or traffic for this account has exceeded the limit.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                5 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] This account is suspended.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                7 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] IP address does not match, this account can only be used in the specified IP address.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                11 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] MAC address does not match, this account can only be used in the specified IP and MAC address.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                20 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] This account has too many IP addresses.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                21 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] The client version is incorrect.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                22 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] This account can only be used on specified MAC and IP address.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                23 => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] Your PC set up a static IP, please change to DHCP, and then re-login.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    strcpy(
                        err_msg.as_mut_ptr(),
                        b"[Tips] Unknown error number.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, err_msg.as_mut_ptr());
            if logging_flag != 0 {
                logging(err_msg.as_mut_ptr(), 0 as *mut libc::c_uchar, 0 as libc::c_int);
            }
        }
        return 1 as libc::c_int;
    } else {
        if verbose_flag != 0 {
            print_packet(
                b"[login recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                100 as libc::c_int,
            );
        }
        printf(b"<<< Logged in >>>\n\0" as *const u8 as *const libc::c_char);
        if logging_flag != 0 {
            logging(
                b"[login recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                100 as libc::c_int,
            );
            logging(
                b"<<< Logged in >>>\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int,
            );
        }
    }
    memcpy(
        auth_information as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(23 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) >= 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pppoe_challenge(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut pppoe_counter: *mut libc::c_int,
    mut seed: *mut libc::c_uchar,
    mut sip: *mut libc::c_uchar,
    mut encrypt_mode: *mut libc::c_int,
) -> libc::c_int {
    let mut challenge_packet: [libc::c_uchar; 8] = [0; 8];
    let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
    memset(
        challenge_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    let mut challenge_tmp: [libc::c_uchar; 5] = [
        0x7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x8 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
    ];
    memcpy(
        challenge_packet.as_mut_ptr() as *mut libc::c_void,
        challenge_tmp.as_mut_ptr() as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    );
    challenge_packet[1 as libc::c_int
        as usize] = (*pppoe_counter % 0xff as libc::c_int) as libc::c_uchar;
    *pppoe_counter += 1;
    *pppoe_counter;
    sendto(
        sockfd,
        challenge_packet.as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[Challenge sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            challenge_packet.as_mut_ptr(),
            8 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Challenge sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            challenge_packet.as_mut_ptr(),
            8 as libc::c_int,
        );
    }
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) < 0 as libc::c_int as libc::c_long
    {
        perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if verbose_flag != 0 {
        print_packet(
            b"[Challenge recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            32 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Challenge recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            32 as libc::c_int,
        );
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int != 0x7 as libc::c_int {
        printf(
            b"Bad challenge response received.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if recv_packet[5 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        *encrypt_mode = 1 as libc::c_int;
    } else {
        *encrypt_mode = 0 as libc::c_int;
    }
    memcpy(
        seed as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(8 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        sip as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(12 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        (drcom_config.KEEP_ALIVE_VERSION).as_mut_ptr() as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(28 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pppoe_login(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut pppoe_counter: *mut libc::c_int,
    mut seed: *mut libc::c_uchar,
    mut sip: *mut libc::c_uchar,
    mut login_first: *mut libc::c_int,
    mut encrypt_mode: *mut libc::c_int,
    mut encrypt_type: *mut libc::c_int,
) -> libc::c_int {
    let mut login_packet: [libc::c_uchar; 96] = [0; 96];
    let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
    memset(
        login_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        96 as libc::c_int as libc::c_ulong,
    );
    let mut login_tmp: [libc::c_uchar; 5] = [
        0x7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x60 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x3 as libc::c_int as libc::c_uchar,
    ];
    memcpy(
        login_packet.as_mut_ptr() as *mut libc::c_void,
        login_tmp.as_mut_ptr() as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    );
    login_packet[1 as libc::c_int
        as usize] = (*pppoe_counter % 0xff as libc::c_int) as libc::c_uchar;
    *pppoe_counter += 1;
    *pppoe_counter;
    memcpy(
        login_packet.as_mut_ptr().offset(12 as libc::c_int as isize)
            as *mut libc::c_void,
        sip as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    if *login_first != 0 {
        login_packet[17 as libc::c_int as usize] = 0x62 as libc::c_int as libc::c_uchar;
    } else {
        login_packet[17 as libc::c_int as usize] = 0x63 as libc::c_int as libc::c_uchar;
    }
    memcpy(
        login_packet.as_mut_ptr().offset(19 as libc::c_int as isize)
            as *mut libc::c_void,
        &mut drcom_config.pppoe_flag as *mut libc::c_uchar as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        login_packet.as_mut_ptr().offset(20 as libc::c_int as isize)
            as *mut libc::c_void,
        seed as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    let mut crc: [libc::c_uchar; 8] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    *encrypt_type = *seed.offset(0 as libc::c_int as isize) as libc::c_int
        & 3 as libc::c_int;
    if *encrypt_mode == 0 {
        *encrypt_type = 0 as libc::c_int;
    }
    gen_crc(seed, *encrypt_type, crc.as_mut_ptr());
    let mut crc_tmp: [libc::c_uchar; 32] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    memcpy(
        crc_tmp.as_mut_ptr() as *mut libc::c_void,
        login_packet.as_mut_ptr() as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        crc_tmp.as_mut_ptr().offset(24 as libc::c_int as isize) as *mut libc::c_void,
        crc.as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    let mut ret: uint64_t = 0 as libc::c_int as uint64_t;
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut crc2: [libc::c_uchar; 4] = [0 as libc::c_int as libc::c_uchar, 0, 0, 0];
    if *encrypt_type == 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            ret = 0 as libc::c_int as uint64_t;
            let mut j: libc::c_int = 4 as libc::c_int;
            while j > 0 as libc::c_int {
                ret = ret
                    .wrapping_mul(256 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        crc_tmp[(i + j - 1 as libc::c_int) as usize] as libc::c_int
                            as libc::c_ulong,
                    );
                j -= 1;
                j;
            }
            sum ^= ret;
            sum &= 0xffffffff as libc::c_uint as libc::c_ulong;
            i += 4 as libc::c_int;
        }
        sum = sum.wrapping_mul(19680126 as libc::c_int as libc::c_ulong)
            & 0xffffffff as libc::c_uint as libc::c_ulong;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            crc2[i_0
                as usize] = sum.wrapping_rem(256 as libc::c_int as libc::c_ulong)
                as libc::c_uchar;
            sum = (sum as libc::c_ulong)
                .wrapping_div(256 as libc::c_int as libc::c_ulong) as uint64_t
                as uint64_t;
            i_0 += 1;
            i_0;
        }
        memcpy(
            login_packet.as_mut_ptr().offset(24 as libc::c_int as isize)
                as *mut libc::c_void,
            crc2.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
    } else {
        memcpy(
            login_packet.as_mut_ptr().offset(24 as libc::c_int as isize)
                as *mut libc::c_void,
            crc.as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
    }
    sendto(
        sockfd,
        login_packet.as_mut_ptr() as *const libc::c_void,
        96 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[PPPoE_login sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            login_packet.as_mut_ptr(),
            96 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[PPPoE_login sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            login_packet.as_mut_ptr(),
            96 as libc::c_int,
        );
    }
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) < 0 as libc::c_int as libc::c_long
    {
        perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if verbose_flag != 0 {
        print_packet(
            b"[PPPoE_login recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            48 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[PPPoE_login recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            48 as libc::c_int,
        );
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int != 0x7 as libc::c_int {
        printf(
            b"Bad pppoe_login response received.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    recvfrom(
        sockfd,
        recv_packet.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    ) >= 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dogcom(mut try_times: libc::c_int) -> libc::c_int {
    let mut sockfd: libc::c_int = 0;
    let mut bind_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    memset(
        &mut bind_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    bind_addr.sin_family = 2 as libc::c_int as sa_family_t;
    if verbose_flag != 0 {
        printf(
            b"You are binding at %s!\n\n\0" as *const u8 as *const libc::c_char,
            bind_ip.as_mut_ptr(),
        );
    }
    bind_addr.sin_addr.s_addr = inet_addr(bind_ip.as_mut_ptr());
    bind_addr.sin_port = htons(61440 as libc::c_int as uint16_t);
    let mut dest_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    memset(
        &mut dest_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    dest_addr.sin_family = 2 as libc::c_int as sa_family_t;
    dest_addr.sin_addr.s_addr = inet_addr((drcom_config.server).as_mut_ptr());
    dest_addr.sin_port = htons(61440 as libc::c_int as uint16_t);
    srand(time(0 as *mut time_t) as libc::c_uint);
    sockfd = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        IPPROTO_UDP as libc::c_int,
    );
    if sockfd < 0 as libc::c_int {
        perror(b"Failed to create socket\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if bind(
        sockfd,
        &mut bind_addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"Failed to bind socket\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    timeout.tv_sec = 3 as libc::c_int as __time_t;
    timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    if setsockopt(
        sockfd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut timeout as *mut timeval as *mut libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"Failed to set sock opt\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if strcmp(mode.as_mut_ptr(), b"dhcp\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut login_failed_attempts: libc::c_int = 0 as libc::c_int;
        let mut try_JLUversion: libc::c_int = 0 as libc::c_int;
        let mut try_counter: libc::c_int = 0 as libc::c_int;
        while try_counter < try_times {
            if eternal_flag != 0 {
                try_counter = 0 as libc::c_int;
            }
            let mut seed: [libc::c_uchar; 4] = [0; 4];
            let mut auth_information: [libc::c_uchar; 16] = [0; 16];
            if dhcp_challenge(sockfd, dest_addr, seed.as_mut_ptr()) != 0 {
                printf(b"Retrying...\n\0" as *const u8 as *const libc::c_char);
                if logging_flag != 0 {
                    logging(
                        b"Retrying...\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_uchar,
                        0 as libc::c_int,
                    );
                }
                sleep(3 as libc::c_int as libc::c_uint);
            } else {
                usleep(200000 as libc::c_int as __useconds_t);
                if login_failed_attempts > 2 as libc::c_int {
                    try_JLUversion = 1 as libc::c_int;
                }
                if dhcp_login(
                    sockfd,
                    dest_addr,
                    seed.as_mut_ptr(),
                    auth_information.as_mut_ptr(),
                    try_JLUversion,
                ) == 0
                {
                    let mut keepalive_counter: libc::c_int = 0 as libc::c_int;
                    let mut keepalive_try_counter: libc::c_int = 0 as libc::c_int;
                    let mut first: libc::c_int = 1 as libc::c_int;
                    loop {
                        if keepalive_1(
                            sockfd,
                            dest_addr,
                            seed.as_mut_ptr(),
                            auth_information.as_mut_ptr(),
                        ) == 0
                        {
                            usleep(200000 as libc::c_int as __useconds_t);
                            if keepalive_2(
                                sockfd,
                                dest_addr,
                                &mut keepalive_counter,
                                &mut first,
                                0 as *mut libc::c_int,
                            ) != 0
                            {
                                continue;
                            }
                            if verbose_flag != 0 {
                                printf(
                                    b"Keepalive in loop.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if logging_flag != 0 {
                                logging(
                                    b"Keepalive in loop.\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    0 as *mut libc::c_uchar,
                                    0 as libc::c_int,
                                );
                            }
                            sleep(20 as libc::c_int as libc::c_uint);
                        } else {
                            if keepalive_try_counter > 5 as libc::c_int {
                                break;
                            }
                            keepalive_try_counter += 1;
                            keepalive_try_counter;
                        }
                    }
                } else {
                    login_failed_attempts += 1 as libc::c_int;
                    printf(b"Retrying...\n\0" as *const u8 as *const libc::c_char);
                    if logging_flag != 0 {
                        logging(
                            b"Retrying...\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            0 as *mut libc::c_uchar,
                            0 as libc::c_int,
                        );
                    }
                    sleep(3 as libc::c_int as libc::c_uint);
                }
            }
            try_counter += 1;
            try_counter;
        }
    } else if strcmp(mode.as_mut_ptr(), b"pppoe\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut pppoe_counter: libc::c_int = 0 as libc::c_int;
        let mut keepalive_counter_0: libc::c_int = 0 as libc::c_int;
        let mut seed_0: [libc::c_uchar; 4] = [0; 4];
        let mut sip: [libc::c_uchar; 4] = [0; 4];
        let mut login_first: libc::c_int = 1 as libc::c_int;
        let mut first_0: libc::c_int = 1 as libc::c_int;
        let mut encrypt_mode: libc::c_int = 0 as libc::c_int;
        let mut encrypt_type: libc::c_int = 0 as libc::c_int;
        let mut try_counter_0: libc::c_int = 0 as libc::c_int;
        loop {
            if pppoe_challenge(
                sockfd,
                dest_addr,
                &mut pppoe_counter,
                seed_0.as_mut_ptr(),
                sip.as_mut_ptr(),
                &mut encrypt_mode,
            ) != 0
            {
                printf(b"Retrying...\n\0" as *const u8 as *const libc::c_char);
                if logging_flag != 0 {
                    logging(
                        b"Retrying...\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_uchar,
                        0 as libc::c_int,
                    );
                }
                login_first = 1 as libc::c_int;
                try_counter_0 += 1;
                try_counter_0;
                if eternal_flag != 0 {
                    try_counter_0 = 0 as libc::c_int;
                }
                if try_counter_0 >= try_times {
                    break;
                }
                sleep(5 as libc::c_int as libc::c_uint);
            } else {
                usleep(200000 as libc::c_int as __useconds_t);
                if pppoe_login(
                    sockfd,
                    dest_addr,
                    &mut pppoe_counter,
                    seed_0.as_mut_ptr(),
                    sip.as_mut_ptr(),
                    &mut login_first,
                    &mut encrypt_mode,
                    &mut encrypt_type,
                ) != 0
                {
                    continue;
                }
                login_first = 0 as libc::c_int;
                if keepalive_2(
                    sockfd,
                    dest_addr,
                    &mut keepalive_counter_0,
                    &mut first_0,
                    &mut encrypt_type,
                ) != 0
                {
                    continue;
                }
                if verbose_flag != 0 {
                    printf(b"PPPoE in loop.\n\0" as *const u8 as *const libc::c_char);
                }
                if logging_flag != 0 {
                    logging(
                        b"PPPoE in loop.\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_uchar,
                        0 as libc::c_int,
                    );
                }
                sleep(10 as libc::c_int as libc::c_uint);
            }
        }
    }
    printf(
        b">>>>> Failed to keep in touch with server, exiting <<<<<\n\n\0" as *const u8
            as *const libc::c_char,
    );
    if logging_flag != 0 {
        logging(
            b">>>>> Failed to keep in touch with server, exiting <<<<<\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int,
        );
    }
    close(sockfd);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_packet(
    mut msg: *mut libc::c_char,
    mut packet: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    printf(b"%s\0" as *const u8 as *const libc::c_char, msg);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *packet.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn logging(
    mut msg: *mut libc::c_char,
    mut packet: *mut libc::c_uchar,
    mut length: libc::c_int,
) {
    let mut ptr_file: *mut FILE = 0 as *mut FILE;
    ptr_file = fopen(log_path, b"a\0" as *const u8 as *const libc::c_char);
    let mut wday: [*mut libc::c_char; 7] = [
        b"Sun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Mon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Tue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Wed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Thu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Fri\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Sat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut timep: time_t = 0;
    let mut p: *mut tm = 0 as *mut tm;
    time(&mut timep);
    p = localtime(&mut timep);
    fprintf(
        ptr_file,
        b"[%04d/%02d/%02d %s %02d:%02d:%02d] \0" as *const u8 as *const libc::c_char,
        1900 as libc::c_int + (*p).tm_year,
        1 as libc::c_int + (*p).tm_mon,
        (*p).tm_mday,
        wday[(*p).tm_wday as usize],
        (*p).tm_hour,
        (*p).tm_min,
        (*p).tm_sec,
    );
    fprintf(ptr_file, b"%s\0" as *const u8 as *const libc::c_char, msg);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        fprintf(
            ptr_file,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *packet.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    fprintf(ptr_file, b"\n\0" as *const u8 as *const libc::c_char);
    fclose(ptr_file);
}
