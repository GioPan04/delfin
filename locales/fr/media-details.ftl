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
