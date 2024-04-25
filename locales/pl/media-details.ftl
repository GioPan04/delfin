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
media-details-unwatched = Nieobejrzany
media-details-watched = Obejrzany
media-details-run-time =
    { $hours ->
        [0] { $minutes }min
       *[other] { $hours }godz { $minutes }min
    }
media-details-toggle-watched-error =
    Failed to mark { $type ->
       *[episode] odcinek
        [series] seria
        [movie] film
    } as { $watched ->
        [true] obejrzane
       *[false] nie obejrzane
    }
