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
Because of its limited scope and minimalistic implementation, it has some
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
| Short | Long        | Arguments | Usage                         |
| :---- | :---------- | :-------- | :---------------------------- |
| `-p`  | `--port`    | `<port>`  | TCP port [default: 8080]      |
| `-i`  | `--coi`     | _(None)_  | Enable cross-origin isolation |
| `-h`  | `--help`    | _(None)_  | Print help                    |

If the `--port` option is not set, the default port of `8080` will be used.
Ports below `1024` are likely to be reserved or require administrator
privileges.

If the `--coi` flag is set, additional HTTP headers will be served to enable
cross-origin isolation. Cross-origin isolation may not function correctly on
local servers in some browsers.

If the `--help` flag is set, Holo will print help information but not perform
any action.

## Examples
Serve files in the current working directory on port `8080`:
```shell
holo
```

Serve files in `C:\htdocs\` on port `80`:
```shell
holo C:\htdocs -p 80
```

Serve files in `ignore/` on port `8080` with cross-origin isolation:
```shell
holo ignore -i
```

Serve files in `etc/builds/web/` on port `8060` with cross-origin isolation:
```shell
holo etc/builds/web/ -p 8060 -i
```

# Limitations
* Holo does not redirect requests or generate index pages. Only addresses with
explicit file names should be used e.g. `http://localhost:8080/index.html`.
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
* [new_mime_guess](https://crates.io/crates/new_mime_guess) - MIME type
inference.
* [percent-encoding](https://crates.io/crates/percent-encoding) - Request URL
decoding.

# License
Holo is released under the MIT License:  
https://krobbi.github.io/license/2024/mit.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
