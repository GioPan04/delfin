fn main() {
    println!(
        "cargo:rustc-link-search={}",
        std::env::current_dir()
            .unwrap()
            .join("../video_player_mpv/sys/build/")
            .to_str()
            .unwrap()
    );
    println!(
        "cargo:rustc-link-search={}",
        std::env::current_dir()
            .unwrap()
            .join("../build/video_player_mpv/sys/")
            .to_str()
            .unwrap()
    );
}
