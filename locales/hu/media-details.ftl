media-details-refresh-button = Frissítés
media-details-unnamed-episode = Névtelen epizód
media-details-play-button =
    { $resume ->
        [true] Folytatás
       *[false] Lejátszás
    }
    .with-episode-and-season =
        { $resume ->
            [true] Folytatás
           *[false] Lejátszás
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Következő epizód lejátszása
media-details-episode-list-empty = Ehhez az évadhoz nem találhatóak epizódok.
media-details-unnamed-season = Névtelen évad
media-details-watched = Megnézve
media-details-unwatched = Nincs megnézve
media-details-unnamed-item = Névtelen elem
media-details-years = { $startYear } - { $endYear }
    .until-present = { $startYear } - Napjainkig
    .present = Napjainkig
media-details-run-time =
    { $hours ->
        [0] { $minutes } perc
       *[other] { $hours } óra { $minutes } perc
    }
media-details-season-tooltip = Ebben az évadban { $unplayedItemCount } meg nem nézett epizód van
    .unknown-item-count = Ebben az évadban van meg nem nézett epizód
