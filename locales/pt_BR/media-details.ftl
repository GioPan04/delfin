media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Presente
    .present = Presente
media-details-refresh-button = Atualizar
media-details-unnamed-episode = Episódio Sem Nome
media-details-unnamed-item = Item Sem Nome
media-details-play-button =
    { $resume ->
        [true] Retomar
       *[false] Reproduzir
    }
    .with-episode-and-season =
        { $resume ->
            [true] Continuar
           *[false] Reproduzir
        } T{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Reproduzir próximo episódio
media-details-season-tooltip =
    Esta temporada tem { $unplayedItemCount } { $unplayedItemCount ->
        [one] episódio
       *[other] episódios
    } não assistidos
    .unknown-item-count = Esta temporada tem episódios não assistidos
media-details-episode-list-empty = Nenhum episódio encontrado para esta temporada.
media-details-unnamed-season = Temporada Sem Nome
