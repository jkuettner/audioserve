#![allow(dead_code)]

pub const ALBUM: &str = "album";
pub const ALBUM_ARTIST: &str = "album_artist";
pub const ARTIST: &str = "artist";
pub const COMMENT: &str = "comment";
pub const COMPOSER: &str = "composer";
pub const COPYRIGHT: &str = "copyright";
pub const CREATION_TIME: &str = "creation_time";
pub const DATE: &str = "date";
pub const DISC: &str = "disc";
pub const ENCODER: &str = "encoder";
pub const ENCODED_BY: &str = "encoded_by";
pub const FILENAME: &str = "filename";
pub const GENRE: &str = "genre";
pub const LANGUAGE: &str = "language";
pub const PERFORMER: &str = "performer";
pub const PUBLISHER: &str = "publisher";
pub const SERVICE_NAME: &str = "service_name";
pub const SERVICE_PROVIDER: &str = "service_provider";
pub const TITLE: &str = "title";
pub const TRACK: &str = "track";
pub const VARIANT_BITRATE: &str = "variant_bitrate";

pub const ALLOWED_TAGS: &[&str] = &[
    ALBUM,
    ALBUM_ARTIST,
    ARTIST,
    COMMENT,
    COMPOSER,
    COPYRIGHT,
    CREATION_TIME,
    DATE,
    DISC,
    ENCODER,
    ENCODED_BY,
    FILENAME,
    GENRE,
    LANGUAGE,
    PERFORMER,
    PUBLISHER,
    SERVICE_NAME,
    SERVICE_PROVIDER,
    TITLE,
    TRACK,
    VARIANT_BITRATE,
];

pub fn print_tags_help() {
    print!("
You can define metadata tags, that will be collected from audiofiles and presented via API with folder information.
Tags that will be same for all audiofiles in folder will be available on folder level, tags that differs per file
will be present on file level. Tags, you'd like to collect and present should be listed via --tags argument, 
separated by comma. 

Available tags are: 
");

    print!(
        "{}",
        ALLOWED_TAGS
            .into_iter()
            .map(|r| *r)
            .collect::<Vec<_>>()
            .join("\n")
    );
}