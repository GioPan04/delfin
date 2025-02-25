prefs-vp-experimental =
    .title = Experimental preferences
    .subtitle =
        These preferences may be broken or incomplete.
        Modifying them is not recommended.
prefs-vp-interface = Interface
prefs-general-page = General
prefs-skip-amount =
    { $seconds } { $seconds ->
        [one] second
       *[other] seconds
    }
prefs-window-title = Preferences
prefs-vp-intro-skipper-auto-skip =
    .title = Automatically skip intros
    .subtitle = Skip intros without having to press the Skip Intro button
prefs-vp-skip-forwards =
    .title = Skip forwards amount
    .subtitle = How many seconds to skip forwards at a time
prefs-vp-jellyscrub =
    .title = Jellyscrub plugin
    .subtitle =
        Show thumbnails while hovering over the video progress bar.
        This requires the <a href="{ $jellyscrubUrl }">Jellyscrub</a> plugin to be installed on your server.
prefs-vp-other = Other
prefs-vp-skip-backwards =
    .title = Skip backwards amount
    .subtitle = How many seconds to skip backwards at a time
prefs-vp-hls-playback = HLS playback
prefs-vp-page = Video Player
prefs-vp-backend =
    .title = Video player backend
    .subtitle = Requires restarting { app-name }.
    .value-mpv = MPV
    .value-gstreamer = GStreamer
prefs-vp-plugins = Plugins
prefs-general-theme =
    .title = Theme
    .option-default = System Default
    .option-light = Light
    .option-dark = Dark
prefs-vp-intro-skipper =
    .title = Intro Skipper plugin
    .subtitle =
        Shows a Skip Intro button during episode intros.
        This requires the <a href="{ $introSkipperUrl }">Intro Skipper</a> plugin to be installed on your server.
prefs-vp-on-left-click-options =
    .play-pause = Play/pause video
    .toggle-controls = Show/hide controls
prefs-vp-on-left-click =
    .title = When the mouse is clicked
    .subtitle = What happens when you left click inside the video player
prefs-vp-subs = Subtitles
prefs-vp-subs-position =
    .title = Subtitle position
    .subtitle = Where 0 is the top of the screen, and 100 is the bottom
prefs-vp-subs-scale =
    .title = Subtitle scale
    .subtitle = Scaling factor for subtitle text
prefs-vp-subs-colour =
    .title = Subtitle text colour
prefs-vp-subs-background-colour =
    .title = Subtitle background colour
prefs-vp-subs-reset =
    .label = Reset
    .tooltip = Reset subtitle settings
prefs-general-language =
    .title = Language
    .subtitle = Contribute to translations on <a href="{ $weblateUrl }">Weblate</a>
    .option-default = System Default ({ $languageId })
prefs-vp-subs-style-ass-warning =
    These settings don't apply to ASS/SSA subtitles.
    For ASS/SSA subtitles, see "{ prefs-vp-subs-more }".
prefs-vp-subs-font =
    .title = Subtitle font
    .subtitle = Only supported fonts are listed
prefs-vp-subs-style = Subtitle style
prefs-vp-subs-more-ass-warning =
    .title = These preferences will affect ASS/SSA subtitles!
    .subtitle =
        ASS/SSA subtitles often include special styling and positioning. Changing these preferences may cause ASS/SSA subtitles to render incorrectly.

        If you only want to change the style for other subtitle formats, it's recommended that you change the preferences under "{ prefs-vp-subs-style }" instead.
prefs-vp-subs-more = More subtitle preferences
