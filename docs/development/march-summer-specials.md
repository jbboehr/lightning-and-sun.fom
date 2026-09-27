# March Summer smithing animations

Adds hammer East, wipebrow South and work_sit start/loop/end North: five strips, 23 source frames and 19 distinct images. March now has 264 sources and 1,056 recolored variants. The native seated-work cycle is complex; hammer and wipebrow are linear. Native rendering supports West using the East hammer pack with `image_xscale = -1`; the review therefore includes seven West mirrors, for 30 cases. This does not assert that the natural work schedule chooses West.

## Source and material boundaries

The game archive remains read-only and every PNG is pinned to its exact SHA-256. All five strips use 80×80 frames and the `Default` atlas with vertical offset 54. Hammer and wipebrow preserve numeric horizontal offset 40; the work_sit strips preserve `Middle`.

Hammer has seven frames at 0.1 seconds. Wipebrow has six durations `[0.15, 0.15, 0.45, 0.1, 0.075, 1.0]`. Work_sit start has one frame with default timing, loop has seven durations `[0.5, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25]`, and end has two durations `[0.25, 0.175]`. Native seated behavior and hammer sounds are retained by the game; the recoloring adds no behavior changes.

The five new regions add 565 explicit component seeds, selecting 894 skin pixels per target using the existing `#EEDDA5`, `#E8B271`, `#D37A57` and `#7D3B14` roles. These cover the exposed Summer arms and ankles, hammer grips, raised wipebrow hand and small neck opening beneath the hair. No source colors, groups or target mappings change.

The handle's `#E3B57F`, `#BB8151` and `#936244` remain wood. Blue metal, pale swing trail, yellow sparks, red hair, brown apron, pale collar, green shorts, footwear and mouth interiors remain original. Every actual frame was inspected in Vanilla and all four target palettes. The artwork reaches x76/y63, so the complete review uses crop `[0,22,80,44]`, preserving the entire hammer, trail and sparks in both directions.

## Verification

`tests/march_summer_specials.rs` uses 36 literal source-art landmarks across moving skin and protected materials. The opt-in test checks strict source hashes, exact source-or-target pixels, alpha, dimensions, raw metadata and the same selected mask in all four targets. All 259 earlier region objects, sixteen source colors, seven groups and mappings remain unchanged. All 2,590 earlier original/variant PNG and metadata files are compared against `generated/characters-reina-juniper-march-summer-reading-trial/characters/march`.

Temporary negative controls fail at the intended boundaries: an omitted deep grip pixel at hammer frame 0 `[43,47]` and a recolored wooden handle highlight at `[45,45]`. The focused candidate test, targeted Clippy, formatting and normal test all passed. All 1,056 final variants strictly validate and match the reviewed candidate bytes.

Selection SHA-256: `5c6b70aa43ddfe3b0ba06d0c40bf7ed513371aeb0a89b46385f98c0f94848c71`.

```sh
nix-shell --pure --run 'cargo test --test march_summer_specials'
nix-shell --pure --run 'cargo test --test march_summer_specials -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_summer_specials -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-summer-specials-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-summer-specials-trial
```

Local previews:

- [Five-choice summary](../../generated/march-summer-specials-preview/summary.png).
- [Every frame in Vanilla and Debug Blue](../../generated/march-summer-specials-preview/blue-review/index.html), across five detail pages, including seven native West hammer mirrors.

Preview checks cover exact source/palette bindings, hashes, complete crops, raw metadata, every displayed pixel and West mirroring. The checker passed 5,280,000 gallery pixels and 880,000 summary pixels. Chromium passed all six pages, 31 images and their links. Static images do not demonstrate animation timing or sound. Combined installation and native runtime checks are handled separately by the integration slice. All game images and generated artifacts remain ignored and uncommitted.
