media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – heute
    .present = heute
media-details-refresh-button = Aktualisieren
media-details-unnamed-episode = Unbenannte Folge
media-details-unnamed-item = Unbenanntes Element
media-details-play-button =
    { $resume ->
        [true] Fortsetzen
       *[false] Abspielen
    }
    .with-episode-and-season =
        S{ $seasonNumber }:E{ $episodeNumber } { $resume ->
            [true] fortsetzen
           *[false] abspielen
        }
    .next-episode = Nächste Folge abspielen
media-details-season-tooltip =
    Du hast { $unplayedItemCount } { $unplayedItemCount ->
        [one] Folge
       *[other] Folgen
    } dieser Staffel noch nicht gesehen
    .unknown-item-count = Du hast einige Folgen dieser Staffel noch nicht gesehen
media-details-episode-list-empty = Diese Staffel enthält keine Folgen.
media-details-unnamed-season = Unbenannte Staffel
