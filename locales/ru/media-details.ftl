media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Настоящее время
    .present = Настоящее время
media-details-refresh-button = Обновить
media-details-unnamed-episode = Серия без названия
media-details-unnamed-item = Элемент без названия
media-details-play-button =
    { $resume ->
        [true] Смотреть дальше
       *[false] Смотреть
    }
    .with-episode-and-season =
        { $resume ->
            [true] Смотреть дальше
           *[false] Смотреть
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Смотреть следующую серию
media-details-season-tooltip =
    В этом сезоне вы не смотрели { $unplayedItemCount } { $unplayedItemCount ->
        [one] серию
       *[other] сер.
    }
    .unknown-item-count = В этом сезоне есть непросмотренные серии
media-details-episode-list-empty = Не найдены серии для этого сезона.
media-details-unnamed-season = Сезон без названия
