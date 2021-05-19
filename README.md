# Another S3 command line utility

A quick and _extremely_ simple utility based on [durch/rust-s3](https://github.com/durch/rust-s3) for interacting with S3 compatible object stores because all the other tools I could find needed _configuration_ and I'm not into it.

## Status
![Build](https://github.com/ryankurte/s3-util/workflows/Rust/badge.svg)
[![GitHub tag](https://img.shields.io/github/tag/ryankurte/s3-util.svg)](https://github.com/ryankurte/s3-util)
[![Crates.io](https://img.shields.io/crates/v/s3-util.svg)](https://crates.io/crates/s3-util)
[![Docs.rs](https://docs.rs/s3-util/badge.svg)](https://docs.rs/s3-util)


## Usage

Install via `cargo install s3-util` to build from source, `cargo binstall s3-util` to fetch a binary if you have [`cargo-binstall`](https://github.com/ryankurte/cargo-binstall) available, or grab a pre-build binary from the [releases](https://github.com/ryankurte/s3-util/releases/latest) or a docker container from `ghcr.io/ryankurte/s3-util`.

Examples:

- `s3-util upload dir/something.txt ./localfile.txt` to upload `localfile.txt` to `dir/something.txt` in the bucket
- `s3-util upload dir/something.txt ./*.txt` to glob for a file matching `./*.txt` and upload to `dir/something.txt` in the bucket
- `s3-util upload-dir prefix/ ./*.txt` to upload each file matching `./*.txt` with the prefix `prefix/`


See `s3-util [SUBCOMMAND] --help` for more information, you need to configure _all_ the options as appropriate for your object-storage provider (specifically: `ACCESS_KEY`, `S3_BUCKET`, `S3_ENDPOINT`, `S3_REGION` and `SECRET_KEY`).

```
s3-util 0.1.0

USAGE:
    s3-util [OPTIONS] --access-key <access-key> --bucket <bucket> --endpoint <endpoint> --region <region> --secret-key <secret-key> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --access-key <access-key>    Access key for bucket [env: ACCESS_KEY=]
        --bucket <bucket>            Bucket name [env: S3_BUCKET=]
        --endpoint <endpoint>        Bucket endpoint (eg. amazonaws.com) [env: S3_ENDPOINT=]
        --log-level <log-level>       [default: info]
        --region <region>            Bucket region (eg. s3-ap-northeast-1) [env: S3_REGION=]
        --secret-key <secret-key>    Secret key for bucket [env: SECRET_KEY=]

SUBCOMMANDS:
    delete      Delete an item from the bucket
    download    Download an item from the bucket
    help        Prints this message or the help of the given subcommand(s)
    list        Show items in bucket
    upload      Upload an item to the bucket
    upload-dir    Upload files from a directory
```


## Alternatives

- [s3cmd](https://github.com/s3tools/s3cmd)
