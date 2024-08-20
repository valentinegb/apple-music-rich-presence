# Apple Music Rich Presence

[![Rust](https://github.com/valentinegb/apple-music-rich-presence/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/apple-music-rich-presence/actions/workflows/rust.yml)

Rust program that runs in the background and updates your Discord activity if
you're playing music with Apple Music.

<img width="295" alt="IMG_4105" src="https://github.com/user-attachments/assets/55ffd362-321e-428d-bfe5-38f2d90b168a">

<img width="295" alt="IMG_4106" src="https://github.com/user-attachments/assets/921fc130-5f36-4f61-94da-e64e34db4db3">

On desktop, Discord might display the activity like this instead:

<img width="295" alt="Screenshot 2024-08-17 at 5 40 49 PM" src="https://github.com/user-attachments/assets/c2711e08-e4cb-4280-b44a-6e6d7df43b58">

Unfortunately, there's nothing I can do about that, but at least it doesn't look bad still.

## Install

Right now, the easiest way to install if you have the Rust toolchain installed
is with Cargo:

```
cargo install --git https://github.com/valentinegb/apple-music-rich-presence.git --tag v1.1.0
```

The program has only been tested on macOS, and it probably only works on macOS.

## Usage

Just run the program (if installed through Cargo, enter
`apple-music-rich-presence` into your terminal) and you're good to go! I would
recommend adding it to your Login Items in System Settings so that you don't
have to start it manually every time you restart your computer.

### Configuration

There are a few environment variables that can be used to configure Apple Music
Rich Presence.

#### `MIN_ITER_DUR`

Apple Music Rich Presence, by default, will ensure that each iteration (where it
checks whether Discord and Apple Music are open, whether the currently playing
song has changed, etc.) spans at least 1 second. If you feel that it need more
time to prevent glitchy behaviour like Apple Music refusing to quit, you can
raise this to, say, `MIN_ITER_DUR=3`.

#### `DAEMON`

By default, Apple Music Rich Presence will launch a daemon process to run in the
background. You can disable this behaviour, however, by setting `DAEMON` to `0`.
Then, the program will stay present and you'll be able to see its logging in
your terminal.

#### `RUST_LOG`

This variable configures the level of logging to perform. See
[`env_logger`'s documentation](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)
for info.
