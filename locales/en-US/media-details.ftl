media-details-refresh-button = Refresh
media-details-unnamed-item = Unnamed Item
media-details-unnamed-episode = Unnamed Episode
media-details-unnamed-season = Unnamed Season
media-details-play-button =
    { $resume ->
        [true] Resume
       *[false] Play
    }
    .with-episode-and-season =
        { $resume ->
            [true] Resume
           *[false] Play
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = Play next episode
media-details-episode-list-empty = No episodes were found for this season.
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Present
    .present = Present
media-details-season-tooltip =
    This season has { $unplayedItemCount } unplayed { $unplayedItemCount ->
        [one] episode
       *[other] episodes
    }
    .unknown-item-count = This season has unplayed episodes
media-details-watched = Watched
media-details-unwatched = Not watched
media-details-toggle-watched-error = Failed to mark { $type ->
        *[episode] episode
        [series] series
        [movie] movie
    } as { $watched ->
        [true] watched
        *[false] not watched
    }
