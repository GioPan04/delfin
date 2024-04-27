use anyhow::Result;
use fluent_templates::lazy_static::lazy_static;
use jellyfin_api::types::{
    DeviceProfile, DirectPlayProfile, DlnaProfileType, SubtitleDeliveryMethod, SubtitleProfile,
};

lazy_static! {
    pub static ref DEVICE_PROFILE_DIRECT_PLAY: DeviceProfile =
        device_profile_direct_play().expect("Error generating device profile");
}

const SUB_FORMATS: &[&str] = &["srt", "ass", "sub", "vtt"];

/// Generates a device profile for direct play, similar to the one that Jellyfin Media Player uses.
fn device_profile_direct_play() -> Result<DeviceProfile> {
    let profile = DeviceProfile::builder()
        .name("Delfin Direct Play Profile".to_string())
        .max_streaming_bitrate(140000000)
        .direct_play_profiles(vec![
            DirectPlayProfile::builder()
                .type_(DlnaProfileType::Video)
                .try_into()?,
            DirectPlayProfile::builder()
                .type_(DlnaProfileType::Audio)
                .try_into()?,
        ]);

    let mut subtitle_profiles: Vec<SubtitleProfile> = vec![];
    for sub_format in SUB_FORMATS {
        subtitle_profiles.push(
            SubtitleProfile::builder()
                .format(Some((*sub_format).to_string()))
                .method(Some(SubtitleDeliveryMethod::External))
                .try_into()?,
        );
        subtitle_profiles.push(
            SubtitleProfile::builder()
                .format(Some((*sub_format).to_string()))
                .method(Some(SubtitleDeliveryMethod::Embed))
                .try_into()?,
        );
    }

    Ok(profile.subtitle_profiles(subtitle_profiles).try_into()?)
}
