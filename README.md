# process-killer

![Rust](https://github.com/nygrenh/process-killer/workflows/Rust/badge.svg)

A simple utility for for terminating processes quickly and cleanly. It first tries to terminate all processes that match the given substring gracefully with `SIGTERM` and terminates the ones that don't shut down with `SIGKILL`.

## Installation

Download binary from Github Actions artifacts. Only Linux binaries are provided for now.

## Usage

```
USAGE:
    process-killer <process-name-substring>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <process-name-substring>    All processes that contain this substring will be killed. Case insensitive.
```
