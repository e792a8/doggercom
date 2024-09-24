use crate::{configparse::Config, debug::DisplayBytes, ArgMode};
use anyhow::Result;
use bstr::ByteSlice;
use digest::Digest;
use md4::Md4;
use md5::Md5;
use rand::random;
use sha1::Sha1;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

pub fn keepalive_1(
    sock: &UdpSocket,
    addr: &SocketAddr,
    seed: &[u8; 4],
    auth_information: &[u8; 16],
    config: &Config,
) -> Result<()> {
    if config.keepalive1_mod {
        let keepalive_1_packet1: [u8; 8] = [0x07, 0x01, 0x08, 0x00, 0x01, 0x00, 0x00, 0x00];
        let mut recv_packet1 = [0u8; 1024];
        let mut keepalive_1_packet2 = [0u8; 38];
        let mut recv_packet2 = [0u8; 1024];
        sock.send_to(&keepalive_1_packet1, addr)?;
        debug!(
            "[Keepalive1_packet1 sent] {}",
            DisplayBytes(keepalive_1_packet1)
        );
        loop {
            sock.recv_from(&mut recv_packet1)?;
            debug!(
                "[Keepalive1 challenge_recv] {}",
                DisplayBytes(&recv_packet1[..100])
            );
            if recv_packet1[0] == 0x07 {
                break;
            }
            if recv_packet1[0] == 0x4d {
                continue;
            }
            bail!("bad keepalive1 challenge response");
        }
        let mut keepalive1_seed = [0u8; 4];
        let mut crc = [0u8; 8];
        keepalive1_seed.copy_from_slice(&recv_packet1[8..12]);
        let encrypt_type = keepalive1_seed[0] & 3u8;
        gen_crc(&keepalive1_seed, encrypt_type, &mut crc);
        keepalive_1_packet2[0] = 0xff;
        keepalive_1_packet2[8..12].copy_from_slice(&keepalive1_seed);
        keepalive_1_packet2[12..20].copy_from_slice(&crc);
        keepalive_1_packet2[20..36].copy_from_slice(auth_information);
        keepalive_1_packet2[36] = random();
        keepalive_1_packet2[37] = random();
        sock.send_to(&keepalive_1_packet2, addr)?;
        debug!(
            "[Keepalive1_packet2 sent] {}",
            DisplayBytes(&keepalive_1_packet2[..38])
        );
        sock.recv_from(&mut recv_packet2)?;
        debug!("[Keepalive1 recv] {}", DisplayBytes(&recv_packet2[..100]));
        ensure!(recv_packet2[0] == 0x07, "bad keepalive1 response");
    } else {
        let mut keepalive_1_packet = [0u8; 42];
        let mut recv_packet = [0u8; 1024];
        let mut md5_a = [0u8; 16];
        let pwd = &config.password[..];
        keepalive_1_packet[0] = 0xff;
        let md5_a_len = 6 + pwd.len();
        let mut md5_a_str = vec![0u8; md5_a_len];
        md5_a_str[0] = 0x03;
        md5_a_str[1] = 0x01;
        md5_a_str[2..6].copy_from_slice(seed);
        md5_a_str[6..6 + pwd.len()].copy_from_slice(pwd);
        md5_a.copy_from_slice(&Md5::digest(md5_a_str));
        keepalive_1_packet[1..17].copy_from_slice(&md5_a[..16]);
        keepalive_1_packet[20..36].copy_from_slice(auth_information);
        keepalive_1_packet[36] = random();
        keepalive_1_packet[37] = random();
        sock.send_to(&keepalive_1_packet, addr)?;
        debug!(
            "[Keepalive1 sent] {}",
            DisplayBytes(&keepalive_1_packet[..42])
        );
        loop {
            sock.recv_from(&mut recv_packet)?;
            debug!("[Keepalive1 recv] {}", DisplayBytes(&recv_packet[..100]));
            if recv_packet[0] == 0x07 {
                break;
            }
            if recv_packet[0] == 0x4d {
                continue;
            }
            bail!("bad keepalive1 response");
        }
    }

    Ok(())
}

pub fn gen_crc(seed: &[u8], encrypt_type: u8, crc: &mut [u8; 8]) {
    let seed = &seed[..4];
    match encrypt_type {
        0 => {
            let drcom_dial_ext_proto_crc_init: [u8; 4] = [0xc7, 0x2f, 0x31, 0x01];
            let gencrc_tmp: [u8; 4] = [0x7e, 0, 0, 0];
            crc[..4].copy_from_slice(&drcom_dial_ext_proto_crc_init);
            crc[4..8].copy_from_slice(&gencrc_tmp);
        }
        1 => {
            let hash = Md5::digest(seed);
            crc[0] = hash[2];
            crc[1] = hash[3];
            crc[2] = hash[8];
            crc[3] = hash[9];
            crc[4] = hash[5];
            crc[5] = hash[6];
            crc[6] = hash[13];
            crc[7] = hash[14];
        }
        2 => {
            let hash_0 = Md4::digest(seed);
            crc[0] = hash_0[1];
            crc[1] = hash_0[2];
            crc[2] = hash_0[8];
            crc[3] = hash_0[9];
            crc[4] = hash_0[4];
            crc[5] = hash_0[5];
            crc[6] = hash_0[11];
            crc[7] = hash_0[12];
        }
        3 => {
            let hash_1 = Sha1::digest(seed);
            crc[0] = hash_1[2];
            crc[1] = hash_1[3];
            crc[2] = hash_1[9];
            crc[3] = hash_1[10];
            crc[4] = hash_1[5];
            crc[5] = hash_1[6];
            crc[6] = hash_1[15];
            crc[7] = hash_1[16];
        }
        _ => error!("Unknown encrypt_type {}", encrypt_type),
    }
}

