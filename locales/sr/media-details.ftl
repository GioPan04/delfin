media-details-refresh-button = Освежи
media-details-unnamed-episode = Епизода без имена
media-details-unnamed-item = Ставка без имена
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
media-details-unnamed-season = Сезона без имена
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Sada
    .present = Sada
media-details-season-tooltip =
    Нисте одгледали { $unplayedItemCount }  { $unplayedItemCount ->
        [one] епизоду
       *[other] епизода
    }
    .unknown-item-count = Нисте одгледали све епизоде ове сезоне
media-details-episode-list-empty = Nije pronađena ni jedna epizoda ove sezone.
