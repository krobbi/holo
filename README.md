# Holo
_Basic HTTP server for local hosting._  
__Copyright &copy; 2024 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
   * [Options](#options)
2. [Dependencies](#dependencies)
3. [License](#license)

# Usage
Holo is a basic HTTP server targeted at serving local files for testing.

Build Holo with `cargo build --release` and move the executable from
`target/release/holo(.exe)` to a directory with environment access. After this
you can use Holo from the command line:
```shell
holo [OPTIONS]
```

Holo will serve files from the current working directory at
`http://localhost:8080/` until `Ctrl+C` is used to exit.

## Options
| Short | Long        | Value    | Usage                         |
| :---- | :---------- | :------- | :---------------------------- |
| `-p`  | `--port`    | `<PORT>` | TCP port [default: 8080]      |
| `-i`  | `--coi`     | _(None)_ | Enable cross-origin isolation |
| `-h`  | `--help`    | _(None)_ | Print help                    |

If the `--port` option is not set, the default port of `8080` will be used.
Ports below `1024` are likely to be reserved or require administrator
privileges.

If the `--coi` flag is set, additional HTTP headers will be served to enable
cross-origin isolation. Cross-origin isolation may not function correctly on
local servers in some browsers.

If the `--help` flags is set, Holo will print help information but not perform
any action.

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
