media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Obecnie
    .present = Obecnie
media-details-refresh-button = Odśwież
media-details-unnamed-episode = Nienazwany odcinek
media-details-play-button =
    { $resume ->
        [true] Wznów
       *[false] Odtwórz
    }
    .with-episode-and-season =
        { $resume ->
            [true] Wznów
           *[false] Odtwórz
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Odtwórz następny odcinek
media-details-season-tooltip =
    Ten sezon ma { $unplayedItemCount } nieobejrzany/-e { $unplayedItemCount ->
        [one] odcinek
       *[other] odcinki/odcinków
    }
    .unknown-item-count = Ten sezon ma nieobejrzane odcinki
media-details-episode-list-empty = Nie znaleziono odcinków dla tego sezonu.
media-details-unnamed-season = Nienazwany sezon
media-details-unnamed-item = Nienazwany element
