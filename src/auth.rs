use crate::{
    configparse::Config,
    debug::DisplayBytes,
    keepalive::{gen_crc, keepalive_1, keepalive_2},
    ArgMode, Args,
};
use anyhow::Result;
use bstr::ByteSlice;
use digest::Digest;
use md5::Md5;
use rand::random;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    thread::sleep,
    time::Duration,
};

const CHECK_MAC: u8 = 0x01;
const SERVER_BUSY: u8 = 0x02;
const WRONG_PASS: u8 = 0x03;
const NOT_ENOUGH: u8 = 0x04;
const FREEZE_UP: u8 = 0x05;
const NOT_ON_THIS_IP: u8 = 0x07;
const NOT_ON_THIS_MAC: u8 = 0x0B;
const TOO_MUCH_IP: u8 = 0x14;
const UPDATE_CLIENT: u8 = 0x15;
const NOT_ON_THIS_IP_MAC: u8 = 0x16;
const MUST_USE_DHCP: u8 = 0x17;

fn dhcp_challenge(
    sock: &UdpSocket,
    addr: &SocketAddr,
    seed: &mut [u8; 4],
    config: &Config,
) -> Result<()> {
    let mut challenge_packet = [0u8; 20];
    let mut recv_packet = [0u8; 1024];
    challenge_packet[0] = 0x01;
    challenge_packet[1] = 0x02;
    challenge_packet[2] = random();
    challenge_packet[3] = random();
    challenge_packet[4] = config.auth_version[0];

    sock.send_to(&challenge_packet, addr)?;
    debug!("[Challenge sent] {}", DisplayBytes(challenge_packet));

    sock.recv_from(&mut recv_packet)?;
    debug!("[Challenge recv] {}", DisplayBytes(&recv_packet[..76]));
    ensure!(recv_packet[0] == 0x02, "bad challenge response");

    seed.copy_from_slice(&recv_packet[4..8]);
    debug!("[Get seed] {}", DisplayBytes(seed));

    Ok(())
}

