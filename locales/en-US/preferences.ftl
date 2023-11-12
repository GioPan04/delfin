prefs-window-title = Preferences

prefs-general-page = General
prefs-vp-page = Video Player

# General
# =======

prefs-general-theme =
    .title = Theme
    .option-default = System Default
    .option-light = Light
    .option-dark = Dark

# Interface
# =========

prefs-vp-interface = Interface

prefs-skip-amount = {$seconds} {
        $seconds ->
            [one] second
            *[other] seconds
    }
prefs-vp-skip-backwards =
    .title = Skip backwards amount
    .subtitle = How many seconds to skip backwards at a time
prefs-vp-skip-forwards =
    .title = Skip forwards amount
    .subtitle = How many seconds to skip forwards at a time

# Plugins
# =======

prefs-vp-plugins = Plugins

prefs-vp-intro-skipper =
    .title = Intro Skipper plugin
    .subtitle =
        Shows a Skip Intro button during episode intros.
        This requires the <a href="{$introSkipperUrl}">Intro Skipper</a> plugin to be installed on your server.
prefs-vp-intro-skipper-auto-skip =
    .title = Automatically skip intros
    .subtitle = Skip intros without having to press the Skip Intro button

prefs-vp-jellyscrub =
    .title = Jellyscrub plugin
    .subtitle =
        Show thumbnails while hovering over the video progress bar.
        This requires the <a href="{$jellyscrubUrl}">Jellyscrub</a> plugin to be installed on your server.

# Other
# =====

prefs-vp-other = Other

prefs-vp-experimental =
    .title = Experimental Preferences
    .subtitle =
        These preferences may be broken or incomplete.
        Modifying them is not recommended.

prefs-vp-backend =
    .title = Video player backend
    .subtitle = Requires restarting {app-name}.
    .value-mpv = MPV
    .value-gstreamer = GStreamer

prefs-vp-hls-playback = HLS playback
