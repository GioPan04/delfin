vp-volume-mute-tooltip =
    { $muted ->
        [true] Unmute
       *[false] Mute
    }
vp-unnamed-track = Unnamed Track
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Subtitle Tracks
       *[false] No Subtitle Tracks Available
    }
vp-subtitle-track-off = Off
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Audio Tracks
vp-no-subtitles-available = No subtitles available.
vp-audio-track-menu = Audio Track
vp-duration-tooltip =
    .total = Swap to remaining time
    .remaining = Swap to total duration
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Next
       *[previous] Previous
    } episode
vp-unnamed-item = Unnamed Item
vp-play-pause-tooltip =
    { $playing ->
        [true] Pause
       *[false] Play
    }
vp-fullscreen-tooltip =
    { $enter ->
        [true] Enter
       *[false] Exit
    } fullscreen
vp-skip-forwards-backwards-tooltip =
    Skip { $direction ->
        [forwards] forwards
       *[backwards] backwards
    } { $seconds } { $seconds ->
        [one] second
       *[other] seconds
    }
vp-skip-intro =
    .manual = Skip Intro
    .auto = Skipping intro in { $seconds }…
vp-subtitle-track-menu = Subtitle Track
vp-next-up-starting =
    Next episode starting in { $remaining } { $remaining ->
        [one] second
       *[other] seconds
    }…
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Track { $id } – { $language }
    .id = Track { $id }
vp-next-up-action =
    .play = Play now
    .hide = Hide
vp-subtitle-track-external = External Subtitle Track
