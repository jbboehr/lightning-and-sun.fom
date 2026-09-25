# Celine's Autumn action, sleep and kiss sprites

Five regular Autumn strips add general action North/South/East, sleep East and
kiss East. The actions have seven frames each, kiss has four, and sleep has one:
26 source frames containing 17 distinct images. Native West mirroring adds 12
review cases. Garden assets remain outside this slice.

The source is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Every earlier original PNG and metadata sidecar matches the retained combined
Autumn-standard baseline; no old hash or mask was refreshed for the new archive.
The full local corpus is `extracted/celine-autumn-special-study`.

All new frames are 80×80, Default atlas, Middle/54 origin. Actions retain
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; kiss retains `[0.15,0.15,0.8,0.15]`.
Sleep retains the single-frame defaults. Raw sidecars and frame counts are in
`tmp/celine-autumn-special-metadata.json`.

The profile grows from 299 to 304 regions. All previous region objects, seven
color groups and source/target mappings remain unchanged. The existing world
skin shades `FCD9B3`, `F0B988`, `D37A57` and selected `672115` components retain
light/middle/shadow/deep roles; no additional colors were needed.

Masks include face and lip contours, tiny North-facing fingertips, and moving
hands. Hair, covered arms, cuffs, scarf, belt, leggings, boots and black eye
marks stay original. East action frame one's face `[45,37]` maps while hair
`[36,32]` stays. Frame two's fingertip `[48,45]` maps beside protected sleeve
and belt details `[38,43]` and `[40,45]`. North action's tiny hand `[35,46]`
maps in frame two while adjacent hair `[35,44]` stays. Sleep's raised hand and
face map while the dark hair below `[36,43]` remains. Coordinates are
frame-local; frame numbers in this paragraph start at one.

The 260 new seeds select 771 pixels per target: 533 core skin and 238 outline
pixels. Another 323 matching dark material pixels stay original. Every frame
has 3–49 selected pixels.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Action East | 7 | 259 |
| Action North | 7 | 36 |
| Action South | 7 | 266 |
| Kiss East | 4 | 171 |
| Sleep East | 1 | 39 |

Profile SHA-256:
`7eddf77e5ee0e18b77a56cf2de82eebebb696a1187ef1d06d208d62c2960a67a`.

All new source frames and four actual target versions were inspected. The
focused opt-in `tests/celine_autumn_special.rs` passes 30 literal source-art
boundaries, four target ramps, every new pixel, alpha, exact metadata, complete
core skin coverage and common selection. A temporary omission profile failed
on the face `[45,37]`, and a spill profile failed on hair `[36,32]`, before the
final profile passed. `FOM_CELINE_AUTUMN_SPECIAL_PROFILE` selects those local
negative controls.

```sh
nix-shell --pure --run 'cargo test --locked --test celine_autumn_special -- --ignored'
```

All 1,216 standalone variants validate. The final bundle in
`generated/celine-autumn-special-trial` matches the inspected candidate outputs.
All 2,990 previous original/variant PNG and metadata files remain byte-identical
to `generated/characters-world-autumn-standard-trial/characters/celine`.
The common new-frame selection digest is
`1c442158051dd5a5f73a4e89d722acb2e2c60b4b4b7975bbbd3bc988b8ed1860`.

- [Five-choice summary](../../generated/celine-autumn-special-preview/summary.png):
  action North/South/East, kiss and sleep samples.
- [Complete Vanilla/Blue review](../../generated/celine-autumn-special-preview/blue-review/index.html):
  all 38 direction/frame cases on six content pages, at most eight per page.

Fresh crop `[28,24,24,32]` includes every visible source pixel and is centered
for native West reversal. Exact checks cover 5,836,800 full-review pixels,
307,200 summary pixels, all 25 summary bindings, metadata and West reversal.
All 12 West pairs were visually inspected. Chromium verifies all seven HTML
pages, 39 decoded images, navigation links and absence of horizontal overflow.
Evidence is under `tmp/celine-autumn-special-`, including `final-audit.json`,
`test.log`, `preview-check.json` and `preview-browser.json`.

Static review does not exercise live timing, automatic outfit/state transitions
or separately drawn held items. Shared registry, native metadata, combined
build, installation and repository-wide checks belong to the batch integration.
Source art, generated variants, previews and temporary helpers remain ignored.
