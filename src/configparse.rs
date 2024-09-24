use anyhow::{Context, Result};
use bstr::{BString, ByteSlice};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::from_utf8_unchecked,
};

#[derive(Debug, PartialEq, Default)]
pub struct Config {
    pub server: BString,
    pub username: BString,
    pub password: BString,
    pub controlcheckstatus: u8,
    pub adapternum: u8,
    pub host_ip: BString,
    pub ipdog: u8,
    pub host_name: BString,
    pub primary_dns: BString,
    pub dhcp_server: BString,
    pub auth_version: [u8; 2],
    pub mac: [u8; 6],
    pub host_os: BString,
    pub keep_alive_version: [u8; 2],
    pub ror_version: bool,
    pub keepalive1_mod: bool,
    pub pppoe_flag: u8,
    pub keep_alive2_flag: u8,
}

fn parse_bool(v: &[u8]) -> bool {
    match str::parse(unsafe { from_utf8_unchecked(&v.to_ascii_lowercase()) }) {
        Ok(x) => x,
        Err(_) => panic!("Parse error"),
    }
}
fn parse_hexbyte(v: &[u8]) -> u8 {
    let mut a = 0u8;
    for i in &v[0..2] {
        a *= 0x10u8;
        a += match i {
            b'0'..=b'9' => i - b'0',
            b'a'..=b'f' => i - b'a' + 10u8,
            b'A'..=b'F' => i - b'A' + 10u8,
            _ => panic!("Parse error"),
        }
    }
    a
}
fn parse_1byte(v: &[u8]) -> u8 {
    parse_hexbyte(&v[3..=4])
}
fn parse_str(v: &[u8]) -> BString {
    BString::from(&v[1..v.len() - 1])
}
fn parse_bytes<const N: usize>(v: &[u8]) -> [u8; N] {
    let mut ret = [0u8; N];
    for (i, x) in ret.iter_mut().enumerate() {
        *x = parse_hexbyte(&v[4 * i + 3..=4 * i + 4]);
    }
    ret
}
fn parse_mac(v: &[u8]) -> [u8; 6] {
    let mut ret = [0u8; 6];
    for (i, x) in ret.iter_mut().enumerate() {
        *x = parse_hexbyte(&v[2 * i + 2..=2 * i + 3]);
    }
    ret
}

pub fn do_config_parse(config_reader: &mut impl BufRead) -> Result<Config> {
    let mut config = Config::default();
    for ln in config_reader.split(b'\n').map(|l| l.unwrap()) {
        let ln = BString::from(ln);
        let (k, v) = ln.split_at(
            ln.find(b"=")
                .context(format!("Bad config format: {}", ln))?,
        );
        let k = k.trim();
        let v = v[1..].trim();
        match k {
            b"server" => config.server = parse_str(v),
            b"username" => config.username = parse_str(v),
            b"password" => config.password = parse_str(v),
            b"CONTROLCHECKSTATUS" => config.controlcheckstatus = parse_1byte(v),
            b"ADAPTERNUM" => config.adapternum = parse_1byte(v),
            b"host_ip" => config.host_ip = parse_str(v),
            b"IPDOG" => config.ipdog = parse_1byte(v),
            b"host_name" => config.host_name = parse_str(v),
            b"PRIMARY_DNS" => config.primary_dns = parse_str(v),
            b"dhcp_server" => config.dhcp_server = parse_str(v),
            b"AUTH_VERSION" => config.auth_version = parse_bytes(v),
            b"mac" => config.mac = parse_mac(v),
            b"host_os" => config.host_os = parse_str(v),
            b"KEEP_ALIVE_VERSION" => config.keep_alive_version = parse_bytes(v),
            b"ror_version" => config.ror_version = parse_bool(v),
            b"keepalive1_mod" => config.keepalive1_mod = parse_bool(v),
            b"pppoe_flag" => config.pppoe_flag = parse_1byte(v),
            b"keep_alive2_flag" => config.keep_alive2_flag = parse_1byte(v),
            _ => println!("Unknown config: {}", ln),
        }
    }
    Ok(config)
}

pub fn config_parse(filepath: &PathBuf) -> Result<Config> {
    let config_file = File::open(filepath).context(format!(
        "Cannot open config file: {}",
        filepath.to_string_lossy()
    ))?;
    let mut config_reader = BufReader::new(config_file);
    let config_ret = do_config_parse(&mut config_reader)?;

    Ok(config_ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_parse_test() {
        let config_content = br#"
server = '192.168.1.14'
username = 'a'
password = 'a'
CONTROLCHECKSTATUS = '\x20'
ADAPTERNUM = '\x01'
host_ip = '10.30.22.17'
IPDOG = '\x01'
host_name = 'LIYUANYUAN'
PRIMARY_DNS = '114.114.114.114'
dhcp_server = '0.0.0.0'
AUTH_VERSION = '\x0A\x00'
mac = 0xb888e3051680
host_os = '8089D'
KEEP_ALIVE_VERSION = '\xDC\x02'
ror_version = True
keepalive1_mod = True
pppoe_flag = '\x18'
keep_alive2_flag = '\xd8'
"#;
        let config_content = config_content.trim_ascii_start();
        let mut config_reader = BufReader::new(config_content);
        let config = do_config_parse(&mut config_reader).unwrap();
        assert_eq!(
            config,
            Config {
                server: b"192.168.1.14".into(),
                username: b"a".into(),
                password: b"a".into(),
                controlcheckstatus: 0x20,
                adapternum: 0x01,
                host_ip: b"10.30.22.17".into(),
                ipdog: 0x01,
                host_name: b"LIYUANYUAN".into(),
                primary_dns: b"114.114.114.114".into(),
                dhcp_server: b"0.0.0.0".into(),
                auth_version: *b"\x0A\x00",
                mac: *b"\xb8\x88\xe3\x05\x16\x80",
                host_os: b"8089D".into(),
                keep_alive_version: *b"\xDC\x02",
                ror_version: true,
                keepalive1_mod: true,
                pppoe_flag: 0x18,
                keep_alive2_flag: 0xd8
            }
        );
    }
}
