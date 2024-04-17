# lrc-dl

A CLI that, given an audio file, downloads its lyrics from
[lrclib.net](https://lrclib.net) with the same filename in the same directory,
but with either `.lrc` or `.txt`.

I wrote this to download lyrics for my music and be ready for the upcoming
[lyrics support in Jellyfin](https://github.com/jellyfin/jellyfin/pull/8381).

## Usage

```
$ lrc-dl /absolute/path/to/audiofile.flac
```