fn dhcp_login(
    sock: &UdpSocket,
    addr: &SocketAddr,
    seed: &[u8; 4],
    auth_info: &mut [u8; 16],
    try_jlu_version: bool,
    config: &Config,
) -> Result<()> {
    let mut length_padding = 0usize;
    let mut jlu_padding = 0usize;
    let pwd = &config.password[..];
    let uname = &config.username[..];
    let host_ip = config.host_ip.to_str_lossy().parse::<Ipv4Addr>()?.octets();
    let primary_dns = config
        .primary_dns
        .to_str_lossy()
        .parse::<Ipv4Addr>()?
        .octets();
    let dhcp_server = config
        .dhcp_server
        .to_str_lossy()
        .parse::<Ipv4Addr>()?
        .octets();
    let host_name = &config.host_name[..];
    let host_os = &config.host_os[..];

    if pwd.len() > 8 {
        length_padding = pwd.len() - 8;
        if try_jlu_version {
            info!("Start JLU mode.");
            if pwd.len() != 16 {
                jlu_padding = pwd.len() / 4;
            }
            length_padding = 20usize + pwd.len() + jlu_padding;
        }
    }
    let login_packet_size = if config.ror_version {
        338usize + length_padding
    } else {
        330usize
    };
    let mut login_packet = vec![0u8; login_packet_size];
    let mut recv_packet = [0u8; 1024];
    let mut md5_a = [0u8; 16];
    let mut mac_xor_md5_a = [0u8; 6];
    let mut md5_b = [0u8; 16];
    let mut checksum1 = [0u8; 16];
    let mut checksum2 = [0u8; 4];
    login_packet[0] = 0x03;
    login_packet[1] = 0x01;
    login_packet[2] = 0x00;
    login_packet[3] = uname.len() as u8 + 20;
    let md5_a_len = 6 + pwd.len();
    let mut md5_a_str = vec![0u8; md5_a_len];
    md5_a_str[0] = 0x03;
    md5_a_str[1] = 0x01;
    md5_a_str[2..6].copy_from_slice(seed);
    md5_a_str[6..6 + pwd.len()].copy_from_slice(pwd);
    md5_a.copy_from_slice(&Md5::digest(&md5_a_str));
    login_packet[4..20].copy_from_slice(&md5_a);
    login_packet[20..20 + uname.len()].copy_from_slice(uname);
    login_packet[56] = config.controlcheckstatus;
    login_packet[57] = config.adapternum;
    for (i, v) in mac_xor_md5_a.iter_mut().enumerate() {
        *v = md5_a[i] ^ config.mac[i];
    }
    login_packet[58..64].copy_from_slice(&mac_xor_md5_a);
    let md5_b_len = 9 + pwd.len();
    let mut md5_b_str = vec![0u8; md5_b_len];
    md5_b_str[0] = 0x01;
    md5_b_str[1..1 + pwd.len()].copy_from_slice(pwd);
    md5_b_str[1 + pwd.len()..1 + pwd.len() + 4].copy_from_slice(seed);
    md5_b.copy_from_slice(&Md5::digest(md5_b_str));
    login_packet[64..80].copy_from_slice(&md5_b);
    login_packet[80] = 0x01;
    login_packet[81..85].copy_from_slice(&host_ip);
    let mut checksum1_str = [0u8; 101];
    let checksum1_tmp: [u8; 4] = [0x14, 0x00, 0x07, 0x0b];
    checksum1_str[..97].copy_from_slice(&login_packet[..97]);
    checksum1_str[97..101].copy_from_slice(&checksum1_tmp);
    checksum1.copy_from_slice(&Md5::digest(checksum1_str));
    login_packet[97..105].copy_from_slice(&checksum1[..8]);
    login_packet[105] = config.ipdog;
    login_packet[110..110 + host_name.len()].copy_from_slice(host_name);
    login_packet[142..146].copy_from_slice(&primary_dns);
    login_packet[146..150].copy_from_slice(&dhcp_server);
    let mut os_version_info_size: [u8; 4] = [0x94, 0, 0, 0];
    let mut osmajor: [u8; 4] = [0x05, 0, 0, 0];
    let mut osminor: [u8; 4] = [0x01, 0, 0, 0];
    let mut osbuild: [u8; 4] = [0x28, 0x0a, 0, 0];
    let mut platform_id: [u8; 4] = [0x02, 0, 0, 0];
    if try_jlu_version {
        os_version_info_size[0] = 0x94;
        osmajor[0] = 0x06;
        osminor[0] = 0x02;
        osbuild[0] = 0xf0;
        osbuild[1] = 0x23;
        platform_id[0] = 0x02;
        let service_pack: [u8; 40] = [
            0x33, 0x64, 0x63, 0x37, 0x39, 0x66, 0x35, 0x32, 0x31, 0x32, 0x65, 0x38, 0x31, 0x37,
            0x30, 0x61, 0x63, 0x66, 0x61, 0x39, 0x65, 0x63, 0x39, 0x35, 0x66, 0x31, 0x64, 0x37,
            0x34, 0x39, 0x31, 0x36, 0x35, 0x34, 0x32, 0x62, 0x65, 0x37, 0x62, 0x31,
        ];
        let hostname: [u8; 9] = [0x44, 0x72, 0x43, 0x4f, 0x4d, 0x00, 0xcf, 0x07, 0x68];
        login_packet[182..191].copy_from_slice(&hostname);
        login_packet[246..286].copy_from_slice(&service_pack);
    }
    login_packet[162..162 + 4].copy_from_slice(&os_version_info_size);
    login_packet[166..170].copy_from_slice(&osmajor);
    login_packet[170..174].copy_from_slice(&osminor);
    login_packet[174..178].copy_from_slice(&osbuild);
    login_packet[178..182].copy_from_slice(&platform_id);
    if !try_jlu_version {
        login_packet[182..182 + host_os.len()].copy_from_slice(host_os);
    }
    login_packet[310..312].copy_from_slice(&config.auth_version);
    let mut counter = 312usize;
    let mut ror_padding = 0usize;
    if pwd.len() <= 8 {
        ror_padding = 8 - pwd.len();
    } else {
        if (pwd.len() - 8) % 2 != 0 {
            ror_padding = 1;
        }
        if try_jlu_version {
            ror_padding = jlu_padding;
        }
    }
    if config.ror_version {
        md5_a.copy_from_slice(&Md5::digest(&md5_a_str));
        login_packet[counter + 1] = pwd.len() as u8;
        counter += 2;
        for i in 0..pwd.len() {
            let x = md5_a[i] ^ pwd[i];
            login_packet[counter + i] = x.rotate_left(3);
        }
        counter += pwd.len();
    } else {
        ror_padding = 2;
    }
    login_packet[counter] = 0x02;
    login_packet[counter + 1] = 0x0c;
    let mut checksum2_str = vec![0u8; counter + 18];
    let checksum2_tmp: [u8; 6] = [0x01, 0x26, 0x07, 0x11, 0, 0];
    checksum2_str[..counter + 2].copy_from_slice(&login_packet[..counter + 2]);
    checksum2_str[counter + 2..counter + 8].copy_from_slice(&checksum2_tmp);
    checksum2_str[counter + 8..counter + 14].copy_from_slice(&config.mac);
    let mut sum = 1234u64;
    for i in (0..counter + 14).step_by(4) {
        let mut ret = 0u64;
        for j in (0..4).rev() {
            ret = ret * 256 + checksum2_str[i + j] as u64;
        }
        sum ^= ret;
    }
    sum = (1968 * sum) & 0xffffffff;
    for (j, x) in checksum2.iter_mut().enumerate() {
        *x = (sum >> (j * 8) & 0xff) as u8;
    }
    login_packet[counter + 2..counter + 6].copy_from_slice(&checksum2);
    login_packet[counter + 8..counter + 14].copy_from_slice(&config.mac);
    login_packet[counter + ror_padding + 14] = 0xe9;
    login_packet[counter + ror_padding + 15] = 0x13;
    if try_jlu_version {
        login_packet[counter + ror_padding + 14] = 0x60;
        login_packet[counter + ror_padding + 15] = 0xa2;
    }

    sock.send_to(&login_packet, addr)?;
    debug!("[Login sent] {}", DisplayBytes(login_packet));

    sock.recv_from(&mut recv_packet)?;
    debug!("[Login recv] {}", DisplayBytes(&recv_packet[..100]));
    if recv_packet[0] != 0x04 {
        info!("<<< Login failed >>>");
        if recv_packet[0] == 0x05 {
            let err = match recv_packet[4] {
                CHECK_MAC => "[Tips] Someone is using this account with wired.",
                SERVER_BUSY => "[Tips] The server is busy, please log back in again.",
                WRONG_PASS => "[Tips] Account and password not match.",
                NOT_ENOUGH => "[Tips] The cumulative time or traffic for this account has exceeded the limit.",
                FREEZE_UP => "[Tips] This account is suspended.",
                NOT_ON_THIS_IP => "[Tips] IP address does not match, this account can only be used in the specified IP address.",
                NOT_ON_THIS_MAC => "[Tips] MAC address does not match, this account can only be used in the specified IP and MAC address.",
                TOO_MUCH_IP => "[Tips] This account has too many IP addresses.",
                UPDATE_CLIENT => "[Tips] The client version is incorrect.",
                NOT_ON_THIS_IP_MAC => "[Tips] This account can only be used on specified MAC and IP address.",
                MUST_USE_DHCP => "[Tips] Your PC set up a static IP, please change to DHCP, and then re-login.",
                _ => "[Tips] Unknown eror number.",
            };
            info!("{}", err);
        }
        bail!("login failed");
    } else {
        info!("<<< Logged in >>>");
    }

    auth_info.copy_from_slice(&recv_packet[23..39]);
    debug!("[Auth info] {}", DisplayBytes(auth_info));

    if sock.recv_from(&mut recv_packet).is_ok() {
        debug!("Get notice packet.");
    }

    Ok(())
}

