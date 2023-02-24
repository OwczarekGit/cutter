# cutter

A program for cutting media using ffmpeg into slices.

### Usage

```sh
cutter <INPUT> <OUTPUT_FORMAT> [TIMESTAMPS]...
```

For example to cut file `songs.mp3` into N+1 parts you would run:
```sh
cutter songs.mp3 mp3 00:02:44 00:03:32 00:05:12
```

This will create four .mp3 files named `1.mp3, 2.mp3, 3.mp3, 4.mp3`

Visually it basically splits it like that:

Output files      1.mp3      2.mp3      3.mp3     4.mp3

-                         /           \ /               \ /           \ /               \

Input Media   [.................................................................]

TIMESTAMPS                ↑                 ↑             ↑

### Building
```sh
cargo build
```

You'll need ffmpeg installed to actually use it.
