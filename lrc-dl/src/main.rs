use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};
use serde::Deserialize;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrcResponse {
    instrumental: bool,
    plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_str = std::env::args().nth(1).expect("ERROR: No path specified!");
    let path = Path::new(&path_str);

    if !path.is_file() {
        panic!("ERROR: Path is not a file!");
    }

    let tagged_file = Probe::open(path)?.read()?;

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        // If the "primary" tag doesn't exist, we just grab the
        // first tag we can find. Realistically, a tag reader would likely
        // iterate through the tags to find a suitable one.
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    println!("--- Tag Information ---");
    let tag_title = tag.title();
    let track_name = tag_title.as_deref().unwrap_or("None");
    println!("Title: {}", track_name);
    let tag_artist = tag.artist();
    let artist_name = tag_artist.as_deref().unwrap_or("None");
    println!("Artist: {}", artist_name);
    let tag_album = tag.album();
    let album_name = tag_album.as_deref().unwrap_or("None");
    println!("Album: {}", album_name);

    let properties = tagged_file.properties();
    let duration = properties.duration();
    let seconds = duration.as_secs() % 60;
    let duration_display = format!("{:02}:{:02}", (duration.as_secs() - seconds) / 60, seconds);
    println!("Duration: {duration_display}");

    let res = ureq::get("https://lrclib.net/api/get")
        .set("User-Agent", "lrc-dl 0.1.0 (unpublished)")
        .query("track_name", track_name)
        .query("artist_name", artist_name)
        .query("album_name", album_name)
        .query("duration", duration.as_secs().to_string().as_str())
        .call()?;
    if res.status() == 404 {
        println!("No lyrics found for this track");
    } else if res.status() != 200 {
        panic!("Failed to fetch lyrics: {}", res.status());
    }

    let track: LrcResponse = res.into_json()?;
    if track.instrumental {
        println!("Instrumental track detected, skipping");
        return Ok(());
    }

    if let Some(synced_lyrics) = track.synced_lyrics {
        println!("{}", synced_lyrics);
        let mut lrc_path = PathBuf::from(path);
        lrc_path.set_extension("lrc");
        let mut file = File::create(lrc_path)?;
        file.write_all(synced_lyrics.as_bytes())?;
        return Ok(());
    }

    if let Some(plain_lyrics) = track.plain_lyrics {
        println!("{}", plain_lyrics);
        let mut txt_path = PathBuf::from(path);
        txt_path.set_extension("txt");
        let mut file = File::create(txt_path)?;
        file.write_all(plain_lyrics.as_bytes())?;
        return Ok(());
    }

    Ok(())
}
