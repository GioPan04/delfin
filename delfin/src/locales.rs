use fluent_templates::{lazy_static::lazy_static, FluentLoader};
use sys_locale::get_locale;
use tera::Tera;
use tracing::warn;
use unic_langid::{langid, LanguageIdentifier};

use crate::globals::CONFIG;

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "../locales/",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

lazy_static! {
    pub static ref DEFAULT_LANGUAGE: LanguageIdentifier = get_locale()
        .and_then(|l| l.parse().ok())
        .unwrap_or_else(|| {
            warn!("Error parsing system locale, defaulting to en-US");
            langid!("en-US")
        });
}

pub fn current_language() -> LanguageIdentifier {
    CONFIG
        .read()
        .general
        .language
        .clone()
        .unwrap_or(DEFAULT_LANGUAGE.clone())
}

#[macro_export]
macro_rules! tr {
    ($id:expr) => {{
        use fluent_templates::Loader;
        use $crate::{locales::{LOCALES, current_language}};

        &LOCALES
            .lookup(&current_language(), $id)
    }};

    // Lookup message with arguments
    ($id:expr, {$($k:expr => $v:expr),* $(,)?}$(,)?) => {{
        use fluent_templates::Loader;
        use std::collections::HashMap;
        use $crate::{locales::{LOCALES, current_language}};

        &LOCALES.lookup_with_args(
            &current_language(),
            $id,
            &HashMap::from([$(($k, $v.into()),)*]),
        )
    }};
}

pub fn tera_tr(input: &str) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    let ctx = tera::Context::default();
    tera.register_function(
        "tr",
        FluentLoader::new(&*LOCALES).with_default_lang(current_language()),
    );
    tera.render_str(input, &ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(tr!("app-name"), "Delfin");
    }

    #[test]
    fn test_tera_translate() -> Result<(), tera::Error> {
        assert_eq!(tera_tr(r#"{{ tr(key="app-name") }}"#)?, "Delfin");
        Ok(())
    }
}
