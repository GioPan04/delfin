vp-volume-mute-tooltip =
    { $muted ->
        [true] Stummschaltung aufheben
       *[false] Stummschalten
    }
vp-unnamed-track = Unbenannte Spur
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Untertitel
       *[false] Keine Untertitel verfügbar
    }
vp-subtitle-track-off = Aus
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Tonspuren
vp-no-subtitles-available = Keine Untertitel verfügbar.
vp-audio-track-menu = Tonspur
vp-duration-tooltip =
    .total = Verbleibende Dauer anzeigen
    .remaining = Gesamtdauer anzeigen
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Nächste
       *[previous] Vorherige
    } Folge
vp-unnamed-item = Unbenanntes Element
vp-play-pause-tooltip =
    { $playing ->
        [true] Pause
       *[false] Abspielen
    }
vp-fullscreen-tooltip =
    { $enter ->
        [true] Vollbild
       *[false] Vollbildmodus beenden
    }
vp-skip-forwards-backwards-tooltip =
    { $seconds } { $seconds ->
        [one] Sekunde
       *[other] Sekunden
    } { $direction ->
        [forwards] vorspulen
       *[backwards] zurückspulen
    }
vp-subtitle-track-external = Externer Untertitel
vp-skip-intro =
    .manual = Vorspann überspringen
    .auto = Überspringe Vorspann in { $seconds }…
vp-subtitle-track-menu = Untertitel
vp-next-up-starting =
    Nächste Folge beginnt in { $remaining } { $remaining ->
        [one] Sekunde
       *[other] Sekunden
    }…
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Track { $id } – { $language }
    .id = Track { $id }
vp-next-up-action =
    .play = Jetzt abspielen
    .hide = Ausblenden
