# doggercom

[dogcom](https://github.com/mchome/dogcom) yet [RIIR](https://github.com/ansuz/RIIR).

```
Usage: doggercom [OPTIONS] --mode <MODE> --conf <FILEPATH>

Options:
  -m, --mode <MODE>      Set your dogcom mode [possible values: dhcp, pppoe]
  -c, --conf <FILEPATH>  Import configuration file
  -b, --bindip <IPADDR>  Bind your ip address [default: 0.0.0.0]
  -l, --log <LOGPATH>    Specify log file
  -d, --daemon           Set daemon flag
  -x, --802.1x           Enable 802.1x (unimplemented)
  -e, --eternal          Set eternal flag
  -v, --verbose          Set verbose flag
  -h, --help             Print help
  -V, --version          Print version
```

Config file is compatible with [drcom-generic](https://github.com/drcoms/drcom-generic).

#### Example

```bash
$ doggercom -m dhcp -c dogcom.conf
$ doggercom -m dhcp -c dogcom.conf -l /tmp/dogcom.log -v
$ doggercom -m dhcp -c dogcom.conf -d
$ doggercom -m pppoe -c dogcom.conf -x # currently not implemented
$ doggercom -m pppoe -c dogcom.conf -e # eternal doggercoming
$ doggercom -m pppoe -c dogcom.conf -v
$ doggercom -m dhcp -c dogcom.conf -b 10.2.3.12 -v
```

#### Build

To build a statically linked binary for some target (armv7-unknown-linux-gnueabihf for me):

```bash
# requires armv7l-linux-gnueabihf-gcc
# rustup target add armv7-unknown-linux-gnueabihf
RUSTFLAGS='-C linker=armv7l-linux-gnueabihf-gcc -C target-feature=+crt-static' cargo build --release --target=armv7-unknown-linux-gnueabihf
```
