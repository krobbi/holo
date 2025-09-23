# Holo
Holo is a simple HTTP server targeted at serving local files for testing.
Because of its focus on simplicity and privacy, it has some
[limitations](#limitations) that may prevent it from doing what you want it to
do.

# Usage
To use Holo, build it with `cargo build --release` and move the executable from
`target/release/holo(.exe)` to a directory listed in the `PATH` environment
variable. After this, you can use Holo from the command line in any directory:
```shell
holo [OPTIONS] [ROOT]
```

By default, Holo will serve files from the current working directory at
`http://localhost:8080`. Holo will continue running until `Ctrl+C` is used to
exit.

## Arguments
| Argument | Usage                              |
| :------- | :--------------------------------- |
| `[ROOT]` | Server root directory [default: .] |

Holo can be given an optional argument representing the path to the root
directory for serving files. The path must be an existing directory. If no path
is given, then current working directory is used.

## Options
| Short | Long        | Arguments | Usage                                |
| :---- | :---------- | :-------- | :----------------------------------- |
| `-p`  | `--port`    | `<PORT>`  | TCP port [default: 8080]             |
| `-h`  | `--help`    | _(None)_  | Print help                           |
| `-V`  | `--version` | _(None)_  | Print version                        |
<!--
| `-i`  | `--index`   | _(None)_  | Serve automatic index pages          |
| `-c`  | `--cors`    | _(None)_  | Enable cross-origin resource sharing |
-->

If the `--port` option is not set, then a default port of `8080` will be used.
Ports below `1024` are likely to be reserved or require administrator
privileges. The operating system may assign a different port to the given
option (especially if it is `0`), but Holo will print a URL to connect to
including the port.

<!--
If the `--index` flag is set, an automatic index page listing directories and
files will be served when a directory without an `index.html` file is
requested. If the flag is not set, a 404 page will be served instead.

If the `--cors` flag is set, resources from outside the host machine will be
allowed in webpages, but some JavaScript features will be disabled.
-->

If the `--help` or `--version` flag is set, then Holo will print information
but not perform any action.

## Examples
Serve files from the current working directory on port `8080`:
```shell
holo
```

Serve files from `etc/builds/web/` on port `8080`:
```shell
holo etc/builds/web/
```

<!--
Serve files in `files/` on port `8080` with automatic index pages:
```shell
holo files -i
```

Serve files in `C:\htdocs\` on port `80` with cross-origin resources:
```shell
holo C:\htdocs -p 80 -c
```
-->

# Limitations
* Clients other than the host machine are served 403 error pages. This is an
intentional design choice to improve privacy.
* HTTPS and content encoding are not supported.
* Requests are handled one at a time on individual connections.
* Requests using HTTP methods other than GET are ignored.
* Request query strings and header fields are ignored.

# Dependencies
Holo uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [percent-encoding](https://crates.io/crates/percent-encoding) - URI decoding.
<!--
* [new_mime_guess](https://crates.io/crates/new_mime_guess) - Media type
inference.
-->

# License
Holo is released under the MIT License. See [LICENSE.txt](/LICENSE.txt) for a
full copy of the license text.
