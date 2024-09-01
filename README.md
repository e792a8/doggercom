# doggercom

[dogcom](https://github.com/mchome/dogcom) implementation in Rust.

```
Usage:
        doggercom -m <dhcp/pppoe> -c <FILEPATH> [options <argument>]...

Options:
        --mode <dhcp/pppoe>, -m <dhcp/pppoe>  set your dogcom mode
        --conf <FILEPATH>, -c <FILEPATH>      import configuration file
        --bindip <IPADDR>, -b <IPADDR>        bind your ip address(default is 0.0.0.0)
        --log <LOGPATH>, -l <LOGPATH>         specify log file
        --802.1x, -x                          enable 802.1x
        --daemon, -d                          set daemon flag
        --eternal, -e                         set eternal flag
        --verbose, -v                         set verbose flag
        --help, -h                            display this help
```

Config file is compatible with [drcom-generic](https://github.com/drcoms/drcom-generic).

#### Example:

```bash
$ doggercom -m dhcp -c dogcom.conf
$ doggercom -m dhcp -c dogcom.conf -l /tmp/dogcom.log -v
$ doggercom -m dhcp -c dogcom.conf -d # (PS: only on Linux build)
$ doggercom -m pppoe -c dogcom.conf -x # (PS: only on Linux build)
$ doggercom -m pppoe -c dogcom.conf -e # eternal dogcoming (default times is 5)
$ doggercom -m pppoe -c dogcom.conf -v
$ doggercom -m dhcp -c dogcom.conf -b 10.2.3.12 -v
```
