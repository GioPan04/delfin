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
    ($id:expr, {$($k:expr => $v:expr),* $(,)?}) => {{
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_translate() {
        assert_eq!(tr!("app-name"), "Delfin");
    }
}
