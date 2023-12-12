vp-volume-mute-tooltip =
    { $muted ->
        [true] Појачај
       *[false] Утишај
    }
vp-unnamed-track = Неименовани запис
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Титлови
       *[false] Нема доступних титлова
    }
vp-subtitle-track-off = Искључено
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Звучни записи
vp-no-subtitles-available = Нема доступних титлова.
vp-audio-track-menu = Звучни запис
vp-duration-tooltip =
    .total = Пребаци на преостало време
    .remaining = Пребаци на укупно трајање
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Следећа
       *[previous] Претходна
    } епизода
vp-unnamed-item = Ставка без имена
vp-play-pause-tooltip =
    { $playing ->
        [true] Пауза
       *[false] Пусти
    }
vp-fullscreen-tooltip =
    { $enter ->
        [true] Повећај преко целог екрана
       *[false] Смањи
    }
vp-skip-forwards-backwards-tooltip =
    Премотај  { $seconds } { $seconds ->
        [one] секунд
       *[other] секунди
    } { $direction ->
        [forwards] унапред
       *[backwards] уназад
    }
vp-subtitle-track-external = Спољашњи титл
vp-skip-intro =
    .manual = Прескочи шпицу
    .auto = Прескачем шпицу за { $seconds }…
vp-subtitle-track-menu = Титл
vp-next-up-starting =
    Следећа епизода почиње за { $remaining } { $remaining ->
        [one] секунду
       *[other] секунди
    }…
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Запис { $id } – { $language }
    .id = Запис { $id }
vp-next-up-action =
    .play = Пусти сада
    .hide = Сакриј
