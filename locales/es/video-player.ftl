vp-volume-mute-tooltip =
    { $muted ->
        [true] Dejar de silenciar
       *[false] Silenciar
    }
vp-unnamed-track = Pista sin nombre
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Pistas de audio
       *[false] No hay pistas de subtítulos disponibles
    }
vp-subtitle-track-off = Apagado
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Pistas de audio
vp-no-subtitles-available = No hay subtítulos disponibles.
vp-audio-track-menu = Pista de audio
vp-duration-tooltip =
    .total = Cambiar a tiempo restante
    .remaining = Cambiar a duración total
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Siguiente
       *[previous] Anterior
    } episode
vp-unnamed-item = Elemento sin nombre
vp-play-pause-tooltip =
    { $playing ->
        [true] Pausar
       *[false] Reproducir
    }
vp-fullscreen-tooltip =
    { $enter ->
        [true] Entrar
       *[false] Salir
    } fullscreen
vp-skip-forwards-backwards-tooltip =
    Skip { $direction ->
        [forwards] Hacia adelante
       *[backwards] Hacia atrás
    } { $seconds } { $seconds ->
        [one] segundo
       *[other] segundos
    }
vp-subtitle-track-external = Pista de subtítulos externos
vp-skip-intro =
    .manual = Saltar intro
    .auto = Saltando la intro en { $seconds }…
vp-subtitle-track-menu = Pista de subtítulos
vp-next-up-starting =
    El próximo episodio empieza en { $remaining } { $remaining ->
        [one] segundo
       *[other] segundos
    }…
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Pista { $id } – { $language }
    .id = Pista { $id }
vp-next-up-action =
    .play = Reproducir ahora
    .hide = Ocultar
