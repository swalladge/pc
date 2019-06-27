# pc

pc (paste-client) is a command line tool for uploading text to a pastebin
server. It supports many different pastebin style servers, and is highly
configurable.  It aims to be simple, work with stdin/stdout, and adhere to the
unix philosophy.

Please note that until v1 is released, the command line args api and config
file specification should be considered unstable. Anything could change without
notice. Please avoid using this non-interactively if you plan to update it
often and do not want to debug failures because of api changes.

## Features

- Many supported servers. If your favourite pastebin isn't supported, please
  open an issue.
- Configuration file for providing defaults and server configurations.
- Comprehensive command line help. Quickly list configured servers, backends,
  full config, detailed backend information.
- Comprehensive graceful error handling.
- Baked in, sane defaults for use without config file.
- Optional paste url history.
- Optional arguments for servers that support extra features, such as title,
  filetype, private pastes, expire time, etc.


## Installation

Build from source:

```
git clone git@github.com:swalladge/pc.git
cd pc
cargo build --release
./target/release/pc --help
# copy the binary ^ to your path to use from anywhere
```

Install from crates.io with cargo:

```
cargo install pc
```

Arch user repository: [pc-git](https://aur.archlinux.org/packages/pc-git/),
[pc](https://aur.archlinux.org/packages/pc/).


## Usage examples

Simplest, out of the box usage:

```
$ echo "Hello" | pc
https://paste.rs/saC
```

Select a custom server:

```
$ pc vpaste < code.txt
http://vpaste.net/example
```

Each configured server accepts cli args to override defaults, depending on
which backend is used. Here, the `fedora` server block uses the `modern_paste`
backend, which allows setting a custom title for the paste.
Note: can only use server-specific args if specifying the server on the cli.

```
$ pc fedora --title "foo debug log" < debug.log
https://paste.fedoraproject.org/paste/7Taaazf88VimfqOnriOsFg
```

To see a server block's configuration, backend, and allowed args:

```
$ pc fedora --help
[servers.fedora]
backend = "modern_paste"
url = "https://paste.fedoraproject.org/"
title = "my paste"
expires = "10m"
syntax = "python"
password = "password123"
apikey = "BbK1F09sZZXL2335iqDGvGeQswQUcvUmzxMoWjp3yvZDxpWwRiP4YQL6PiUA8gy2"
---

modern_paste backend

USAGE:
    fedora [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --apikey <apikey|NONE>        Upload paste as authenticated user
    -e, --expires <duration|NONE>     Time to live as a duration
    -P, --password <password|NONE>    Protects paste access with a password
    -s, --syntax <filetype|NONE>      Filetype for syntax highlighting
    -t, --title <title|NONE>          Title for the paste
    -u, --url <url>                   Overrides url set in config
```

Show a concise list of configured servers available to use:

```
$ pc list
rs => paste_rs | https://paste.rs/ [default]
vpaste => vpaste | http://vpaste.net/
haste => haste | https://hastebin.com/
...
```

List all supported backends:

```
$ pc list-backends
paste_rs
haste
vpaste
...
```

Show info and configuration help for a particular backend:

```
$ pc show-backend fiche
Fiche backend.
Supports any servers running fiche <https://github.com/solusipse/fiche>.
(for example: termbin.com)

Example config block:

    [servers.termbin]
    backend = "fiche"
    url = "termbin.com"

    # Optional values

    # default port if missing is 9999
    port = 9999

```

Dump the current config as interpreted. Helpful for debugging.

```
$ pc dump-config
<toml config as currently used>
```

Copy the default config to the user config file. Could be useful for first
setup (although the [example config](./example_config.toml) with comments may
be more helpful).

```
$ mkdir -p ~/.config/pc/ && pc -c NONE dump-config > ~/.config/pc/config.toml
```

Histfile feature can also be disabled temporarily with args:

```
$ echo "hi" | pc --histfile NONE
http://vpaste.net/example
```


## Configuration

Configuration is via a toml file. The configuration file is determined by the
following:

1. file given to the `-c` command line arg
  a) if filename is `NONE`, skip to 5
  b) if file isn't found, exit with error
2. otherwise use `$XDG_CONFIG_HOME/pc/config.toml` if exists
3. otherwise use `$HOME/.config/pc/config.toml` if exists
5. finally, no files found; use defaults (see what defaults
   are with `pc -c NONE dump-config` or see the default config file in this
   repo.

See [example_config.toml](./example_config.toml) for an example config file
with an exhaustive listing of options.  See also
[default_config.toml](./default_config.toml), which gets compiled into the
binary and used as the default config if no config file found.


## Supported server backends

| server spec                                                         | backend         | example instance                 |
| ------                                                              | -------         | ---------------                  |
| [bpaste](https://bpaste.net/)                                       | `bpaste`        | https://bpaste.net/              |
| [dpaste.com](http://dpaste.com/api/v2/)                             | `dpaste_com`    | http://dpaste.com/               |
| [dpaste](https://github.com/bartTC/dpaste)\*                        | `dpaste`        | https://dpaste.de/               |
| [fiche](https://github.com/solusipse/fiche)\*                       | `fiche`         | https://termbin.com/             |
| [haste](https://github.com/seejohnrun/haste-server)\*               | `haste`         | https://hastebin.com/            |
| [ix.io](http://ix.io/)                                              | `ix`            | http://ix.io/                    |
| [modern paste](https://github.com/LINKIWI/modern-paste)\*           | `modern_paste`  | https://paste.fedoraproject.org/ |
| [one-time secret](https://github.com/onetimesecret/onetimesecret)\* | `onetimesecret` | https://onetimesecret.com/       |
| [p.ip.fi](http://p.ip.fi/)                                          | `pipfi`         | http://p.ip.fi/                  |
| [paste.rs](https://paste.rs/web)                                    | `paste_rs`      | https://paste.rs/                |
| [sprunge](https://github.com/rupa/sprunge)\*                        | `sprunge`       | http://sprunge.us/               |
| [ubuntu](https://paste.ubuntu.com/)                                 | `ubuntu`        | https://paste.ubuntu.com/        |
| [vpaste](http://pileus.org/tools/vpaste)\*                          | `vpaste`        | http://vpaste.net/               |

\*: open source; possible to self-host or find alternate public servers.

See the [server list](docs/server-list.md) for a list of public server instances supported.

See also the [feature matrix page](docs/feature-matrix.md) for which features
each backend supports.


## Development

Standard cargo project. `cargo build`, `cargo run`, et al.

There is a Makefile for some other common tasks. Eg. `make fmt` will run
rustfmt on all source files. `make test` will run some basic tests.


## Related projects

- [pastebinit](https://launchpad.net/pastebinit): similar concept, supports
  config file, several pastebin servers, and options like private paste, set
  title, etc. An advantage is that this is available in several distros'
  official repos.
- curl: most lightweight pastebins support uploading text in a curl-friendly
  way. Eg. `<command> | curl -F 'sprunge=<-' http://sprunge.us` for sprunge.
  This is the simplest and most lightweight option, but requires remembering
  the specific curl arguments or setting up a shell alias for easy use.
- server/site-specific clients include: [ix](http://ix.io/client),
  [gist](https://github.com/defunkt/gist),
  [fb](https://git.server-speed.net/users/flo/fb/) (paste.xinu.at), ...

See the [archwiki list of pastebin clients](https://wiki.archlinux.org/index.php/List_of_applications/Internet#Pastebin_clients) for other related projects.


## License

Copyright © 2019 Samuel Walladge

Dual licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
