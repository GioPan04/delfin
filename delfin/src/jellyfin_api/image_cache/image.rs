use jellyfin_api::types::BaseItemDto;

/// A unique image on Jellfyin side.
///
/// This image should exist as it is promised by the Jellyfin API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageId {
    pub uuid: String,
    pub tag: String,
    pub kind: ImageKind,
}

impl ImageId {
    /// Request a specific [`ImageKind`] from a [`BaseItemDto`], if any.
    pub fn from_item(item: &BaseItemDto, kind: ImageKind) -> Option<ImageId> {
        let tag = item
            .image_tags
            .as_ref()
            .and_then(|tags| tags.get(kind.as_ref()))?;

        Some(ImageId {
            uuid: item.id.unwrap().to_string(),
            tag: tag.to_string(),
            kind,
        })
    }

    /// Request an item's own or inherited [`ImageKind::Backdrop`], if any.
    pub fn from_item_backdrop(item: &BaseItemDto) -> Option<ImageId> {
        let tag = item
            .backdrop_image_tags
            .as_ref()
            .and_then(|tags| tags.first())?;

        Some(ImageId {
            uuid: item.id.unwrap().to_string(),
            tag: tag.to_string(),
            kind: ImageKind::Backdrop,
        })
    }

    /// Request an item's own or inherited [`ImageKind::Thumb`], if any.
    pub fn from_item_parent_thumb(item: &BaseItemDto) -> Option<ImageId> {
        let id = item.parent_thumb_item_id?;
        let tag = item.parent_thumb_image_tag.as_ref()?;

        Some(ImageId {
            uuid: id.to_string(),
            tag: tag.to_string(),
            kind: ImageKind::Thumb,
        })
    }

    /// Request an item's parent [`ImageKind::Backdrop`], if any.
    pub fn from_item_parent_backdrop(item: &BaseItemDto) -> Option<ImageId> {
        let id = item.parent_backdrop_item_id?;
        let tag = item
            .parent_backdrop_image_tags
            .as_ref()
            .and_then(|tags| tags.first())?;

        Some(ImageId {
            uuid: id.to_string(),
            tag: tag.to_string(),
            kind: ImageKind::Backdrop,
        })
    }
}

/// The different types of images returned by Jellyfin API.
///
/// There are in fact a lot more of them, but these are not supported yet:
///
/// - Art
/// - Banner
/// - Logo
/// - Disc
/// - Box
/// - Screnshot
/// - Menu
/// - Chapter
/// - BoxRear
/// - Profile
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageKind {
    /// Poster image for an item
    Primary,
    /// Large background image for playing an item
    Backdrop,
    /// Small preview for next-up
    Thumb,
}

impl AsRef<str> for ImageKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Primary => "Primary",
            Self::Backdrop => "Backdrop",
            Self::Thumb => "Thumb",
        }
    }
}

impl std::fmt::Display for ImageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