pub fn keepalive_2_packetbuilder(
    keepalive_2_packet: &mut [u8; 40],
    keepalive_counter: u8,
    filepacket: bool,
    type_0: u8,
    encrypt_type: u8,
    keep_alive_veresion: &[u8; 2],
    config: &Config,
    mode: &ArgMode,
) {
    keepalive_2_packet[0] = 0x07;
    keepalive_2_packet[1] = keepalive_counter;
    keepalive_2_packet[2] = 0x28;
    keepalive_2_packet[4] = 0x0b;
    keepalive_2_packet[5] = type_0;
    if filepacket {
        keepalive_2_packet[6] = 0x0f;
        keepalive_2_packet[7] = 0x27;
    } else {
        keepalive_2_packet[6..8].copy_from_slice(keep_alive_veresion);
    }
    keepalive_2_packet[8] = 0x2f;
    keepalive_2_packet[9] = 0x12;
    if type_0 == 3 {
        match mode {
            ArgMode::DHCP => {
                let host_ip = config
                    .host_ip
                    .to_str_lossy()
                    .parse::<Ipv4Addr>()
                    .unwrap()
                    .octets();
                keepalive_2_packet[28..32].copy_from_slice(&host_ip);
            }
            ArgMode::PPPoE => {
                let mut crc = [0u8; 8];
                gen_crc(&keepalive_2_packet[..4], encrypt_type, &mut crc);
                keepalive_2_packet[32..40].copy_from_slice(&crc);
            }
        }
    }
}

pub fn keepalive_2(
    sock: &UdpSocket,
    addr: &SocketAddr,
    keepalive_counter: &mut u8,
    first: &mut bool,
    encrypt_type: u8,
    keep_alive_veresion: &[u8; 2],
    config: &Config,
    mode: &ArgMode,
) -> Result<()> {
    let mut keepalive_2_packet = [0u8; 40];
    let mut recv_packet = [0u8; 1024];
    let mut tail = [0u8; 4];
    if *first {
        match mode {
            ArgMode::PPPoE => {
                keepalive_2_packetbuilder(
                    &mut keepalive_2_packet,
                    *keepalive_counter,
                    *first,
                    1,
                    encrypt_type,
                    keep_alive_veresion,
                    config,
                    mode,
                );
            }
            ArgMode::DHCP => {
                keepalive_2_packetbuilder(
                    &mut keepalive_2_packet,
                    *keepalive_counter,
                    *first,
                    1,
                    0,
                    keep_alive_veresion,
                    config,
                    mode,
                );
            }
        }
        *keepalive_counter = keepalive_counter.wrapping_add(1);
        sock.send_to(&keepalive_2_packet, addr)?;
        debug!(
            "[Keepalive2_file sent] {}",
            DisplayBytes(&keepalive_2_packet[..40])
        );
        sock.recv_from(&mut recv_packet)?;
        debug!(
            "[Keepalive2_file recv] {}",
            DisplayBytes(&recv_packet[..40])
        );
        ensure!(recv_packet[0] == 0x07, "bad keepalive2 response");
        if recv_packet[2] == 0x10 {
            debug!("Filepacket received.");
        } else {
            ensure!(recv_packet[2] == 0x28, "bad keepalive2 response");
        }
    }
    *first = false;
    keepalive_2_packet = [0; 40];
    match mode {
        ArgMode::PPPoE => {
            keepalive_2_packetbuilder(
                &mut keepalive_2_packet,
                *keepalive_counter,
                *first,
                1,
                encrypt_type,
                keep_alive_veresion,
                config,
                mode,
            );
        }
        ArgMode::DHCP => {
            keepalive_2_packetbuilder(
                &mut keepalive_2_packet,
                *keepalive_counter,
                *first,
                1,
                0,
                keep_alive_veresion,
                config,
                mode,
            );
        }
    }
    *keepalive_counter += 1;
    sock.send_to(&keepalive_2_packet, addr)?;
    debug!(
        "[Keepalive2_A sent] {}",
        DisplayBytes(&keepalive_2_packet[..40])
    );
    sock.recv_from(&mut recv_packet)?;
    debug!("[Keepalive2_B recv] {}", DisplayBytes(&recv_packet[..40]));
    ensure!(
        recv_packet[0] == 0x07 && recv_packet[2] == 0x28,
        "bad keepalive2 response"
    );
    tail.copy_from_slice(&recv_packet[16..20]);
    keepalive_2_packet = [0; 40];
    match mode {
        ArgMode::PPPoE => {
            keepalive_2_packetbuilder(
                &mut keepalive_2_packet,
                *keepalive_counter,
                *first,
                3,
                encrypt_type,
                keep_alive_veresion,
                config,
                mode,
            );
        }
        ArgMode::DHCP => {
            keepalive_2_packetbuilder(
                &mut keepalive_2_packet,
                *keepalive_counter,
                *first,
                3,
                0,
                keep_alive_veresion,
                config,
                mode,
            );
        }
    }
    keepalive_2_packet[16..20].copy_from_slice(&tail);
    *keepalive_counter += 1;

    sock.send_to(&keepalive_2_packet, addr)?;
    debug!(
        "[Keepalive2_C sent] {}",
        DisplayBytes(&keepalive_2_packet[..40])
    );
    sock.recv_from(&mut recv_packet)?;
    debug!("[Keepalive2_D recv] {}", DisplayBytes(&recv_packet[..40]));

    ensure!(
        recv_packet[0] == 0x07 && recv_packet[2] == 0x28,
        "bad keepalive2 response"
    );

    Ok(())
}
