# copostr

A simple cohost bot for posting images periodically.

## Usage

### Requirements

- Python 3
    - `flickr_api` from pypi
- Rust
    - `openssl-devel` for build, `openssl` for runtime

### Running

1. Generate an image index. See [here](./flickr-indexer/README.md) for how to do so with flickr.
2. Build the bot with `cargo build`.
3. Run it with a cron daemon, systemd timer, or other scheduling method. The bot will post one image per invocation.

## Licence

This project is licenced under the [BSD-2-Clause Licence](./LICENCE).
