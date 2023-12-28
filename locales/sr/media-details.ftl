media-details-refresh-button = Освежи
media-details-unnamed-episode = Епизода без имена
media-details-unnamed-item = Ставка без имена
media-details-play-button =
    { $resume ->
        [true] Настави
       *[false] Пусти
    }
    .with-episode-and-season =
        { $resume ->
            [true] Настави
           *[false] Пусти
        } С{ $seasonNumber }:Е{ $episodeNumber }
    .next-episode = Пусти следећу епизоду
media-details-unnamed-season = Сезона без имена
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Сада
    .present = Сада
media-details-season-tooltip =
    Нисте одгледали { $unplayedItemCount }  { $unplayedItemCount ->
        [one] епизоду
       *[other] епизода
    }
    .unknown-item-count = Нисте одгледали све епизоде ове сезоне
media-details-episode-list-empty = Није пронађена ни једна епизода ове сезоне.
