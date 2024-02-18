media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Huidig
    .present = Huidig
media-details-refresh-button = Herladen
media-details-unnamed-episode = Naamloze aflevering
media-details-unnamed-item = Naamloos item
media-details-play-button =
    { $resume ->
        [true] Hervatten
       *[false] Afspelen
    }
    .with-episode-and-season =
        { $resume ->
            [true] Hervatten
           *[false] Afspelen
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Volgende aflevering afspelen
media-details-season-tooltip =
    Dit seizoen bevat { $unplayedItemCount } niet-bekeken { $unplayedItemCount ->
        [one] aflevering
       *[other] afleveringen
    }
    .unknown-item-count = Dit seizoen bevat niet-bekeken afleveringen
media-details-episode-list-empty = Dit seizoen bevat geen afleveringen.
media-details-unnamed-season = Naamloos seizoen
media-details-backdrop-error = De media-achtergrond kan niet worden geladen
media-details-watched = Bekeken
media-details-unwatched = Niet bekeken
media-details-run-time =
    { $hours ->
        [0] { $minutes }m
       *[other] { $hours }h { $minutes }m
    }
media-details-toggle-watched-error =
    Het is niet mogelijk om de { $type ->
       *[episode] aflevering
        [series] serie
        [movie] film
    } te markeren als { $watched ->
        [true] bekeken
       *[false] niet-bekeken
    }
