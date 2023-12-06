use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum CollectionType {
    Movies,
    TvShows,
    Music,
    Playlists,
    Other,
}

impl From<Option<String>> for CollectionType {
    fn from(value: Option<String>) -> Self {
        match value.as_deref() {
            Some("movies") => CollectionType::Movies,
            Some("tvshows") => CollectionType::TvShows,
            Some("music") => CollectionType::Music,
            Some("playlists") => CollectionType::Playlists,
            Some(_) | None => CollectionType::Other,
        }
    }
}

impl Display for CollectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Movies => "Movies",
                Self::TvShows => "Shows",
                Self::Music => "Music",
                Self::Playlists => "Playlists",
                Self::Other => "Folder",
            }
        )
    }
}

impl CollectionType {
    pub fn icon(&self) -> String {
        match self {
            Self::Movies => "video-clip-multiple-filled",
            Self::TvShows => "video-clip-multiple-filled",
            Self::Music => "play-multiple-filled",
            Self::Playlists => "tag-multiple-filled",
            Self::Other => "folder-filled",
        }
        .to_string()
    }
}
