use adw::prelude::*;
use relm4::{adw, gtk, prelude::FactoryComponent};

use crate::config::Account;

use super::account_list::AccountListInput;

pub struct AccountListItem {
    account: Account,
}

#[relm4::factory(pub)]
impl FactoryComponent for AccountListItem {
    type Init = Account;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;
    type ParentInput = AccountListInput;

    view! {
        adw::ActionRow {
            #[watch]
            set_title: &self.account.username,
            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { account: init }
    }
}
