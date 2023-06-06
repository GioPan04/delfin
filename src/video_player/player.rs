use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use gst::{
    glib::{self, SignalHandlerId, SourceId},
    Element,
};
use gstplay::{Play, PlayVideoOverlayVideoRenderer};
use gtk::prelude::*;
use relm4::gtk;

pub fn create_player(
    sink: &Element,
    scrubber: &gtk::Scale,
    scrubber_value_change_handler: SignalHandlerId,
    scrubber_being_moved: &Arc<RwLock<bool>>,
    timestamp: &gtk::Label,
) -> (Play, SourceId) {
    let renderer = PlayVideoOverlayVideoRenderer::with_sink(sink);
    let player = Play::new(Some(renderer));

    let scrubber = scrubber.downgrade();
    let timestamp = timestamp.downgrade();
    let player_ref = player.downgrade();
    let scrubber_being_moved = Arc::clone(scrubber_being_moved);
    let playback_timeout_id = glib::timeout_add_local(Duration::from_millis(500), {
        move || {
            if let Some(player) = player_ref.upgrade() {
                let (scrubber, timestamp) = match (scrubber.upgrade(), timestamp.upgrade()) {
                    (Some(scrubber), Some(timestamp)) => (scrubber, timestamp),
                    _ => return glib::Continue(true),
                };

                scrubber.block_signal(&scrubber_value_change_handler);

                let position = player.position();
                // TODO: some formats don't have duration, maybe we can default
                // to the one that Jellyfin gives us in RunTimeTicks
                let duration = player.duration();
                if let (Some(position), Some(duration)) = (position, duration) {
                    if !*scrubber_being_moved.read().unwrap() {
                        scrubber.set_range(0.0, duration.seconds() as f64);
                        scrubber.set_value(position.seconds() as f64);
                        timestamp.set_label(&format!(
                            "{} / {}",
                            position.to_timestamp(),
                            duration.to_timestamp()
                        ));
                    } else {
                        let position = gst::ClockTime::from_seconds(scrubber.value() as u64);
                        timestamp.set_label(&format!(
                            "{} / {}",
                            position.to_timestamp(),
                            duration.to_timestamp()
                        ));
                    }
                }

                scrubber.unblock_signal(&scrubber_value_change_handler);
            }
            glib::Continue(true)
        }
    });

    (player, playback_timeout_id)
}

trait ToTimestamp {
    fn to_timestamp(self) -> String;
}

impl ToTimestamp for gst::ClockTime {
    fn to_timestamp(self) -> String {
        let minutes = self.seconds() / 60;
        let seconds = self.seconds() % 60;
        format!("{:0>2}:{:0>2}", minutes, seconds)
    }
}
