use adw::prelude::*;
use relm4::{
    adw,
    gtk::{
        self,
        gdk::{self, Texture},
        gdk_pixbuf::Pixbuf,
    },
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

use crate::{config::Account, jellyfin_api::api::user::get_user_avatar};

pub struct AccountListItem {
    account: Account,
    avatar: Option<Texture>,
}

#[derive(Debug)]
pub enum AccountListItemOutput {
    AccountSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum AccountListItemCommandOutput {
    AvatarLoaded(Option<Box<Texture>>),
}

#[relm4::factory(pub)]
impl FactoryComponent for AccountListItem {
    type Init = (String, Account);
    type Input = ();
    type Output = AccountListItemOutput;
    type CommandOutput = AccountListItemCommandOutput;
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            #[watch]
            set_title: &self.account.username,

            add_prefix = &adw::Avatar {
                set_text: Some(&self.account.username),
                set_size: 40,
                #[watch]
                set_custom_image: self.avatar.as_ref(),
            },

            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },

            set_activatable: true,
            connect_activated[sender, index] => move |_| {
                sender.output(AccountListItemOutput::AccountSelected(index.clone()));
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        sender: relm4::FactorySender<Self>,
    ) -> Self {
        let (url, account) = init;

        try_load_avatar(sender, &url, &account.id);

        Self {
            account,
            avatar: None,
        }
    }

    fn update_cmd(&mut self, message: Self::CommandOutput, _sender: FactorySender<Self>) {
        match message {
            AccountListItemCommandOutput::AvatarLoaded(avatar) => self.avatar = avatar.map(|a| *a),
        }
    }
}

fn try_load_avatar(sender: FactorySender<AccountListItem>, server_url: &str, account_id: &str) {
    let server_url = server_url.to_string();
    let account_id = account_id.to_string();
    sender.oneshot_command(async move {
        // This errors if the user doesn't have an avatar image set
        if let Ok(avatar_bytes) = get_user_avatar(&server_url, &account_id).await {
            return match Pixbuf::from_read(avatar_bytes) {
                Ok(pixbuf) => AccountListItemCommandOutput::AvatarLoaded(Some(Box::new(
                    gdk::Texture::for_pixbuf(&pixbuf),
                ))),
                _ => return AccountListItemCommandOutput::AvatarLoaded(None),
            };
        }

        AccountListItemCommandOutput::AvatarLoaded(None)
    });
}