fn pppoe_challenge(
    sock: &UdpSocket,
    addr: &SocketAddr,
    pppoe_counter: &mut u8,
    seed: &mut [u8; 4],
    sip: &mut [u8; 4],
    encrypt_mode: &mut u32,
    keep_alive_version: &mut [u8; 2],
) -> Result<()> {
    let mut challenge_packet = [0u8; 8];
    let mut recv_packet = [0u8; 1024];
    let challenge_tmp: [u8; 5] = [0x07, 0x00, 0x08, 0x00, 0x01];
    challenge_packet[..5].copy_from_slice(&challenge_tmp);
    challenge_packet[1] = *pppoe_counter;
    *pppoe_counter = pppoe_counter.wrapping_add(1);

    sock.send_to(&challenge_packet, addr)?;
    debug!("[Challenge sent] {}", DisplayBytes(challenge_packet));

    sock.recv_from(&mut recv_packet)?;
    debug!("[Challenge recv] {}", DisplayBytes(&recv_packet[..32]));

    ensure!(recv_packet[0] == 0x07, "bad challenge response");

    if recv_packet[5] != 0 {
        *encrypt_mode = 1;
    } else {
        *encrypt_mode = 0;
    }

    seed.copy_from_slice(&recv_packet[8..12]);
    sip.copy_from_slice(&recv_packet[12..16]);
    keep_alive_version.copy_from_slice(&recv_packet[28..30]);

    Ok(())
}

