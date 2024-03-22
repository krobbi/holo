# Holo
_Basic HTTP server for local hosting._  
__Copyright &copy; 2024 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
   * [Arguments](#arguments)
   * [Options](#options)
   * [Examples](#examples)
2. [Limitations](#limitations)
3. [Dependencies](#dependencies)
4. [License](#license)

# Usage
Holo is a basic HTTP server targeted at serving local files for testing.
Because of its focus on simplicity and privacy, it has some
[limitations](#limitations) that may prevent it from doing what you want it to
do.

Build Holo with `cargo build --release` and move the executable from
`target/release/holo(.exe)` to a directory with environment access. After this
you can use Holo from the command line:
```shell
holo [OPTIONS] [root]
```

By default, Holo will serve files from the current working directory at
`http://localhost:8080/`. Holo will continue running until `Ctrl+C` is used to
exit.

## Arguments
| Argument | Usage                              |
| :------- | :--------------------------------- |
| `[root]` | Server root directory [default: .] |

Holo can be given an optional argument representing the path to serve files
from. The path must be an existing directory. If no path is given, the current
working directory will be used.

## Options
| Short | Long        | Arguments | Usage                                |
| :---- | :---------- | :-------- | :----------------------------------- |
| `-p`  | `--port`    | `<port>`  | TCP port [default: 8080]             |
| `-i`  | `--index`   | _(None)_  | Serve automatic index pages          |
| `-c`  | `--cors`    | _(None)_  | Enable cross-origin resource sharing |
| `-h`  | `--help`    | _(None)_  | Print help                           |
| `-V`  | `--version` | _(None)_  | Print version                        |

If the `--port` option is not set, the default port of `8080` will be used.
Ports below `1024` are likely to be reserved or require administrator
privileges.

If the `--index` flag is set, an automatic index page listing directories and
files will be served when a directory without an `index.html` file is
requested. If the flag is not set, a 404 page will be served instead.

If the `--cors` flag is set, resources from outside the host machine will be
allowed in webpages, but some JavaScript features will be disabled.

If the `--help` or `--version` flags are set, Holo will print information but
not perform any action.

## Examples
Serve files in the current working directory on port `8080`:
```shell
holo
```

Serve files in `etc/builds/web/` on port `8080`:
```shell
holo etc/builds/web/
```

Serve files in `files/` on port `8080` with automatic index pages:
```shell
holo files -i
```

Serve files in `C:\htdocs\` on port `80` with cross-origin resources:
```shell
holo C:\htdocs -p 80 -c
```

# Limitations
* Clients from outside the host machine will only be served 403 pages. This is
an intentional design choice to improve privacy. An option to open servers to
LAN or the internet is being considered.
* Holo does not support HTTPS or any kind of content encoding.
* Holo runs on a single thread. HTTP requests are queued up and must be handled
one at a time.
* Request methods and headers are ignored.

# Dependencies
Holo uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [new_mime_guess](https://crates.io/crates/new_mime_guess) - Media type
inference.
* [percent-encoding](https://crates.io/crates/percent-encoding) - URL encoding
and decoding.

# License
Holo is released under the MIT License:  
https://krobbi.github.io/license/2024/mit.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
