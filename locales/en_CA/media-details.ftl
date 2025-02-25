media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – Present
    .present = Present
media-details-refresh-button = Refresh
media-details-unnamed-episode = Unnamed Episode
media-details-unnamed-item = Unnamed Item
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
media-details-season-tooltip =
    This season has { $unplayedItemCount } unplayed { $unplayedItemCount ->
        [one] episode
       *[other] episodes
    }
    .unknown-item-count = This season has unplayed episodes
media-details-episode-list-empty = No episodes were found for this season.
media-details-unnamed-season = Unnamed Season