fn pppoe_login(
    sock: &UdpSocket,
    addr: &SocketAddr,
    pppoe_counter: &mut u8,
    seed: &[u8; 4],
    sip: &[u8; 4],
    login_first: bool,
    encrypt_mode: &u32,
    encrypt_type: &mut u8,
    config: &Config,
) -> Result<()> {
    let mut login_packet = [0u8; 96];
    let mut recv_packet = [0u8; 1024];
    let login_tmp: [u8; 5] = [0x07, 0x00, 0x60, 0x00, 0x03];
    login_packet[..5].copy_from_slice(&login_tmp);
    login_packet[1] = *pppoe_counter;
    *pppoe_counter = pppoe_counter.wrapping_add(1);
    login_packet[12..16].copy_from_slice(&sip[..]);
    if login_first {
        login_packet[17] = 0x62;
    } else {
        login_packet[17] = 0x63;
    }
    login_packet[19] = config.pppoe_flag;
    login_packet[20..24].copy_from_slice(&seed[..]);
    let mut crc = [0u8; 8];
    *encrypt_type = seed[0] & 3;
    if *encrypt_mode == 0 {
        *encrypt_type = 0;
    }
    gen_crc(seed, *encrypt_type, &mut crc);
    let mut crc_tmp = [0u8; 32];
    crc_tmp[..24].copy_from_slice(&login_packet[..24]);
    crc_tmp[24..32].copy_from_slice(&crc);
    let mut sum = 0u64;
    let mut crc2 = [0u8; 4];
    if *encrypt_type == 0 {
        for i in (0..32).step_by(4) {
            let mut ret = 0u64;
            for j in (0..4).rev() {
                ret = ret * 256 + crc_tmp[i + j] as u64;
            }
            sum ^= ret;
            sum &= 0xffffffff;
        }
        sum = (sum * 19680126) & 0xffffffff;
        for i in &mut crc2 {
            *i = (sum % 256) as u8;
            sum /= 256;
        }
        login_packet[24..28].copy_from_slice(&crc2);
    } else {
        login_packet[24..32].copy_from_slice(&crc);
    }

    sock.send_to(&login_packet, addr)?;
    debug!("[PPPoE_login sent] {}", DisplayBytes(&login_packet[..96]));

    sock.recv_from(&mut recv_packet)?;
    debug!("[PPPoE_login recv] {}", DisplayBytes(&recv_packet[..48]));

    ensure!(recv_packet[0] == 0x07, "bad login response");

    if sock.recv_from(&mut recv_packet).is_ok() {
        debug!("Get notice packet.");
    }

    Ok(())
}

