use anyhow::bail;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::tr;

use super::collection_type::CollectionType;

#[derive(Clone, Debug)]
pub struct UserView(BaseItemDto);

impl UserView {
    pub fn id(&self) -> Uuid {
        self.0.id.unwrap()
    }

    pub fn name(&self) -> String {
        self.0
            .name
            .clone()
            .unwrap_or(tr!("library-unnamed-collection").to_string())
    }

    pub fn collection_type(&self) -> CollectionType {
        self.0.collection_type.clone().into()
    }
}

impl TryFrom<BaseItemDto> for UserView {
    type Error = anyhow::Error;

    fn try_from(item: BaseItemDto) -> std::result::Result<Self, Self::Error> {
        if item.id.is_none() {
            bail!("Item was missing ID: {item:#?}");
        }
        Ok(Self(item))
    }
}

impl From<UserView> for BaseItemDto {
    fn from(val: UserView) -> Self {
        val.0
    }
}
