# Celine's Winter general actions, kiss and sleep

Five strips add Winter general actions North/South/East, kiss East and sleep
East. Their 26 source frames contain 17 distinct images; native West mirroring
adds 12 review cases. The profile grows from 345 to 350 regions.

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
unchanged from the accepted Winter-special batch. Exact paths are under
`assets/animations/NPCs/Celine/Sprites/Winter/`, named
`spr_npc_celine_winter_{action,kiss,sleep}_{direction}.png`. The complete export
is `extracted/celine-winter-general-study`; raw metadata and frame counts are
in `tmp/celine-winter-general-author-metadata.json`.

Every frame retains 80×80 geometry, Default atlas and Middle/54 origin.
Actions retain `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kissing retains
`[0.15,0.15,0.8,0.15]`; sleeping retains single-frame defaults. Earlier region
objects, seven color groups and source/target mappings remain unchanged.
The existing `FCD9B3`, `F0B988`, `D37A57` and selected `672115` components
retain light/middle/shadow/deep roles. No new shade is needed.

All actual Winter frames were inspected in Vanilla and all four targets.
Accepted Autumn counterparts guided the material review; their seeds were
not copied. Masks include face contours and moving fingers while preserving
hair, coat, scarf, belt, boots, black eye marks and mouth details. For example,
action East frame 0 cheek `[45,37]` and hand `[40,48]` change while hair
`[36,32]` and boot `[38,52]` stay original. North action frame 0 fingertip
`[33,45]` changes beside protected hair `[35,45]`. Kiss frame 2 hair
`[38,40]` is `B65932` in Winter, differing from the Autumn reference, and
remains original. These coordinates are frame-local; frame numbers start at
zero.

The 265 new seeds select 779 pixels per target: 534 core skin and 245 deep
contour pixels. Another 287 matching dark material pixels remain original.
Every frame has 3–46 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Action East | 7 | 265 |
| Action North | 7 | 36 |
| Action South | 7 | 270 |
| Kiss East | 4 | 167 |
| Sleep East | 1 | 41 |

Profile SHA-256:
`cba4f7d54a183c58c548573d57ad8c7e944a3eda03df8e3feefc9a291482efb1`.
The common new-frame selection digest is
`31ffa6ced3816c7b8551c47499c53a706a9cf985b4e58badddb731cb1be4d12a`.

The focused opt-in `tests/celine_winter_general.rs` checks 30 literal
source-art landmarks, all four target ramps, every new pixel, alpha,
metadata, complete core skin coverage and common selection. An omission
mutation removing the action East cheek component failed at `[45,37]`; a
spill mutation selecting its boot failed at `[38,52]`. The final profile
passes. Local negative controls use `FOM_CELINE_WINTER_GENERAL_PROFILE`.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_winter_general -- --ignored'
```

All 1,400 standalone variants validate with strict source hashes and exact
recipes. The final bundle in `generated/celine-winter-general-trial` matches
the inspected candidate PNG and metadata files byte for byte. All 3,450
earlier original/variant PNG and metadata files match
`generated/characters-world-winter-special-trial/characters/celine` exactly;
no previous source hash or mask was refreshed.

- [Five-choice summary](../../generated/celine-winter-general-preview/summary.png):
  general actions, kissing and sleeping in Vanilla, Debug Blue, Hayden, Ryis
  and Seridia.
- [Complete Vanilla/Blue review](../../generated/celine-winter-general-preview/blue-review/index.html):
  all 38 direction/frame cases on six content pages, at most eight per page.

Crop `[28,24,24,32]` contains every visible source pixel and is symmetric for
native West reversal. Exact checks cover 5,836,800 full-review pixels,
307,200 summary pixels, all 25 summary bindings, raw metadata and West
reversal. All 12 West pairs were also visually inspected. Chromium verifies
all seven HTML pages, 39 decoded images, navigation links and absence of
horizontal overflow. Evidence is under `tmp/celine-winter-general-author-`,
including `initial-audit.json`, `green.log`, `omission-red.log`,
`spill-red.log`, `validation.json`, `preview-pixels.json` and
`preview-browser.json`.

See [combined integration](world-winter-general.md) for shared checks, native
metadata and installation. Static images do not establish live animation
timing, natural outfit/state transitions or separately drawn held items.
Source art, generated variants, previews and temporary helpers remain ignored.
This character slice changes no runtime code.