pub fn doggercom(args: &Args, config: &Config) -> Result<()> {
    let addr: IpAddr = args.bindip.parse()?;
    let port = 61440u16;
    let server_addr: IpAddr = config.server.to_str_lossy().parse()?;
    let server_port = 61440u16;
    let dest_addr = SocketAddr::new(server_addr, server_port);
    let try_times = 5;

    info!("Binding at {}", addr);
    let sock = UdpSocket::bind((addr, port))?;
    sock.set_read_timeout(Some(Duration::from_secs(3)))?;

    match args.arg_mode {
        ArgMode::DHCP => {
            let mut login_failed_attempts = 0u32;
            let mut try_jlu_version = false;
            let mut try_cnt = 0u32;
            while try_cnt < try_times {
                if !args.eternal {
                    try_cnt += 1;
                }
                let mut seed = [0u8; 4];
                let mut auth_info = [0u8; 16];
                if let Err(err) = dhcp_challenge(&sock, &dest_addr, &mut seed, config) {
                    warn!("DHCP challenge failed: {err}. Retrying.");
                    sleep(Duration::from_secs(3));
                } else {
                    sleep(Duration::from_secs_f32(0.2));
                    if login_failed_attempts > 2 {
                        try_jlu_version = true;
                    }
                    if let Err(err) = dhcp_login(
                        &sock,
                        &dest_addr,
                        &seed,
                        &mut auth_info,
                        try_jlu_version,
                        config,
                    ) {
                        login_failed_attempts += 1;
                        warn!("DHCP Login failed: {err}. Retrying.");
                        sleep(Duration::from_secs(3));
                    } else {
                        let mut keepalive_counter = 0u8;
                        let mut keepalive_try_counter = 0;
                        let mut first = true;
                        loop {
                            if let Err(err) =
                                keepalive_1(&sock, &dest_addr, &seed, &auth_info, config)
                            {
                                warn!("Keepalive1 error: {err}");
                                if keepalive_try_counter > 5 {
                                    break;
                                }
                                keepalive_try_counter += 1;
                                continue;
                            } else {
                                sleep(Duration::from_secs_f32(0.2));
                                if let Err(err) = keepalive_2(
                                    &sock,
                                    &dest_addr,
                                    &mut keepalive_counter,
                                    &mut first,
                                    0,
                                    &config.keep_alive_version,
                                    config,
                                    &args.arg_mode,
                                ) {
                                    warn!("Keepalive1 error: {err}");
                                    continue;
                                }
                                debug!("Keepalive in loop.");
                                sleep(Duration::from_secs(20));
                            }
                        }
                    }
                }
            }
        }
        ArgMode::PPPoE => {
            let mut pppoe_counter = 0u8;
            let mut keepalive_counter = 0u8;
            let mut seed = [0u8; 4];
            let mut sip = [0u8; 4];
            let mut login_first = true;
            let mut first = true;
            let mut encrypt_mode: u32 = 0;
            let mut encrypt_type: u8 = 0;
            let mut try_counter: u32 = 0;
            let mut keep_alive_version = [0u8; 2];
            loop {
                if let Err(err) = pppoe_challenge(
                    &sock,
                    &dest_addr,
                    &mut pppoe_counter,
                    &mut seed,
                    &mut sip,
                    &mut encrypt_mode,
                    &mut keep_alive_version,
                ) {
                    warn!("PPPoE challenge failed: {err}. Retrying.");
                    login_first = true;
                    if !args.eternal {
                        try_counter += 1;
                    }
                    if try_counter >= try_times {
                        break;
                    }
                    sleep(Duration::from_secs(5));
                    continue;
                } else {
                    sleep(Duration::from_secs_f32(0.2));
                    if let Err(err) = pppoe_login(
                        &sock,
                        &dest_addr,
                        &mut pppoe_counter,
                        &seed,
                        &sip,
                        login_first,
                        &encrypt_mode,
                        &mut encrypt_type,
                        config,
                    ) {
                        warn!("PPPoE login error: {err}");
                        continue;
                    } else {
                        login_first = false;
                        if let Err(err) = keepalive_2(
                            &sock,
                            &dest_addr,
                            &mut keepalive_counter,
                            &mut first,
                            encrypt_type,
                            &keep_alive_version,
                            config,
                            &args.arg_mode,
                        ) {
                            warn!("Keepalive2 error: {err}");
                            continue;
                        } else {
                            debug!("PPPoE in loop.");
                            sleep(Duration::from_secs(10));
                            continue;
                        }
                    }
                }
            }
        }
    }

    error!(">>>>> Failed to keep in touch with server, exiting <<<<<");
    bail!("failed to keep in touch with server");
}
