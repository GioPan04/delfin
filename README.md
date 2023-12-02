# Delfin [![status-badge](https://ci.codeberg.org/api/badges/12819/status.svg)](https://ci.codeberg.org/repos/12819)

Stream movies and TV shows from Jellyfin.

Downloads are available at [delfin.avery.cafe](https://delfin.avery.cafe/).

## Features/Roadmap

The following features are currently implemented or planned:

- Direct playback
- Movie collections
- TV show collections
- [Jellyscrub](https://github.com/nicknsy/jellyscrub/) (scrubber thumbnails) plugin support
- [Intro Skipper](https://github.com/ConfusedPolarBear/intro-skipper/) plugin support
- *Music collections (planned)*
- *Live TV (planned)*
- *Library search (planned)*
- *Syncplay (planned)*
- *Transcoded playback (planned)*

These features aren't planned at the moment (but maybe in the future):

- Managing your collections
- Server administration
- Book collections

## Development Setup

- Install pre-commit hooks with `pre-commit install`
- `meson setup build && cd build`
- `meson compile`
- `./delfin/delfin`

## Translations

Contributions to translations are welcome on [Weblate](https://translate.codeberg.org/projects/delfin/).

<a href="https://translate.codeberg.org/engage/delfin/">
    <img src="https://translate.codeberg.org/widget/delfin/open-graph.png" alt="Translation status" width="400px" />
</a>

<br />
<a href="https://translate.codeberg.org/engage/delfin/">
    <img src="https://translate.codeberg.org/widget/delfin/multi-auto.svg" alt="Translation status" />
</a>
