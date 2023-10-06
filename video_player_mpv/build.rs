fn main() {
    println!(
        "cargo:rustc-link-search={}",
        std::env::current_dir()
            .unwrap()
            .join("../video_player_mpv_sys/build/")
            .to_str()
            .unwrap()
    );
}
