use fluent_templates::FluentLoader;
use tera::Tera;

use crate::globals::CONFIG;

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "../locales/",
        fallback_language: "en-US",
    };
}

#[macro_export]
macro_rules! tr {
    ($id:expr) => {{
        use fluent_templates::Loader;
        use $crate::{globals::CONFIG, locales::LOCALES};

        &LOCALES
            .lookup(&CONFIG.read().language, $id)
            .expect(&format!("Error looking up message for identifier: {}", $id))
    }};

    // Lookup message with arguments
    ($id:expr, {$($k:expr => $v:expr),* $(,)?}$(,)?) => {{
        use fluent_templates::Loader;
        use std::collections::HashMap;
        use $crate::{globals::CONFIG, locales::LOCALES};

        &LOCALES.lookup_with_args(
            &CONFIG.read().language,
            $id,
            &HashMap::from([$(($k, $v.into()),)*]),
        )
        .expect(&format!("Error looking up message for identifier: {}", $id))
    }};
}

pub fn tera_tr(input: &str) -> Result<String, tera::Error> {
    let mut tera = Tera::default();
    let ctx = tera::Context::default();
    tera.register_function(
        "tr",
        FluentLoader::new(&*LOCALES).with_default_lang(CONFIG.read().language.clone()),
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
