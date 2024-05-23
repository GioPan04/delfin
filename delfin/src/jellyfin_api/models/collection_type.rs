use std::fmt::Display;

use jellyfin_api::types::{BaseItemKind, CollectionType as JFCollectionType};

#[derive(Debug, Clone, Copy)]
pub enum CollectionType {
    Movies,
    TvShows,
    Music,
    Playlists,
    Other,
}

// TODO: Get rid of our own CollectionType
impl From<Option<JFCollectionType>> for CollectionType {
    fn from(value: Option<JFCollectionType>) -> Self {
        value
            .map(|value| match value {
                JFCollectionType::Movies => CollectionType::Movies,
                JFCollectionType::Tvshows => CollectionType::TvShows,
                JFCollectionType::Music => CollectionType::Music,
                JFCollectionType::Playlists => CollectionType::Playlists,
                _ => CollectionType::Other,
            })
            .unwrap_or(CollectionType::Other)
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
            Self::Movies => "movies-and-tv-filled",
            Self::TvShows => "tv-filled",
            Self::Music => "music-note-2-filled",
            Self::Playlists => "tag-multiple-filled",
            Self::Other => "folder-filled",
        }
        .to_string()
    }

    pub fn item_type(&self) -> Option<BaseItemKind> {
        match self {
            Self::Movies => Some(BaseItemKind::Movie),
            Self::TvShows => Some(BaseItemKind::Series),
            Self::Music => Some(BaseItemKind::MusicAlbum),
            Self::Playlists => Some(BaseItemKind::Playlist),
            Self::Other => None,
        }
    }
}
