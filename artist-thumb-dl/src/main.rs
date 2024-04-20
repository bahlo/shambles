use std::{
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let artist_name = cwd
        .file_stem()
        .expect("Could not derive artist from directory")
        .to_str()
        .expect("Invalid directory name");
    let additional_path = std::env::args().nth(1);

    let url = format!(
        "https://music.apple.com/de/search?l=en&term={}",
        artist_name
    );
    let html_str = ureq::get(&url).call()?.into_string()?;

    let document = Html::parse_document(&html_str);
    let selector = Selector::parse(r#"picture source[type="image/jpeg"]"#).unwrap();

    let source = document.select(&selector).next().expect("No image found!");
    let srcset = source.attr("srcset").expect("No srcset attr found");
    let first_image_path = srcset.split(" ").next().expect("No images found!");

    let new_url = Path::new(first_image_path).with_file_name("1024x1024bb-999.jpg");
    let mut image_reader = ureq::get(new_url.to_str().expect("Invalid URL"))
        .call()?
        .into_reader();
    let mut image_writer = File::create("thumb.jpg")?;
    io::copy(&mut image_reader, &mut image_writer)?;

    let additional_thumb = additional_path
        .map(|path| {
            format!(
                r#"
  <thumb>{}/{}/thumb.jpg</thumb>"#,
                path, artist_name
            )
        })
        .unwrap_or_default();
    let xml = format!(
        r#"<artist>
  <thumb>thumb.jpg</thumb>{}
</artist>"#,
        additional_thumb
    );
    let mut xml_writer = File::create("artist.nfo")?;
    xml_writer.write_all(xml.as_bytes())?;

    Ok(())
}
