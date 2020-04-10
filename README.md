# process-killer

![Rust](https://github.com/nygrenh/process-killer/workflows/Rust/badge.svg)

A simple utility for for terminating processes quickly and cleanly. It first tries to terminate all processes that match the given substring gracefully with `SIGTERM` and terminates the ones that don't shut down with `SIGKILL`.

## Installation

Download binary from Github Actions artifacts. Only Linux binaries are provided for now.

## Usage

```
USAGE:
    process-killer [OPTIONS] <process-name-substring>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --wait-ms <wait-ms>    How many milliseconds to wait for the processes to gracefully terminate
                               before force killing them. [default: 3000]

ARGS:
    <process-name-substring>    All processes that contain this substring will be killed. Case insensitive.
```
