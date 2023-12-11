media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Současnost
    .present = Současnost
media-details-refresh-button = Obnovit
media-details-unnamed-episode = Nepojmenovaná epizoda
media-details-unnamed-item = Nepojmenovaná položka
media-details-play-button =
    { $resume ->
        [true] Pokračovat
       *[false] Přehrát
    }
    .with-episode-and-season =
        { $resume ->
            [true] Pokračovat
           *[false] Přehrát
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Přehrát další epizodu
media-details-season-tooltip =
    Tato série má { $unplayedItemCount } { $unplayedItemCount ->
        [one] nepřehranou epizodu
        [few] nepřehrané epizody
       *[other] nepřehraných epizod
    }
    .unknown-item-count = Tato série má nepřehrané epizody
media-details-episode-list-empty = U této série nebyly nalezeny žádné epizody.
media-details-unnamed-season = Nepojmenovaná série
