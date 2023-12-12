media-details-refresh-button = Osveži
media-details-unnamed-episode = Epizoda bez imena
media-details-unnamed-item = Stavka bez imena
media-details-play-button =
    { $resume ->
        [true] Nastavi
       *[false] Pusti
    }
    .with-episode-and-season =
        { $resume ->
            [true] Nastavi
           *[false] Pusti
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Pusti sledeću epizodu
media-details-unnamed-season = Sezona bez imena
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Sada
    .present = Sada
media-details-season-tooltip =
    Niste odgledali { $unplayedItemCount } ove sezone { $unplayedItemCount ->
        [jednu] epizodu
       *[other] epizoda
    }
    .unknown-item-count = Niste odgledali sve epizode ove sezone
media-details-episode-list-empty = Nije pronađena ni jedna epizoda ove sezone.
