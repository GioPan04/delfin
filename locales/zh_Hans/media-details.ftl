media-details-unnamed-episode = 未命名剧集
media-details-unnamed-item = 未命名项目
media-details-refresh-button = 刷新
media-details-play-button =
    { $resume ->
        [true] 继续播放
       *[false] 播放
    }
    .with-episode-and-season =
        { $resume ->
            [true] 继续播放
           *[false] 播放
        } S{ $seasonNumber }:E{ $episodeNumber }
    .next-episode = 播放下一集
media-details-season-tooltip =
    本季有 { $unplayedItemCount } 未播放 { $unplayedItemCount ->
        [one] 集
       *[other] 集
    }
    .unknown-item-count = 本季有未播放的剧集
media-details-unnamed-season = 未命名季
media-details-episode-list-empty = 本季中未找到任何剧集。
media-details-years = { $startYear } – { $endYear }
    .until-present = { $startYear } – 当前
    .present = 当前
media-details-run-time =
    { $hours ->
        [0] { $minutes }分钟
       *[other] { $hours }小时 { $minutes }分钟
    }
media-details-watched = 已观看
media-details-unwatched = 未观看
media-details-toggle-watched-error =
    无法标记 { $type ->
       *[episode] 剧集
        [series] 系列
        [movie] 电影
    } 为 { $watched ->
        [true] 已观看
       *[false] 未观看
    }
media-details-backdrop-error = 无法加载媒体背景
