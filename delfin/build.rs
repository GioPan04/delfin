use std::{env, fs, path::PathBuf};

use regex::Regex;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!(
        "cargo:rerun-if-changed={}",
        std::env::current_dir()
            .unwrap()
            .join("../video_player_mpv/sys/build/libvideo-player-mpv.a")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-search=/app/lib");
    println!("cargo:rustc-link-search=video_player_mpv/sys");

    build_css(out_dir);
}

fn build_css(out_dir: PathBuf) {
    let css = grass::from_string(include_str!("src/app.scss"), &grass::Options::default()).unwrap();

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
