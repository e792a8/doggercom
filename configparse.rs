use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[no_mangle]
pub static mut verbose_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut logging_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut eapol_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut eternal_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut log_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut mode: [libc::c_char; 10] = [0; 10];
#[no_mangle]
pub static mut bind_ip: [libc::c_char; 20] = [0; 20];
#[no_mangle]
pub static mut drcom_config: config = config {
    server: [0; 20],
    username: [0; 36],
    password: [0; 20],
    CONTROLCHECKSTATUS: 0,
    ADAPTERNUM: 0,
    host_ip: [0; 20],
    IPDOG: 0,
    host_name: [0; 20],
    PRIMARY_DNS: [0; 20],
    dhcp_server: [0; 20],
    AUTH_VERSION: [0; 2],
    mac: [0; 6],
    host_os: [0; 20],
    KEEP_ALIVE_VERSION: [0; 2],
    ror_version: 0,
    keepalive1_mod: 0,
    pppoe_flag: 0,
    keep_alive2_flag: 0,
};
#[no_mangle]
pub unsafe extern "C" fn config_parse(mut filepath: *mut libc::c_char) -> libc::c_int {
    let mut ptr_file: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 100] = [0; 100];
    ptr_file = fopen(filepath, b"r\0" as *const u8 as *const libc::c_char);
    if ptr_file.is_null() {
        printf(b"Failed to read config file.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
        ptr_file,
    ))
        .is_null()
    {
        if strcmp(mode.as_mut_ptr(), b"dhcp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            read_d_config(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                    as libc::c_int,
            );
        } else if strcmp(
            mode.as_mut_ptr(),
            b"pppoe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            read_p_config(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                    as libc::c_int,
            );
        }
    }
    if verbose_flag != 0 {
        printf(b"\n\n\0" as *const u8 as *const libc::c_char);
    }
    fclose(ptr_file);
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_d_config(
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    if verbose_flag != 0 {
        printf(b"%s\0" as *const u8 as *const libc::c_char, buf);
    }
    let mut delim: *mut libc::c_char = b" ='\r\n\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut delim2: *mut libc::c_char = b"\\x\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    key = strtok(buf, delim);
    if strlen(key) != 0 {
        value = strtok(0 as *mut libc::c_char, delim);
    }
    drcom_config.keepalive1_mod = 0 as libc::c_int;
    if strcmp(key, b"server\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        strcpy((drcom_config.server).as_mut_ptr(), value);
    } else if strcmp(key, b"username\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.username).as_mut_ptr(), value);
    } else if strcmp(key, b"password\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.password).as_mut_ptr(), value);
    } else if strcmp(key, b"CONTROLCHECKSTATUS\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        value = strtok(value, delim2);
        sscanf(
            value,
            b"%02hhx\0" as *const u8 as *const libc::c_char,
            &mut drcom_config.CONTROLCHECKSTATUS as *mut libc::c_uchar,
        );
    } else if strcmp(key, b"ADAPTERNUM\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        value = strtok(value, delim2);
        sscanf(
            value,
            b"%02hhx\0" as *const u8 as *const libc::c_char,
            &mut drcom_config.ADAPTERNUM as *mut libc::c_uchar,
        );
    } else if strcmp(key, b"host_ip\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.host_ip).as_mut_ptr(), value);
    } else if strcmp(key, b"IPDOG\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        value = strtok(value, delim2);
        sscanf(
            value,
            b"%02hhx\0" as *const u8 as *const libc::c_char,
            &mut drcom_config.IPDOG as *mut libc::c_uchar,
        );
    } else if strcmp(key, b"host_name\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.host_name).as_mut_ptr(), value);
    } else if strcmp(key, b"PRIMARY_DNS\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.PRIMARY_DNS).as_mut_ptr(), value);
    } else if strcmp(key, b"dhcp_server\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.dhcp_server).as_mut_ptr(), value);
    } else if strcmp(key, b"AUTH_VERSION\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut v1: *mut libc::c_char = strtok(value, delim2);
        let mut v2: *mut libc::c_char = strtok(0 as *mut libc::c_char, delim2);
        sscanf(v1, b"%02hhx\0" as *const u8 as *const libc::c_char, v1);
        sscanf(v2, b"%02hhx\0" as *const u8 as *const libc::c_char, v2);
        memcpy(
            &mut *(drcom_config.AUTH_VERSION)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            v1 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            &mut *(drcom_config.AUTH_VERSION)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            v2 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
    } else if strcmp(key, b"mac\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut delim3: *mut libc::c_char = b"x\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        value = strtok(value, delim3);
        value = strtok(0 as *mut libc::c_char, delim3);
        sscanf(
            value,
            b"%02hhx%02hhx%02hhx%02hhx%02hhx%02hhx\0" as *const u8
                as *const libc::c_char,
            &mut *(drcom_config.mac).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_uchar,
            &mut *(drcom_config.mac).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_uchar,
            &mut *(drcom_config.mac).as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_uchar,
            &mut *(drcom_config.mac).as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut libc::c_uchar,
            &mut *(drcom_config.mac).as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_uchar,
            &mut *(drcom_config.mac).as_mut_ptr().offset(5 as libc::c_int as isize)
                as *mut libc::c_uchar,
        );
    } else if strcmp(key, b"host_os\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        strcpy((drcom_config.host_os).as_mut_ptr(), value);
    } else if strcmp(key, b"KEEP_ALIVE_VERSION\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut v1_0: *mut libc::c_char = strtok(value, delim2);
        let mut v2_0: *mut libc::c_char = strtok(0 as *mut libc::c_char, delim2);
        sscanf(v1_0, b"%02hhx\0" as *const u8 as *const libc::c_char, v1_0);
        sscanf(v2_0, b"%02hhx\0" as *const u8 as *const libc::c_char, v2_0);
        memcpy(
            &mut *(drcom_config.KEEP_ALIVE_VERSION)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            v1_0 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            &mut *(drcom_config.KEEP_ALIVE_VERSION)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            v2_0 as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
        );
    } else if strcmp(key, b"ror_version\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if strcmp(value, b"True\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            drcom_config.ror_version = 1 as libc::c_int;
        } else {
            drcom_config.ror_version = 0 as libc::c_int;
        }
    } else if strcmp(key, b"keepalive1_mod\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if strcmp(value, b"True\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            drcom_config.keepalive1_mod = 1 as libc::c_int;
        } else {
            drcom_config.keepalive1_mod = 0 as libc::c_int;
        }
    } else {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_p_config(
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    if verbose_flag != 0 {
        printf(b"%s\0" as *const u8 as *const libc::c_char, buf);
    }
    let mut delim: *mut libc::c_char = b" ='\r\n\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut delim2: *mut libc::c_char = b"\\x\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    key = strtok(buf, delim);
    if strlen(key) != 0 {
        value = strtok(0 as *mut libc::c_char, delim);
    }
    if strcmp(key, b"server\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        strcpy((drcom_config.server).as_mut_ptr(), value);
    } else if strcmp(key, b"pppoe_flag\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        value = strtok(value, delim2);
        sscanf(
            value,
            b"%02hhx\0" as *const u8 as *const libc::c_char,
            &mut drcom_config.pppoe_flag as *mut libc::c_uchar,
        );
    } else if strcmp(key, b"keep_alive2_flag\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        value = strtok(value, delim2);
        sscanf(
            value,
            b"%02hhx\0" as *const u8 as *const libc::c_char,
            &mut drcom_config.keep_alive2_flag as *mut libc::c_uchar,
        );
    } else {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
