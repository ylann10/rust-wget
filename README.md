# Rust-Wget

Since Linux "wget" functionality is missing from Windows, here is a lightweight version coded in Rust.

## Prerequisites

Before you begin, ensure you have met the following requirements:
* Rust >= 1.58.0

## Installing Rust-Wget

To install rust-wget, follow these steps:

```
cargo build --release
```

## Using Rust-Wget

To use rust-wget, follow these steps:

```
Usage:
  wget [OPTIONS] [URL]

Functionality similar to "wget" in Linux

Positional arguments:
  url                   URL of the file to download

Optional arguments:
  -h,--help             Show this help message and exit
  -o,--output OUTPUT    Location of downloaded file
```
