use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn rand() -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn print_packet(
        msg: *mut libc::c_char,
        packet: *mut libc::c_uchar,
        length: libc::c_int,
    );
    fn logging(msg: *mut libc::c_char, packet: *mut libc::c_uchar, length: libc::c_int);
    static mut drcom_config: config;
    static mut verbose_flag: libc::c_int;
    static mut logging_flag: libc::c_int;
    static mut mode: [libc::c_char; 10];
    fn MD4(data: *const libc::c_void, size: libc::c_ulong, result: *mut libc::c_uchar);
    fn MD5(data: *const libc::c_void, size: libc::c_ulong, result: *mut libc::c_uchar);
    fn SHA1(data: *const libc::c_uchar, len: uint32_t, digest: *mut libc::c_uchar);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
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
#[no_mangle]
pub unsafe extern "C" fn keepalive_1(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut seed: *mut libc::c_uchar,
    mut auth_information: *mut libc::c_uchar,
) -> libc::c_int {
    if drcom_config.keepalive1_mod != 0 {
        let mut keepalive_1_packet1: [libc::c_uchar; 8] = [
            0x7 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
            0x8 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0x1 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ];
        let mut recv_packet1: [libc::c_uchar; 1024] = [0; 1024];
        let mut keepalive_1_packet2: [libc::c_uchar; 38] = [0; 38];
        let mut recv_packet2: [libc::c_uchar; 1024] = [0; 1024];
        memset(
            keepalive_1_packet2.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            38 as libc::c_int as libc::c_ulong,
        );
        sendto(
            sockfd,
            keepalive_1_packet1.as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if verbose_flag != 0 {
            print_packet(
                b"[Keepalive1_packet1 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet1.as_mut_ptr(),
                8 as libc::c_int,
            );
        }
        if logging_flag != 0 {
            logging(
                b"[Keepalive1_packet1 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet1.as_mut_ptr(),
                8 as libc::c_int,
            );
        }
        let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>()
            as libc::c_ulong as socklen_t;
        loop {
            if recvfrom(
                sockfd,
                recv_packet1.as_mut_ptr() as *mut libc::c_void,
                1024 as libc::c_int as size_t,
                0 as libc::c_int,
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut addrlen,
            ) < 0 as libc::c_int as libc::c_long
            {
                perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
                return 1 as libc::c_int;
            } else {
                if verbose_flag != 0 {
                    print_packet(
                        b"[Keepalive1 challenge_recv] \0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        recv_packet1.as_mut_ptr(),
                        100 as libc::c_int,
                    );
                }
                if logging_flag != 0 {
                    logging(
                        b"[Keepalive1 challenge_recv] \0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        recv_packet1.as_mut_ptr(),
                        100 as libc::c_int,
                    );
                }
                if recv_packet1[0 as libc::c_int as usize] as libc::c_int
                    == 0x7 as libc::c_int
                {
                    break;
                }
                if recv_packet1[0 as libc::c_int as usize] as libc::c_int
                    == 0x4d as libc::c_int
                {
                    continue;
                }
                printf(
                    b"Bad keepalive1 challenge response received.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
        let mut keepalive1_seed: [libc::c_uchar; 4] = [
            0 as libc::c_int as libc::c_uchar,
            0,
            0,
            0,
        ];
        let mut encrypt_type: libc::c_int = 0;
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
        memcpy(
            keepalive1_seed.as_mut_ptr() as *mut libc::c_void,
            &mut *recv_packet1.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut libc::c_uchar as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        encrypt_type = keepalive1_seed[0 as libc::c_int as usize] as libc::c_int
            & 3 as libc::c_int;
        gen_crc(keepalive1_seed.as_mut_ptr(), encrypt_type, crc.as_mut_ptr());
        keepalive_1_packet2[0 as libc::c_int
            as usize] = 0xff as libc::c_int as libc::c_uchar;
        memcpy(
            keepalive_1_packet2.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut libc::c_void,
            keepalive1_seed.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            keepalive_1_packet2.as_mut_ptr().offset(12 as libc::c_int as isize)
                as *mut libc::c_void,
            crc.as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            keepalive_1_packet2.as_mut_ptr().offset(20 as libc::c_int as isize)
                as *mut libc::c_void,
            auth_information as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        keepalive_1_packet2[36 as libc::c_int
            as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
        keepalive_1_packet2[37 as libc::c_int
            as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
        sendto(
            sockfd,
            keepalive_1_packet2.as_mut_ptr() as *const libc::c_void,
            38 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if verbose_flag != 0 {
            print_packet(
                b"[Keepalive1_packet2 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet2.as_mut_ptr(),
                38 as libc::c_int,
            );
        }
        if logging_flag != 0 {
            logging(
                b"[Keepalive1_packet2 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet2.as_mut_ptr(),
                38 as libc::c_int,
            );
        }
        if recvfrom(
            sockfd,
            recv_packet2.as_mut_ptr() as *mut libc::c_void,
            1024 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            &mut addrlen,
        ) < 0 as libc::c_int as libc::c_long
        {
            perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        } else {
            if verbose_flag != 0 {
                print_packet(
                    b"[Keepalive1 recv] \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    recv_packet2.as_mut_ptr(),
                    100 as libc::c_int,
                );
            }
            if logging_flag != 0 {
                logging(
                    b"[Keepalive1 recv] \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    recv_packet2.as_mut_ptr(),
                    100 as libc::c_int,
                );
            }
            if recv_packet2[0 as libc::c_int as usize] as libc::c_int
                != 0x7 as libc::c_int
            {
                printf(
                    b"Bad keepalive1 response received.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
    } else {
        let mut keepalive_1_packet: [libc::c_uchar; 42] = [0; 42];
        let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
        let mut MD5A: [libc::c_uchar; 16] = [0; 16];
        memset(
            keepalive_1_packet.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            42 as libc::c_int as libc::c_ulong,
        );
        keepalive_1_packet[0 as libc::c_int
            as usize] = 0xff as libc::c_int as libc::c_uchar;
        let mut MD5A_len: libc::c_int = (6 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((drcom_config.password).as_mut_ptr())) as libc::c_int;
        let vla = MD5A_len as usize;
        let mut MD5A_str: Vec::<libc::c_uchar> = ::std::vec::from_elem(0, vla);
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
            keepalive_1_packet.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            MD5A.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            keepalive_1_packet.as_mut_ptr().offset(20 as libc::c_int as isize)
                as *mut libc::c_void,
            auth_information as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        keepalive_1_packet[36 as libc::c_int
            as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
        keepalive_1_packet[37 as libc::c_int
            as usize] = (rand() & 0xff as libc::c_int) as libc::c_uchar;
        sendto(
            sockfd,
            keepalive_1_packet.as_mut_ptr() as *const libc::c_void,
            42 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if verbose_flag != 0 {
            print_packet(
                b"[Keepalive1 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet.as_mut_ptr(),
                42 as libc::c_int,
            );
        }
        if logging_flag != 0 {
            logging(
                b"[Keepalive1 sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_1_packet.as_mut_ptr(),
                42 as libc::c_int,
            );
        }
        let mut addrlen_0: socklen_t = ::core::mem::size_of::<sockaddr_in>()
            as libc::c_ulong as socklen_t;
        loop {
            if recvfrom(
                sockfd,
                recv_packet.as_mut_ptr() as *mut libc::c_void,
                1024 as libc::c_int as size_t,
                0 as libc::c_int,
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut addrlen_0,
            ) < 0 as libc::c_int as libc::c_long
            {
                perror(b"Failed to recv data\0" as *const u8 as *const libc::c_char);
                return 1 as libc::c_int;
            } else {
                if verbose_flag != 0 {
                    print_packet(
                        b"[Keepalive1 recv] \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        recv_packet.as_mut_ptr(),
                        100 as libc::c_int,
                    );
                }
                if logging_flag != 0 {
                    logging(
                        b"[Keepalive1 recv] \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        recv_packet.as_mut_ptr(),
                        100 as libc::c_int,
                    );
                }
                if recv_packet[0 as libc::c_int as usize] as libc::c_int
                    == 0x7 as libc::c_int
                {
                    break;
                }
                if recv_packet[0 as libc::c_int as usize] as libc::c_int
                    == 0x4d as libc::c_int
                {
                    continue;
                }
                printf(
                    b"Bad keepalive1 response received.\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gen_crc(
    mut seed: *mut libc::c_uchar,
    mut encrypt_type: libc::c_int,
    mut crc: *mut libc::c_uchar,
) {
    if encrypt_type == 0 as libc::c_int {
        let mut DRCOM_DIAL_EXT_PROTO_CRC_INIT: [libc::c_char; 4] = [
            0xc7 as libc::c_int as libc::c_char,
            0x2f as libc::c_int as libc::c_char,
            0x31 as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
        ];
        let mut gencrc_tmp: [libc::c_char; 4] = [
            0x7e as libc::c_int as libc::c_char,
            0,
            0,
            0,
        ];
        memcpy(
            crc as *mut libc::c_void,
            DRCOM_DIAL_EXT_PROTO_CRC_INIT.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            crc.offset(4 as libc::c_int as isize) as *mut libc::c_void,
            gencrc_tmp.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
    } else if encrypt_type == 1 as libc::c_int {
        let mut hash: [libc::c_uchar; 32] = [
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
        MD5(
            seed as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            hash.as_mut_ptr(),
        );
        *crc.offset(0 as libc::c_int as isize) = hash[2 as libc::c_int as usize];
        *crc.offset(1 as libc::c_int as isize) = hash[3 as libc::c_int as usize];
        *crc.offset(2 as libc::c_int as isize) = hash[8 as libc::c_int as usize];
        *crc.offset(3 as libc::c_int as isize) = hash[9 as libc::c_int as usize];
        *crc.offset(4 as libc::c_int as isize) = hash[5 as libc::c_int as usize];
        *crc.offset(5 as libc::c_int as isize) = hash[6 as libc::c_int as usize];
        *crc.offset(6 as libc::c_int as isize) = hash[13 as libc::c_int as usize];
        *crc.offset(7 as libc::c_int as isize) = hash[14 as libc::c_int as usize];
    } else if encrypt_type == 2 as libc::c_int {
        let mut hash_0: [libc::c_uchar; 32] = [
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
        MD4(
            seed as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            hash_0.as_mut_ptr(),
        );
        *crc.offset(0 as libc::c_int as isize) = hash_0[1 as libc::c_int as usize];
        *crc.offset(1 as libc::c_int as isize) = hash_0[2 as libc::c_int as usize];
        *crc.offset(2 as libc::c_int as isize) = hash_0[8 as libc::c_int as usize];
        *crc.offset(3 as libc::c_int as isize) = hash_0[9 as libc::c_int as usize];
        *crc.offset(4 as libc::c_int as isize) = hash_0[4 as libc::c_int as usize];
        *crc.offset(5 as libc::c_int as isize) = hash_0[5 as libc::c_int as usize];
        *crc.offset(6 as libc::c_int as isize) = hash_0[11 as libc::c_int as usize];
        *crc.offset(7 as libc::c_int as isize) = hash_0[12 as libc::c_int as usize];
    } else if encrypt_type == 3 as libc::c_int {
        let mut hash_1: [libc::c_uchar; 32] = [
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
        SHA1(
            seed as *const libc::c_uchar,
            4 as libc::c_int as uint32_t,
            hash_1.as_mut_ptr(),
        );
        *crc.offset(0 as libc::c_int as isize) = hash_1[2 as libc::c_int as usize];
        *crc.offset(1 as libc::c_int as isize) = hash_1[3 as libc::c_int as usize];
        *crc.offset(2 as libc::c_int as isize) = hash_1[9 as libc::c_int as usize];
        *crc.offset(3 as libc::c_int as isize) = hash_1[10 as libc::c_int as usize];
        *crc.offset(4 as libc::c_int as isize) = hash_1[5 as libc::c_int as usize];
        *crc.offset(5 as libc::c_int as isize) = hash_1[6 as libc::c_int as usize];
        *crc.offset(6 as libc::c_int as isize) = hash_1[15 as libc::c_int as usize];
        *crc.offset(7 as libc::c_int as isize) = hash_1[16 as libc::c_int as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn keepalive_2_packetbuilder(
    mut keepalive_2_packet: *mut libc::c_uchar,
    mut keepalive_counter: libc::c_int,
    mut filepacket: libc::c_int,
    mut type_0: libc::c_int,
    mut encrypt_type: libc::c_int,
) {
    *keepalive_2_packet
        .offset(0 as libc::c_int as isize) = 0x7 as libc::c_int as libc::c_uchar;
    *keepalive_2_packet
        .offset(1 as libc::c_int as isize) = keepalive_counter as libc::c_uchar;
    *keepalive_2_packet
        .offset(2 as libc::c_int as isize) = 0x28 as libc::c_int as libc::c_uchar;
    *keepalive_2_packet
        .offset(4 as libc::c_int as isize) = 0xb as libc::c_int as libc::c_uchar;
    *keepalive_2_packet.offset(5 as libc::c_int as isize) = type_0 as libc::c_uchar;
    if filepacket != 0 {
        *keepalive_2_packet
            .offset(6 as libc::c_int as isize) = 0xf as libc::c_int as libc::c_uchar;
        *keepalive_2_packet
            .offset(7 as libc::c_int as isize) = 0x27 as libc::c_int as libc::c_uchar;
    } else {
        memcpy(
            keepalive_2_packet.offset(6 as libc::c_int as isize) as *mut libc::c_void,
            (drcom_config.KEEP_ALIVE_VERSION).as_mut_ptr() as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
    }
    *keepalive_2_packet
        .offset(8 as libc::c_int as isize) = 0x2f as libc::c_int as libc::c_uchar;
    *keepalive_2_packet
        .offset(9 as libc::c_int as isize) = 0x12 as libc::c_int as libc::c_uchar;
    if type_0 == 3 as libc::c_int {
        let mut host_ip: [libc::c_uchar; 4] = [
            0 as libc::c_int as libc::c_uchar,
            0,
            0,
            0,
        ];
        if strcmp(mode.as_mut_ptr(), b"dhcp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
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
                keepalive_2_packet.offset(28 as libc::c_int as isize)
                    as *mut libc::c_void,
                host_ip.as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
        } else if strcmp(
            mode.as_mut_ptr(),
            b"pppoe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
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
            gen_crc(keepalive_2_packet, encrypt_type, crc.as_mut_ptr());
            memcpy(
                keepalive_2_packet.offset(32 as libc::c_int as isize)
                    as *mut libc::c_void,
                crc.as_mut_ptr() as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn keepalive_2(
    mut sockfd: libc::c_int,
    mut addr: sockaddr_in,
    mut keepalive_counter: *mut libc::c_int,
    mut first: *mut libc::c_int,
    mut encrypt_type: *mut libc::c_int,
) -> libc::c_int {
    let mut keepalive_2_packet: [libc::c_uchar; 40] = [0; 40];
    let mut recv_packet: [libc::c_uchar; 1024] = [0; 1024];
    let mut tail: [libc::c_uchar; 4] = [0; 4];
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if *first != 0 {
        memset(
            keepalive_2_packet.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            40 as libc::c_int as libc::c_ulong,
        );
        if strcmp(mode.as_mut_ptr(), b"pppoe\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            keepalive_2_packetbuilder(
                keepalive_2_packet.as_mut_ptr(),
                *keepalive_counter % 0xff as libc::c_int,
                *first,
                1 as libc::c_int,
                *encrypt_type,
            );
        } else {
            keepalive_2_packetbuilder(
                keepalive_2_packet.as_mut_ptr(),
                *keepalive_counter % 0xff as libc::c_int,
                *first,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        *keepalive_counter += 1;
        *keepalive_counter;
        sendto(
            sockfd,
            keepalive_2_packet.as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        );
        if verbose_flag != 0 {
            print_packet(
                b"[Keepalive2_file sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_2_packet.as_mut_ptr(),
                40 as libc::c_int,
            );
        }
        if logging_flag != 0 {
            logging(
                b"[Keepalive2_file sent] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                keepalive_2_packet.as_mut_ptr(),
                40 as libc::c_int,
            );
        }
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
                b"[Keepalive2_file recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                40 as libc::c_int,
            );
        }
        if logging_flag != 0 {
            logging(
                b"[Keepalive2_file recv] \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                recv_packet.as_mut_ptr(),
                40 as libc::c_int,
            );
        }
        if recv_packet[0 as libc::c_int as usize] as libc::c_int == 0x7 as libc::c_int {
            if recv_packet[2 as libc::c_int as usize] as libc::c_int
                == 0x10 as libc::c_int
            {
                if verbose_flag != 0 {
                    printf(
                        b"Filepacket received.\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if recv_packet[2 as libc::c_int as usize] as libc::c_int
                != 0x28 as libc::c_int
            {
                if verbose_flag != 0 {
                    printf(
                        b"Bad keepalive2 response received.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 1 as libc::c_int;
            }
        } else {
            printf(
                b"Bad keepalive2 response received.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    *first = 0 as libc::c_int;
    memset(
        keepalive_2_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        40 as libc::c_int as libc::c_ulong,
    );
    if strcmp(mode.as_mut_ptr(), b"pppoe\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        keepalive_2_packetbuilder(
            keepalive_2_packet.as_mut_ptr(),
            *keepalive_counter % 0xff as libc::c_int,
            *first,
            1 as libc::c_int,
            *encrypt_type,
        );
    } else {
        keepalive_2_packetbuilder(
            keepalive_2_packet.as_mut_ptr(),
            *keepalive_counter % 0xff as libc::c_int,
            *first,
            1 as libc::c_int,
            0 as libc::c_int,
        );
    }
    *keepalive_counter += 1;
    *keepalive_counter;
    sendto(
        sockfd,
        keepalive_2_packet.as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[Keepalive2_A sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keepalive_2_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Keepalive2_A sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keepalive_2_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
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
            b"[Keepalive2_B recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Keepalive2_B recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int == 0x7 as libc::c_int {
        if recv_packet[2 as libc::c_int as usize] as libc::c_int != 0x28 as libc::c_int {
            printf(
                b"Bad keepalive2 response received.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    } else {
        printf(
            b"Bad keepalive2 response received.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    memcpy(
        tail.as_mut_ptr() as *mut libc::c_void,
        &mut *recv_packet.as_mut_ptr().offset(16 as libc::c_int as isize)
            as *mut libc::c_uchar as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    memset(
        keepalive_2_packet.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        40 as libc::c_int as libc::c_ulong,
    );
    if strcmp(mode.as_mut_ptr(), b"pppoe\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        keepalive_2_packetbuilder(
            keepalive_2_packet.as_mut_ptr(),
            *keepalive_counter % 0xff as libc::c_int,
            *first,
            3 as libc::c_int,
            *encrypt_type,
        );
    } else {
        keepalive_2_packetbuilder(
            keepalive_2_packet.as_mut_ptr(),
            *keepalive_counter % 0xff as libc::c_int,
            *first,
            3 as libc::c_int,
            0 as libc::c_int,
        );
    }
    memcpy(
        keepalive_2_packet.as_mut_ptr().offset(16 as libc::c_int as isize)
            as *mut libc::c_void,
        tail.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    *keepalive_counter += 1;
    *keepalive_counter;
    sendto(
        sockfd,
        keepalive_2_packet.as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
        0 as libc::c_int,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if verbose_flag != 0 {
        print_packet(
            b"[Keepalive2_C sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keepalive_2_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Keepalive2_C sent] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keepalive_2_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
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
            b"[Keepalive2_D recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if logging_flag != 0 {
        logging(
            b"[Keepalive2_D recv] \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            recv_packet.as_mut_ptr(),
            40 as libc::c_int,
        );
    }
    if recv_packet[0 as libc::c_int as usize] as libc::c_int == 0x7 as libc::c_int {
        if recv_packet[2 as libc::c_int as usize] as libc::c_int != 0x28 as libc::c_int {
            printf(
                b"Bad keepalive2 response received.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    } else {
        printf(
            b"Bad keepalive2 response received.\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
