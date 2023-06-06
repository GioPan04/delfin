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
    position: &gtk::Label,
    duration: &gtk::Label,
) -> (Play, SourceId) {
    let renderer = PlayVideoOverlayVideoRenderer::with_sink(sink);
    let player = Play::new(Some(renderer));

    let scrubber = scrubber.downgrade();
    let position = position.downgrade();
    let duration = duration.downgrade();
    let player_ref = player.downgrade();
    let scrubber_being_moved = Arc::clone(scrubber_being_moved);
    let playback_timeout_id = glib::timeout_add_local(Duration::from_millis(500), {
        move || {
            if let Some(player) = player_ref.upgrade() {
                let (scrubber, position, duration) =
                    match (scrubber.upgrade(), position.upgrade(), duration.upgrade()) {
                        (Some(scrubber), Some(position), Some(duration)) => {
                            (scrubber, position, duration)
                        }
                        _ => return glib::Continue(true),
                    };

                scrubber.block_signal(&scrubber_value_change_handler);

                let position_time = player.position();
                // TODO: some formats don't have duration, maybe we can default
                // to the one that Jellyfin gives us in RunTimeTicks
                let duration_time = player.duration();
                if let (Some(position_time), Some(duration_time)) = (position_time, duration_time) {
                    if !*scrubber_being_moved.read().unwrap() {
                        scrubber.set_range(0.0, duration_time.seconds() as f64);
                        scrubber.set_value(position_time.seconds() as f64);
                        position.set_label(&position_time.to_timestamp());
                        duration.set_label(&duration_time.to_timestamp());
                    } else {
                        let position_time = gst::ClockTime::from_seconds(scrubber.value() as u64);
                        position.set_label(&position_time.to_timestamp());
                        duration.set_label(&duration_time.to_timestamp());
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
