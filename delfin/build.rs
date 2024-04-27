use std::{
    env, fs,
    path::{Path, PathBuf},
};

use regex::Regex;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if let Ok(build_root) = std::env::var("MESON_BUILD_ROOT") {
        let build_root = PathBuf::from(build_root);
        link_libvideo_player_mpv(&build_root);
    } else {
        println!("cargo:warning=MESON_BUILD_ROOT not set");
    }

    build_css(&out_dir);
}

fn link_libvideo_player_mpv(build_root: &Path) {
    let vpm_build_dir = build_root.join("video_player_mpv/sys");
    println!("cargo:rustc-link-search={}", vpm_build_dir.display());

    let flatpak = std::env::var("FLATPAK")
        .map(|flatpak| flatpak == "true")
        .unwrap_or(false);
    if flatpak {
        // Link mpv when building Flatpak
        println!("cargo:rustc-link-search=/app/lib");
    }
}

fn build_css(out_dir: &Path) {
    let styles_path = PathBuf::from("src/styles");

    println!("cargo:rerun-if-changed={styles_path:#?}");

    let css = grass::from_path(styles_path.join("app.scss"), &grass::Options::default()).unwrap();

    // Output GTK's at-rules. Sass doesn't support custom at-rules, so we wrap
    // them in a string and unwrap them after Sass has done it's thing.

    // GTK colour expressions - these are similar to Sass's colour functions,
    // but they can operate on GTK colour variables.
    let re_gtk_expressions = Regex::new(r#""--gtk-(.+)""#).unwrap();
    // GTK colour variables
    let re_gtk_colours = Regex::new(r#""(@.+)""#).unwrap();

    let css = re_gtk_expressions.replace_all(&css, "$1");
    let css = re_gtk_colours.replace_all(&css, "$1");

    fs::write(out_dir.join("app.css"), &*css).unwrap();
}
