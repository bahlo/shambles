# artist-thumb-dl

A CLI (if you can call it that), which scrapes the iTunes artist image for
the givend directory name and writes it to `thumb.jpg`.
It also writes a `artist.nfo` with the filename and an optional absolute path.

I wrote this to download images for all artists in Jellyfin.

## Installation

```sh
cargo install artist-thumb-dl --git https://github.com/bahlo/shambles
```

## Usage

```sh
$ cd "Taylor Swift" # or whatever
$ artist-thumb-dl
$ ls
thumb.jpg artist.nfo
$ cat artist.nfo
<artist>
  <thumb>thumb.jpg</thumb>
</artist>
```

Jellyfin and possibly other systems need an absolute path, so passing that
will add a second `<thumb>` entry:

```sh
$ cd "Taylor Swift" # or whatever
$ artist-thumb-dl /path/to/my/music
$ ls
thumb.jpg artist.nfo
$ cat artist.nfo
<artist>
  <thumb>thumb.jpg</thumb>
  <thumb>/path/to/my/music/Taylor Swift/thumb.jpg</thumb>
</artist>
```
