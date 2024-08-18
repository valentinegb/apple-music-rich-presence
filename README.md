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
cargo install --git https://github.com/valentinegb/apple-music-rich-presence.git --tag v1.0.0
```

The program has only been tested on macOS, and it probably only works on macOS.
