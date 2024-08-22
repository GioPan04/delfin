media-details-refresh-button = Actualiser
media-details-unnamed-episode = Nom d'épisode inconnu
media-details-unnamed-item = Nom inconnu
media-details-play-button =
    { $resume ->
        [true] Continuer
       *[false] Lecture
    }
    .with-episode-and-season =
        { $resume ->
            [true] Poursuivre
           *[false] Lecture
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Lancer le prochain épisode
media-details-unnamed-season = Nom de saison inconnu
media-details-episode-list-empty = Aucun épisode n'a été trouvé pour cette saison.
media-details-watched = Regardé
media-details-unwatched = Non regardé
media-details-backdrop-error = Échec du chargement de l'arrière-plan
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Présent
    .present = Présent
media-details-season-tooltip =
    Cette saison a { $unplayedItemCount } non regardés { $unplayedItemCount ->
        [one] épisode
       *[other] épisodes
    }
    .unknown-item-count = Cette saison comporte des épisodes non regardés
media-details-run-time =
    { $hours ->
        [0] { $minutes }m
       *[other] { $hours }h { $minutes }m
    }
media-details-toggle-watched-error =
    Échec du marquage des { $type ->
       *[episode] épisodes
        [series] séries
        [movie] films
    } comme { $watched ->
        [true] regardé
       *[false] non regardé
    }
