# lrc-dl

A CLI that, given an audio file, downloads its lyrics from
[lrclib.net](https://lrclib.net) with the same filename in the same directory,
but with either `.lrc` or `.txt`.

I wrote this to download lyrics for my music and be ready for the upcoming
[lyrics support in Jellyfin](https://github.com/jellyfin/jellyfin/pull/8381).

## Installation

```sh
cargo install lrc-dl --git https://github.com/bahlo/shambles
```

## Usage

```sh
lrc-dl /absolute/path/to/audiofile.flac
```

I always run this bash command to run this for all music files in a directory
(replace `.flag` with whatever extension your music has):

```sh
ls *.flac | while read filename; do lrc-dl "$(pwd)/$filename"; done
```
