fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        std::env::current_dir()
            .unwrap()
            .join("../video_player_mpv_sys/build/libvideo-player-mpv.a")
            .to_str()
            .unwrap()
    );
}
