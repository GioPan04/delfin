use anyhow::{Ok, Result};

use crate::jellyfin_api::{
    api_client::ApiClient,
    models::display_preferences::{DisplayPreferences, DisplayPreferencesRaw},
};

impl ApiClient {
    pub async fn get_user_display_preferences(&self, client: &str) -> Result<DisplayPreferences> {
        let mut url = self.root.join("DisplayPreferences/usersettings")?;

        url.query_pairs_mut()
            .append_pair("userId", &self.account.id.to_string())
            .append_pair("client", client);

        let res: DisplayPreferencesRaw = self.client.get(url).send().await?.json().await?;

        let display_preferences = res.into();

        Ok(display_preferences)
    }
}
