use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
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
    fn htons(__hostshort: uint16_t) -> uint16_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn mac_equal(mac1: *const uchar, mac2: *const uchar) -> libc::c_int;
    fn format_time() -> *const libc::c_char;
    fn MD5(data: *const libc::c_void, size: libc::c_ulong, result: *mut libc::c_uchar);
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
}
pub type uchar = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type __uint16_t = libc::c_ushort;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
pub type socklen_t = __socklen_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_0,
    pub ifr_ifru: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ifrn_name: [libc::c_char; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ethII_t {
    pub dst_mac: [uchar; 6],
    pub src_mac: [uchar; 6],
    pub type_0: uint16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct eapol_t {
    pub ver: uchar,
    pub type_0: uchar,
    pub len: uint16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct eap_t {
    pub code: uchar,
    pub id: uchar,
    pub len: uint16,
    pub type_0: uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union eapbody_t {
    pub identity: [uchar; 32],
    pub md5clg: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2RustUnnamed_1 {
    pub _size: uchar,
    pub _md5value: [uchar; 16],
    pub _exdata: [uchar; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: libc::c_ushort,
    pub sll_protocol: libc::c_ushort,
    pub sll_ifindex: libc::c_int,
    pub sll_hatype: libc::c_ushort,
    pub sll_pkttype: libc::c_uchar,
    pub sll_halen: libc::c_uchar,
    pub sll_addr: [libc::c_uchar; 8],
}
static mut client_mac: [uchar; 6] = [0; 6];
static mut sendbuff: [uchar; 512] = [0; 512];
static mut recvbuff: [uchar; 512] = [0; 512];
static mut ifname: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_char; 16],
    >(b"eth0\0\0\0\0\0\0\0\0\0\0\0\0")
};
static mut sendethii: *mut ethII_t = 0 as *const ethII_t as *mut ethII_t;
static mut recvethii: *mut ethII_t = 0 as *const ethII_t as *mut ethII_t;
static mut sendeapol: *mut eapol_t = 0 as *const eapol_t as *mut eapol_t;
static mut recveapol: *mut eapol_t = 0 as *const eapol_t as *mut eapol_t;
static mut sendeap: *mut eap_t = 0 as *const eap_t as *mut eap_t;
static mut recveap: *mut eap_t = 0 as *const eap_t as *mut eap_t;
static mut sendeapbody: *mut eapbody_t = 0 as *const eapbody_t as *mut eapbody_t;
static mut recveapbody: *mut eapbody_t = 0 as *const eapbody_t as *mut eapbody_t;
static mut _uname: [libc::c_char; 32] = [0; 32];
static mut _pwd: [libc::c_char; 32] = [0; 32];
static mut pwdlen: libc::c_int = 0;
unsafe extern "C" fn eapol_init(
    mut skfd: *mut libc::c_int,
    mut skaddr: *mut sockaddr,
) -> libc::c_int {
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_0 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut skllp: *mut sockaddr_ll = skaddr as *mut sockaddr_ll;
    sendethii = sendbuff.as_mut_ptr() as *mut ethII_t;
    sendeapol = (sendethii as *mut uchar)
        .offset(::core::mem::size_of::<ethII_t>() as libc::c_ulong as isize)
        as *mut eapol_t;
    sendeap = (sendeapol as *mut uchar)
        .offset(::core::mem::size_of::<eapol_t>() as libc::c_ulong as isize)
        as *mut eap_t;
    sendeapbody = (sendeap as *mut uchar)
        .offset(::core::mem::size_of::<eap_t>() as libc::c_ulong as isize)
        as *mut eapbody_t;
    recvethii = recvbuff.as_mut_ptr() as *mut ethII_t;
    recveapol = (recvethii as *mut uchar)
        .offset(::core::mem::size_of::<ethII_t>() as libc::c_ulong as isize)
        as *mut eapol_t;
    recveap = (recveapol as *mut uchar)
        .offset(::core::mem::size_of::<eapol_t>() as libc::c_ulong as isize)
        as *mut eap_t;
    recveapbody = (recveap as *mut uchar)
        .offset(::core::mem::size_of::<eap_t>() as libc::c_ulong as isize)
        as *mut eapbody_t;
    *skfd = socket(
        17 as libc::c_int,
        SOCK_RAW as libc::c_int,
        htons(0x3 as libc::c_int as uint16_t) as libc::c_int,
    );
    if -(1 as libc::c_int) == *skfd {
        perror(b"Socket\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        skaddr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_ll>() as libc::c_ulong,
    );
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    strncpy(
        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
        ifname.as_mut_ptr(),
        16 as libc::c_int as libc::c_ulong,
    );
    if -(1 as libc::c_int)
        == ioctl(*skfd, 0x8933 as libc::c_int as libc::c_ulong, &mut ifr as *mut ifreq)
    {
        perror(b"Get index\0" as *const u8 as *const libc::c_char);
    } else {
        (*skllp).sll_ifindex = ifr.ifr_ifru.ifru_ivalue;
        if -(1 as libc::c_int)
            == ioctl(
                *skfd,
                0x8927 as libc::c_int as libc::c_ulong,
                &mut ifr as *mut ifreq,
            )
        {
            perror(b"Get MAC\0" as *const u8 as *const libc::c_char);
        } else {
            memcpy(
                client_mac.as_mut_ptr() as *mut libc::c_void,
                (ifr.ifr_ifru.ifru_hwaddr.sa_data).as_mut_ptr() as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            (*skllp).sll_family = 17 as libc::c_int as libc::c_ushort;
            (*skllp).sll_hatype = 1 as libc::c_int as libc::c_ushort;
            (*skllp).sll_pkttype = 0 as libc::c_int as libc::c_uchar;
            (*skllp).sll_halen = 6 as libc::c_int as libc::c_uchar;
            return 0 as libc::c_int;
        }
    }
    close(*skfd);
    return -(2 as libc::c_int);
}
unsafe extern "C" fn filte_req_identity(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut stime: libc::c_int = time(0 as *mut libc::c_void as *mut time_t)
        as libc::c_int;
    while difftime(time(0 as *mut libc::c_void as *mut time_t), stime as time_t)
        <= 3 as libc::c_int as libc::c_double
    {
        recvfrom(
            skfd,
            recvbuff.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as *mut sockaddr,
            0 as *mut socklen_t,
        );
        if (*recvethii).type_0 as libc::c_int
            == htons(0x888e as libc::c_int as uint16_t) as libc::c_int
            && mac_equal(((*recvethii).dst_mac).as_mut_ptr(), client_mac.as_mut_ptr())
                != 0 && (*recveapol).type_0 as libc::c_int == 0 as libc::c_int
            && (*recveap).code as libc::c_int == 0x1 as libc::c_int
            && (*recveap).type_0 as libc::c_int == 0x1 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn filte_req_md5clg(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut stime: libc::c_int = time(0 as *mut libc::c_void as *mut time_t)
        as libc::c_int;
    while difftime(time(0 as *mut libc::c_void as *mut time_t), stime as time_t)
        <= 3 as libc::c_int as libc::c_double
    {
        recvfrom(
            skfd,
            recvbuff.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as *mut sockaddr,
            0 as *mut socklen_t,
        );
        if (*recvethii).type_0 as libc::c_int
            == htons(0x888e as libc::c_int as uint16_t) as libc::c_int
            && mac_equal(((*recvethii).dst_mac).as_mut_ptr(), client_mac.as_mut_ptr())
                != 0 && (*recveapol).type_0 as libc::c_int == 0 as libc::c_int
        {
            if (*recveap).code as libc::c_int == 0x1 as libc::c_int
                && (*recveap).type_0 as libc::c_int == 0x4 as libc::c_int
            {
                return 0 as libc::c_int
            } else if (*recveap).id as libc::c_int == (*sendeap).id as libc::c_int
                && (*recveap).code as libc::c_int == 0x4 as libc::c_int
            {
                return -(2 as libc::c_int)
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn filte_success(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut stime: libc::c_int = time(0 as *mut libc::c_void as *mut time_t)
        as libc::c_int;
    while difftime(time(0 as *mut libc::c_void as *mut time_t), stime as time_t)
        <= 3 as libc::c_int as libc::c_double
    {
        recvfrom(
            skfd,
            recvbuff.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            0 as libc::c_int,
            0 as *mut sockaddr,
            0 as *mut socklen_t,
        );
        if (*recvethii).type_0 as libc::c_int
            == htons(0x888e as libc::c_int as uint16_t) as libc::c_int
            && mac_equal(((*recvethii).dst_mac).as_mut_ptr(), client_mac.as_mut_ptr())
                != 0 && (*recveapol).type_0 as libc::c_int == 0 as libc::c_int
        {
            if (*recveap).id as libc::c_int == (*sendeap).id as libc::c_int
                && (*recveap).code as libc::c_int == 0x3 as libc::c_int
            {
                return 0 as libc::c_int
            } else if (*recveap).id as libc::c_int == (*sendeap).id as libc::c_int
                && (*recveap).code as libc::c_int == 0x4 as libc::c_int
            {
                return -(2 as libc::c_int)
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn eapol_start(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut broadcast_mac: [uchar; 6] = [
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
    ];
    memcpy(
        ((*sendethii).dst_mac).as_mut_ptr() as *mut libc::c_void,
        broadcast_mac.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ((*sendethii).src_mac).as_mut_ptr() as *mut libc::c_void,
        client_mac.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*sendethii).type_0 = htons(0x888e as libc::c_int as uint16_t);
    (*sendeapol).ver = 0x1 as libc::c_int as uchar;
    (*sendeapol).type_0 = 0x1 as libc::c_int as uchar;
    (*sendeapol).len = 0 as libc::c_int as uint16;
    sendto(
        skfd,
        sendbuff.as_mut_ptr() as *const libc::c_void,
        (6 as libc::c_int * 2 as libc::c_int + 6 as libc::c_int) as size_t,
        0 as libc::c_int,
        skaddr,
        ::core::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn eapol_logoff(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut broadcast_mac: [uchar; 6] = [
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
        0xff as libc::c_int as uchar,
    ];
    memcpy(
        ((*sendethii).dst_mac).as_mut_ptr() as *mut libc::c_void,
        broadcast_mac.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ((*sendethii).src_mac).as_mut_ptr() as *mut libc::c_void,
        client_mac.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*sendethii).type_0 = htons(0x888e as libc::c_int as uint16_t);
    (*sendeapol).ver = 0x1 as libc::c_int as uchar;
    (*sendeapol).type_0 = 0x2 as libc::c_int as uchar;
    (*sendeapol).len = 0 as libc::c_int as uint16;
    (*sendeap).id = 255 as libc::c_int as uchar;
    sendto(
        skfd,
        sendbuff.as_mut_ptr() as *const libc::c_void,
        (6 as libc::c_int * 2 as libc::c_int + 6 as libc::c_int) as size_t,
        0 as libc::c_int,
        skaddr,
        ::core::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn eap_res_identity(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    memcpy(
        ((*sendethii).dst_mac).as_mut_ptr() as *mut libc::c_void,
        ((*recvethii).src_mac).as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    (*sendeapol).type_0 = 0 as libc::c_int as uchar;
    (*sendeapol)
        .len = htons(
        (::core::mem::size_of::<eap_t>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<eapbody_t>() as libc::c_ulong)
            as uint16_t,
    );
    (*sendeap).code = 0x2 as libc::c_int as uchar;
    (*sendeap).id = (*recveap).id;
    (*sendeap)
        .len = htons(::core::mem::size_of::<eapbody_t>() as libc::c_ulong as uint16_t);
    (*sendeap).type_0 = 0x1 as libc::c_int as uchar;
    strncpy(
        ((*sendeapbody).identity).as_mut_ptr() as *mut libc::c_char,
        _uname.as_mut_ptr(),
        32 as libc::c_int as libc::c_ulong,
    );
    sendto(
        skfd,
        sendbuff.as_mut_ptr() as *const libc::c_void,
        ((6 as libc::c_int * 2 as libc::c_int + 6 as libc::c_int + 5 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<eapbody_t>() as libc::c_ulong),
        0 as libc::c_int,
        skaddr,
        ::core::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn eap_md5_clg(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut md5buff: [uchar; 512] = [0; 512];
    (*sendeap).id = (*recveap).id;
    (*sendeap)
        .len = htons(::core::mem::size_of::<eapbody_t>() as libc::c_ulong as uint16_t);
    (*sendeap).type_0 = 0x4 as libc::c_int as uchar;
    (*sendeapbody).md5clg._size = (*recveapbody).md5clg._size;
    memcpy(
        md5buff.as_mut_ptr() as *mut libc::c_void,
        &mut (*sendeap).id as *mut uchar as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        md5buff.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        _pwd.as_mut_ptr() as *const libc::c_void,
        pwdlen as libc::c_ulong,
    );
    memcpy(
        md5buff.as_mut_ptr().offset(1 as libc::c_int as isize).offset(pwdlen as isize)
            as *mut libc::c_void,
        ((*recveapbody).md5clg._md5value).as_mut_ptr() as *const libc::c_void,
        (*recveapbody).md5clg._size as libc::c_ulong,
    );
    MD5(
        md5buff.as_mut_ptr() as *const libc::c_void,
        (1 as libc::c_int + pwdlen + (*recveapbody).md5clg._size as libc::c_int)
            as libc::c_ulong,
        ((*sendeapbody).md5clg._md5value).as_mut_ptr(),
    );
    memcpy(
        ((*sendeapbody).md5clg._exdata).as_mut_ptr() as *mut libc::c_char
            as *mut libc::c_void,
        _uname.as_mut_ptr() as *const libc::c_void,
        strlen(_uname.as_mut_ptr()),
    );
    sendto(
        skfd,
        sendbuff.as_mut_ptr() as *const libc::c_void,
        ((6 as libc::c_int * 2 as libc::c_int + 6 as libc::c_int + 5 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<eapbody_t>() as libc::c_ulong),
        0 as libc::c_int,
        skaddr,
        ::core::mem::size_of::<sockaddr_ll>() as libc::c_ulong as socklen_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn eap_keep_alive(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut stime: time_t = 0;
    let mut etime: time_t = 0;
    stime = time(0 as *mut libc::c_void as *mut time_t);
    loop {
        status = filte_req_identity(skfd, skaddr);
        if 0 as libc::c_int == status {
            etime = time(0 as *mut libc::c_void as *mut time_t);
            if difftime(etime, stime) <= 10 as libc::c_int as libc::c_double {
                stime = time(0 as *mut libc::c_void as *mut time_t);
                continue;
            } else {
                stime = time(0 as *mut libc::c_void as *mut time_t);
                fprintf(
                    stderr,
                    b"%s: [EAP:KPALV] get a request-identity\n\0" as *const u8
                        as *const libc::c_char,
                    format_time(),
                );
                eap_res_identity(skfd, skaddr);
            }
        }
        status = -(1 as libc::c_int);
    };
}
unsafe extern "C" fn eap_daemon(
    mut skfd: libc::c_int,
    mut skaddr: *const sockaddr,
) -> libc::c_int {
    let mut kpalvfd: *mut FILE = fopen(
        b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
        b"r+\0" as *const u8 as *const libc::c_char,
    );
    if kpalvfd.is_null() {
        fprintf(
            stderr,
            b"[EAP:KPALV] No process pidfile. %s: %s\n\0" as *const u8
                as *const libc::c_char,
            b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        kpalvfd = fopen(
            b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
            b"w+\0" as *const u8 as *const libc::c_char,
        );
        if kpalvfd.is_null() {
            fprintf(
                stderr,
                b"[EAP:KPALV] Detect pid file eror(%s)! quit!\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    let mut oldpid: pid_t = 0;
    fseek(kpalvfd, 0 as libc::c_long, 0 as libc::c_int);
    if 1 as libc::c_int
        == fscanf(
            kpalvfd,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut oldpid as *mut pid_t as *mut libc::c_int,
        ) && oldpid != -(1 as libc::c_int)
    {
        kill(oldpid, 9 as libc::c_int);
    }
    setsid();
    if 0 as libc::c_int != chdir(b"/\0" as *const u8 as *const libc::c_char) {
        fprintf(
            stderr,
            b"[EAP:KPALV:WARN] %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    umask(0 as libc::c_int as __mode_t);
    let mut curpid: pid_t = getpid();
    kpalvfd = freopen(
        b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
        b"w+\0" as *const u8 as *const libc::c_char,
        kpalvfd,
    );
    if kpalvfd.is_null() {
        fprintf(
            stderr,
            b"[EAP:KPALV:WARN] truncat pidfile '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    fprintf(kpalvfd, b"%d\0" as *const u8 as *const libc::c_char, curpid);
    fflush(kpalvfd);
    if 0 as libc::c_int == eap_keep_alive(skfd, skaddr) {
        fprintf(
            stderr,
            b"%s: [EAP:KPALV] Server maybe not need keep alive paket.\n\0" as *const u8
                as *const libc::c_char,
            format_time(),
        );
        fprintf(
            stderr,
            b"%s: [EAP:KPALV] Now, keep alive process quit!\n\0" as *const u8
                as *const libc::c_char,
            format_time(),
        );
    }
    kpalvfd = freopen(
        b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
        b"w+\0" as *const u8 as *const libc::c_char,
        kpalvfd,
    );
    if kpalvfd.is_null() {
        fprintf(
            stderr,
            b"[EAP:KPALV:WARN] truncat pidfile '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            b"/tmp/cwnu-drcom-eap.pid\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    fprintf(kpalvfd, b"-1\0" as *const u8 as *const libc::c_char);
    fflush(kpalvfd);
    fclose(kpalvfd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn eaplogin(
    mut uname: *const libc::c_char,
    mut pwd: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut skfd: libc::c_int = 0;
    let mut ll: sockaddr_ll = sockaddr_ll {
        sll_family: 0,
        sll_protocol: 0,
        sll_ifindex: 0,
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: 0,
        sll_addr: [0; 8],
    };
    fprintf(
        stderr,
        b"Use user '%s' to login...\n\0" as *const u8 as *const libc::c_char,
        uname,
    );
    fprintf(
        stderr,
        b"[EAP:0] Initilize interface...\n\0" as *const u8 as *const libc::c_char,
    );
    strncpy(_uname.as_mut_ptr(), uname, 32 as libc::c_int as libc::c_ulong);
    strncpy(_pwd.as_mut_ptr(), pwd, 32 as libc::c_int as libc::c_ulong);
    pwdlen = strlen(_pwd.as_mut_ptr()) as libc::c_int;
    if 0 as libc::c_int
        != eapol_init(&mut skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr)
    {
        return -(1 as libc::c_int);
    }
    eapol_logoff(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
    fprintf(
        stderr,
        b"[EAP:1] Send eap-start...\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        eapol_start(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
        if 0 as libc::c_int
            == filte_req_identity(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr)
        {
            break;
        }
        fprintf(
            stderr,
            b" [EAP:1] %dth Try send eap-start...\n\0" as *const u8
                as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
    if !(i >= 3 as libc::c_int) {
        fprintf(
            stderr,
            b"[EAP:2] Send response-identity...\n\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        loop {
            if !(i < 3 as libc::c_int) {
                current_block = 10652014663920648156;
                break;
            }
            eap_res_identity(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
            state = filte_req_md5clg(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
            if 0 as libc::c_int == state {
                current_block = 10652014663920648156;
                break;
            }
            if -(2 as libc::c_int) == state {
                current_block = 4612887429685283006;
                break;
            }
            fprintf(
                stderr,
                b" [EAP:2] %dth Try send response-identity...\n\0" as *const u8
                    as *const libc::c_char,
                i + 1 as libc::c_int,
            );
            i += 1;
            i;
        }
        match current_block {
            4612887429685283006 => {
                fprintf(
                    stderr,
                    b"[EAP:ERROR] No this user(%s).\n\0" as *const u8
                        as *const libc::c_char,
                    uname,
                );
                close(skfd);
                return 1 as libc::c_int;
            }
            _ => {
                if !(i >= 3 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"[EAP:3] Send response-md5clg...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < 3 as libc::c_int) {
                            current_block = 9520865839495247062;
                            break;
                        }
                        eap_md5_clg(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
                        state = filte_success(
                            skfd,
                            &mut ll as *mut sockaddr_ll as *mut sockaddr,
                        );
                        if 0 as libc::c_int == state {
                            fprintf(
                                stderr,
                                b"[EAP:4] Login success.\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 9520865839495247062;
                            break;
                        } else {
                            if -(2 as libc::c_int) == state {
                                current_block = 7158328990860465316;
                                break;
                            }
                            fprintf(
                                stderr,
                                b" [EAP:3] %dth Try send response-md5clg...\n\0"
                                    as *const u8 as *const libc::c_char,
                                i + 1 as libc::c_int,
                            );
                            i += 1;
                            i;
                        }
                    }
                    match current_block {
                        7158328990860465316 => {
                            fprintf(
                                stderr,
                                b"[EAP:ERROR] The server refuse to login. Password error.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            close(skfd);
                            return 4 as libc::c_int;
                        }
                        _ => {
                            if !(i >= 3 as libc::c_int) {
                                match fork() {
                                    0 => {
                                        if 0 as libc::c_int
                                            != eap_daemon(
                                                skfd,
                                                &mut ll as *mut sockaddr_ll as *mut sockaddr,
                                            )
                                        {
                                            fprintf(
                                                stderr,
                                                b"[EAP:ERROR] Create daemon process to keep alive error!\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                            close(skfd);
                                            exit(1 as libc::c_int);
                                        }
                                        exit(0 as libc::c_int);
                                    }
                                    -1 => {
                                        fprintf(
                                            stderr,
                                            b"[EAP:WARN] Cant create daemon, maybe `OFFLINE` after soon.\n\0"
                                                as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    _ => {}
                                }
                                close(skfd);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    fprintf(
        stderr,
        b"[EAP:ERROR] Not server in range.\n\0" as *const u8 as *const libc::c_char,
    );
    close(skfd);
    return -(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn eaplogoff() -> libc::c_int {
    let mut skfd: libc::c_int = 0;
    let mut ll: sockaddr_ll = sockaddr_ll {
        sll_family: 0,
        sll_protocol: 0,
        sll_ifindex: 0,
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: 0,
        sll_addr: [0; 8],
    };
    let mut state: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    fprintf(
        stderr,
        b"[EAP:0] Initilize interface...\n\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int
        != eapol_init(&mut skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr)
    {
        return -(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"[EAP:1] Requset logoff...\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        eapol_logoff(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
        state = filte_success(skfd, &mut ll as *mut sockaddr_ll as *mut sockaddr);
        if -(2 as libc::c_int) == state {
            fprintf(stderr, b"[EAP:2] Logoff!\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        fprintf(
            stderr,
            b" [EAP:1] %dth Try Requset logoff...\n\0" as *const u8
                as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
    fprintf(
        stderr,
        b"[EAP:ERROR] Not server in range. or You were logoff.\n\0" as *const u8
            as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn eaprefresh(
    mut uname: *const libc::c_char,
    mut pwd: *const libc::c_char,
) -> libc::c_int {
    return eaplogin(uname, pwd);
}
#[no_mangle]
pub unsafe extern "C" fn setifname(mut _ifname: *const libc::c_char) {
    strncpy(ifname.as_mut_ptr(), _ifname, 16 as libc::c_int as libc::c_ulong);
}
