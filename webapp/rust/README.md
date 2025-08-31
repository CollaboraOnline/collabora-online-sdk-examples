Please see [../README.md](../README.md) for usage and general information.

## Requirements

This example has been tested using [Rust](https://www.rust-lang.org/) version 1.85.0 on Debian Linux 13.

## Run the example

`cargo run`

This example offers WOPI-Proof validation and outputs the results to STDOUT.  
To use WOPI-Proof when running behind a reverse HTTP proxy, set the following environment variable like this.  
`REVERSE_PROXY_URL_PREFIX="https://public-name.example.org/subdir"`  
Do NOT append a trailing slash!
