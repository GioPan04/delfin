preferences-window-title = Preferences

preferences-page-general = General

preferences-group-video-player = Video Player

preferences-skip-amount = {$seconds} {
        $seconds ->
            [one] second
            *[other] seconds
    }

preferences-video-player-skip-backwards =
    .title = Skip backwards amount
    .subtitle = How many seconds to skip backwards at a time

preferences-video-player-skip-forwards =
    .title = Skip forwards amount
    .subtitle = How many seconds to skip forwards at a time

preferences-video-player-intro-skipper =
    .title = Intro Skipper plugin
    .subtitle =
        Shows a Skip Intro button during episode intros.
        This requires the <a href="{$introSkipperUrl}">Intro Skipper</a> plugin to be installed on your server.

preferences-video-player-jellyscrub =
    .title = Jellyscrub plugin
    .subtitle =
        Show thumbnails while hovering over the video progress bar.
        This requires the <a href="{$jellyscrubUrl}">Jellyscrub</a> plugin to be installed on your server.

preferences-video-player-backend =
    .title = Video player backend
    .subtitle = Requires restarting {app-name}.
    .value-mpv = MPV
    .value-gstreamer = GStreamer

preferences-video-player-hls-playback = HLS playback
