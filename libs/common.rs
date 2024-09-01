use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type uchar = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iflist_t {
    pub name: [libc::c_char; 16],
}
#[no_mangle]
pub unsafe extern "C" fn getexedir(mut exedir: *mut libc::c_char) -> libc::c_int {
    let mut cnt: libc::c_int = readlink(
        b"/proc/self/exe\0" as *const u8 as *const libc::c_char,
        exedir,
        (4096 as libc::c_int + 1 as libc::c_int) as size_t,
    ) as libc::c_int;
    if cnt < 0 as libc::c_int || cnt >= 4096 as libc::c_int + 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut end: *mut libc::c_char = strrchr(exedir, '/' as i32);
    if end.is_null() {
        return -(1 as libc::c_int);
    }
    *end.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mac_equal(
    mut mac1: *const uchar,
    mut mac2: *const uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if *mac1.offset(i as isize) as libc::c_int
            != *mac2.offset(i as isize) as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ip_equal(
    mut type_0: libc::c_int,
    mut ip1: *const libc::c_void,
    mut ip2: *const libc::c_void,
) -> libc::c_int {
    let mut p1: *const uchar = ip1 as *const uchar;
    let mut p2: *const uchar = ip2 as *const uchar;
    let mut len: libc::c_int = 4 as libc::c_int;
    if 10 as libc::c_int == type_0 {
        len = 16 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        if *p1.offset(i as isize) as libc::c_int != *p2.offset(i as isize) as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_filter(mut ifname: *const libc::c_char) -> libc::c_int {
    let mut filter: [*const libc::c_char; 15] = [
        b"Wireless\0" as *const u8 as *const libc::c_char,
        b"Microsoft\0" as *const u8 as *const libc::c_char,
        b"Virtual\0" as *const u8 as *const libc::c_char,
        b"lo\0" as *const u8 as *const libc::c_char,
        b"wlan\0" as *const u8 as *const libc::c_char,
        b"vboxnet\0" as *const u8 as *const libc::c_char,
        b"ifb\0" as *const u8 as *const libc::c_char,
        b"gre\0" as *const u8 as *const libc::c_char,
        b"teql\0" as *const u8 as *const libc::c_char,
        b"br\0" as *const u8 as *const libc::c_char,
        b"imq\0" as *const u8 as *const libc::c_char,
        b"ra\0" as *const u8 as *const libc::c_char,
        b"wds\0" as *const u8 as *const libc::c_char,
        b"sit\0" as *const u8 as *const libc::c_char,
        b"apcli\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if !(strstr(ifname, filter[i as usize])).is_null() {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_ifname_from_buff(
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    while *(*__ctype_b_loc()).offset(*buff as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        buff = buff.offset(1);
        buff;
    }
    s = buff;
    while ':' as i32 != *buff as libc::c_int && '\0' as i32 != *buff as libc::c_int {
        buff = buff.offset(1);
        buff;
    }
    *buff = '\0' as i32 as libc::c_char;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn getall_ifs(
    mut ifs: *mut iflist_t,
    mut cnt: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if ifs.is_null() || *cnt <= 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut fd: *mut FILE = fopen(
        b"/proc/net/dev\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if fd.is_null() {
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    fgets(buff.as_mut_ptr(), 1024 as libc::c_int, fd);
    fgets(buff.as_mut_ptr(), 1024 as libc::c_int, fd);
    while !(fgets(buff.as_mut_ptr(), 1024 as libc::c_int, fd)).is_null() {
        name = get_ifname_from_buff(buff.as_mut_ptr());
        if is_filter(name) != 0 {
            continue;
        }
        strncpy(
            ((*ifs.offset(i as isize)).name).as_mut_ptr(),
            name,
            16 as libc::c_int as libc::c_ulong,
        );
        i += 1;
        i;
        if i >= *cnt {
            fclose(fd);
            return -(2 as libc::c_int);
        }
    }
    fclose(fd);
    *cnt = i;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn format_time() -> *const libc::c_char {
    static mut buff: [libc::c_char; 64] = [0; 64];
    let mut rawtime: time_t = 0;
    let mut timeinfo: *mut tm = 0 as *mut tm;
    time(&mut rawtime);
    timeinfo = localtime(&mut rawtime);
    if timeinfo.is_null() {
        return 0 as *const libc::c_char;
    }
    strftime(
        buff.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
        timeinfo,
    );
    return buff.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn copy(
    mut f1: *const libc::c_char,
    mut f2: *const libc::c_char,
) -> libc::c_int {
    if f1.is_null() || f2.is_null() {
        return -(1 as libc::c_int);
    }
    let mut src: *mut FILE = 0 as *mut FILE;
    let mut dst: *mut FILE = 0 as *mut FILE;
    src = fopen(f1, b"r\0" as *const u8 as *const libc::c_char);
    dst = fopen(f2, b"w\0" as *const u8 as *const libc::c_char);
    if src.is_null() || dst.is_null() {
        return -(1 as libc::c_int);
    }
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut n: libc::c_int = 0;
    loop {
        n = fread(
            buff.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1024 as libc::c_int as libc::c_ulong,
            src,
        ) as libc::c_int;
        if !((0 as libc::c_int) < n) {
            break;
        }
        fwrite(
            buff.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            n as libc::c_ulong,
            dst,
        );
    }
    fclose(src);
    fclose(dst);
    return 0 as libc::c_int;
}
unsafe extern "C" fn islsb() -> libc::c_int {
    static mut a: uint16 = 0x1 as libc::c_int as uint16;
    return *(&mut a as *mut uint16 as *mut uchar) as libc::c_int;
}
unsafe extern "C" fn exorders(mut n: uint16) -> uint16 {
    return (n as libc::c_int >> 8 as libc::c_int
        | (n as libc::c_int) << 8 as libc::c_int) as uint16;
}
unsafe extern "C" fn exorderl(mut n: uint32) -> uint32 {
    return n >> 24 as libc::c_int
        | (n & 0xff0000 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
        | (n & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | n << 24 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htols(mut n: uint16) -> uint16 {
    return (if islsb() != 0 { n as libc::c_int } else { exorders(n) as libc::c_int })
        as uint16;
}
#[no_mangle]
pub unsafe extern "C" fn htoms(mut n: uint16) -> uint16 {
    return (if islsb() != 0 { exorders(n) as libc::c_int } else { n as libc::c_int })
        as uint16;
}
#[no_mangle]
pub unsafe extern "C" fn ltohs(mut n: uint16) -> uint16 {
    return (if islsb() != 0 { n as libc::c_int } else { exorders(n) as libc::c_int })
        as uint16;
}
#[no_mangle]
pub unsafe extern "C" fn mtohs(mut n: uint16) -> uint16 {
    return (if islsb() != 0 { exorders(n) as libc::c_int } else { n as libc::c_int })
        as uint16;
}
#[no_mangle]
pub unsafe extern "C" fn htoll(mut n: uint32) -> uint32 {
    return if islsb() != 0 { n } else { exorderl(n) };
}
#[no_mangle]
pub unsafe extern "C" fn htoml(mut n: uint32) -> uint32 {
    return if islsb() != 0 { exorderl(n) } else { n };
}
#[no_mangle]
pub unsafe extern "C" fn ltohl(mut n: uint32) -> uint32 {
    return if islsb() != 0 { n } else { exorderl(n) };
}
#[no_mangle]
pub unsafe extern "C" fn mtohl(mut n: uint32) -> uint32 {
    return if islsb() != 0 { exorderl(n) } else { n };
}
#[no_mangle]
pub unsafe extern "C" fn format_mac(mut macarr: *const uchar) -> *const uchar {
    static mut formatmac: [uchar; 18] = unsafe {
        *::core::mem::transmute::<&[u8; 18], &mut [uchar; 18]>(b"xx:xx:xx:xx:xx:xx\0")
    };
    if macarr.is_null() {
        return 0 as *const uchar;
    }
    sprintf(
        formatmac.as_mut_ptr() as *mut libc::c_char,
        b"%.2X:%.2X:%.2X:%.2X:%.2X:%.2X\0" as *const u8 as *const libc::c_char,
        *macarr.offset(0 as libc::c_int as isize) as libc::c_int,
        *macarr.offset(1 as libc::c_int as isize) as libc::c_int,
        *macarr.offset(2 as libc::c_int as isize) as libc::c_int,
        *macarr.offset(3 as libc::c_int as isize) as libc::c_int,
        *macarr.offset(4 as libc::c_int as isize) as libc::c_int,
        *macarr.offset(5 as libc::c_int as isize) as libc::c_int,
    );
    return formatmac.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn format_data(mut d: *const uchar, mut len: size_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < len as libc::c_long {
        if i != 0 as libc::c_int && i % 16 as libc::c_int == 0 as libc::c_int {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
        fprintf(
            stderr,
            b"%02x \0" as *const u8 as *const libc::c_char,
            *d.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
