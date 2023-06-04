use adw::prelude::*;
use relm4::{
    adw, gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::config::Account;

use super::account_list::AccountListInput;

pub struct AccountListItem {
    account: Account,
}

#[derive(Debug)]
pub enum AccountListItemOutput {
    AccountSelected(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for AccountListItem {
    type Init = Account;
    type Input = ();
    type Output = AccountListItemOutput;
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
            set_activatable: true,
            connect_activated[sender, index] => move |_| {
                sender.output(AccountListItemOutput::AccountSelected(index.clone()));
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

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            AccountListItemOutput::AccountSelected(index) => {
                AccountListInput::AcountSelected(index)
            }
        })
    }
}
