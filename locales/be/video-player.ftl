vp-unnamed-item = Элемент без назвы
vp-fullscreen-tooltip =
    { $enter ->
        [true] Увод
       *[false] Выйсці
    } fullscreen
vp-next-up-starting =
    Наступны эпізод пачнецца праз { $remaining } { $remaining ->
        [one] секунд
       *[other] секунд
    }…
vp-next-up-action =
    .play = Прайграць зараз
    .hide = Схаваць
vp-volume-mute-tooltip =
    { $muted ->
        [true] Уключыць гук
       *[false] Адключыць гук
    }
vp-unnamed-track = Дарожка без назвы
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Субцітры
       *[false] Няма субцітраў
    }
vp-subtitle-track-off = Адключана
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Гукавыя дарожкі
vp-no-subtitles-available = Няма даступных субцітраў.
vp-audio-track-menu = Гукавая дарожка
vp-duration-tooltip =
    .total = Пераключыць на час, які застаўся
    .remaining = Пераключыць на агульную працягласць
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Наступны
       *[previous] Папярэдні
    } эпізод
vp-play-pause-tooltip =
    { $playing ->
        [true] Паўза
       *[false] Прайграць
    }
vp-skip-forwards-backwards-tooltip =
    Прапусціць { $direction ->
        [forwards] Наперад
       *[backwards] Назад
    } { $seconds } { $seconds ->
        [one] секунду
       *[other] секунд
    }
vp-subtitle-track-external = Знешнія субцітры
vp-skip-intro =
    .manual = Прапусціць застаўку
    .auto = Пропуск застаўкі праз { $seconds }…
vp-subtitle-track-menu = Субцітры
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Дарожка { $id } – { $language }
    .id = Дарожка { $id }
