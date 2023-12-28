vp-volume-mute-tooltip =
    { $muted ->
        [true] Ativar
       *[false] Desativar
    } Som
vp-unnamed-track = Faixa Sem Nome
vp-subtitle-track-tooltip =
    { $subtitlesAvailable ->
        [true] Legendas
       *[false] Nenhuma legenda disponível
    }
vp-subtitle-track-off = Desligado
vp-backend-gst-track-name = { $displayName } – { $title }
vp-audio-track-tooltip = Faixas de Áudio
vp-no-subtitles-available = Nenhuma legenda disponível.
vp-audio-track-menu = Faixa de Áudio
vp-duration-tooltip =
    .total = Mudar para tempo restante
    .remaining = Mudar para duração total
vp-next-prev-episode-tooltip =
    { $direction ->
        [next] Próximo episódio
       *[previous] Episódio anterior
    }
vp-unnamed-item = Item Sem Nome
vp-play-pause-tooltip =
    { $playing ->
        [true] Pausar
       *[false] Reproduzir
    }
vp-fullscreen-tooltip =
    { $enter ->
        [true] Entrar
       *[false] Sair
    } da tela cheia
vp-skip-forwards-backwards-tooltip =
    { $direction ->
        [forwards] Avançar
       *[backwards] Retroceder
    } { $seconds } { $seconds ->
        [one] segundo
       *[other] segundos
    }
vp-subtitle-track-external = Legenda Externa
vp-skip-intro =
    .manual = Pular Abertura
    .auto = Pulando abertura em { $seconds }…
vp-subtitle-track-menu = Legenda
vp-next-up-starting =
    Próximo episódio em { $remaining } { $remaining ->
        [one] segundo
       *[other] segundos
    }…
vp-backend-mpv-track-name =
    .title-and-language = { $title } – { $language }
    .id-and-language = Faixa { $id } – { $language }
    .id = Faixa { $id }
vp-next-up-action =
    .play = Reproduzir agora
    .hide = Ocultar
