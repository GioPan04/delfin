media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Presente
    .present = Presente
media-details-refresh-button = Actualizar
media-details-unnamed-episode = Episodio sin nombre
media-details-unnamed-item = Elemento sin nombre
media-details-play-button =
    { $resume ->
        [true] Reanudar
       *[false] Reproducir
    }
    .with-episode-and-season =
        { $resume ->
            [true] Reanudar
           *[false] Reproducir
        } T{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Reproducir siguiente episodio
media-details-season-tooltip =
    Esta temporada tiene { $unplayedItemCount } sin reproducir { $unplayedItemCount ->
        [one] episodio
       *[other] episodios
    }
    .unknown-item-count = Esta temporada tiene episodios sin reproducir
media-details-episode-list-empty = No se han encontrado episodios de esta temporada.
media-details-unnamed-season = Temporada sin nombre
