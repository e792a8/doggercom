# doggercom

[dogcom](https://github.com/mchome/dogcom) yet [RIIR](https://github.com/ansuz/RIIR).

- PPPoE mode is untested
- 802.1x is unimplemented
- NEW! specifying preconfigured variant (currently supports JLU)

```
Usage: doggercom [OPTIONS] --conf <FILEPATH> <--mode <MODE>|--variant <VARIANT>>

Options:
     -m, --mode <MODE>        Set your dogcom mode [possible values: dhcp, pppoe]
     -c, --conf <FILEPATH>    Import configuration file
     -b, --bindip <IPADDR>    Bind your ip address [default: 0.0.0.0]
     -l, --log <LOGPATH>      Specify log file
     -d, --daemon             Set daemon flag
     -x, --802.1x             Enable 802.1x (unimplemented)
     -e, --eternal            Set eternal flag
     -v, --verbose            Set verbose flag
NEW! -t, --variant <VARIANT>  Preconfigured variant [possible values: jlu]
     -h, --help               Print help
     -V, --version            Print version
```

Config file is compatible with [drcom-generic](https://github.com/drcoms/drcom-generic).

## Example

```bash
$ doggercom -m dhcp -c dogcom.conf
$ doggercom -m dhcp -c dogcom.conf -l /tmp/dogcom.log -v
$ doggercom -m dhcp -c dogcom.conf -d
$ doggercom -m dhcp -c dogcom.conf -b 10.2.3.12 -v
# NEW! With preconfigured variant
$ doggercom -t jlu -c dogcom-jlu.conf
# PPPoE mode is untested
$ doggercom -m pppoe -c dogcom.conf -x # -x currently not implemented
$ doggercom -m pppoe -c dogcom.conf -e # eternal doggercoming
$ doggercom -m pppoe -c dogcom.conf -v
```

## Preconfigured Variant

Currently supports JLU. The [embedded preconfiguration values](src/preconfig-jlu.conf) come from [drcoms/jlu-drcom-client/jlu-drcom-py3](https://github.com/drcoms/jlu-drcom-client/blob/2ba09ce24041c4ab7021ddbc07a366e9ae3e1c5d/jlu-drcom-py3/newclinet-py3.py).

By specifying `-t jlu`, the `-m` should be omitted and the configuration file can be reduced:

```text
username = 'your-username'
password = 'your-password'
mac = 0xMACADDRESS
host_ip = 'your.ip.addr.ess'
```

Note that values specified in the config file will override preconfigured ones.

## Build

To build a statically linked binary for some target (armv7-unknown-linux-gnueabihf for me):

```bash
# requires armv7l-linux-gnueabihf-gcc
# rustup target add armv7-unknown-linux-gnueabihf
RUSTFLAGS='-C linker=armv7l-linux-gnueabihf-gcc -C target-feature=+crt-static' cargo build --release --target=armv7-unknown-linux-gnueabihf
```
