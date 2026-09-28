# March Winter smithing animations

Adds hammer East, wipebrow South and work_sit start/loop/end North: five strips, 23 source frames and 19 distinct images. March now has 355 sources and 1,420 recolored variants. The native renderer also supports seven West hammer mirrors, giving 30 review cases; this does not assert that the natural work schedule chooses West.

[Five-choice summary](../../generated/march-winter-specials-preview/summary.png) · [Complete Vanilla/Debug Blue review](../../generated/march-winter-specials-preview/blue-review/index.html)

Fresh hashes pin every source PNG. All strips use 80×80 frames on `Default` with vertical offset 54. Hammer and wipebrow retain numeric horizontal offset 40; the work_sit strips retain `Middle`. Raw sidecars remain exact: hammer has seven frames at `0.1`, wipebrow has six durations `[0.15, 0.15, 0.45, 0.1, 0.075, 1.0]`, work start has one frame with default timing, loop has seven durations `[0.5, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25]`, and end has two durations `[0.25, 0.175]`. Recoloring changes no native seated behavior, timing or sound.

Every actual frame was inspected in Vanilla, Debug Blue, Hayden, Ryis and Seridia. Exposed wrists around the gloves, face shading and neck skin follow the selected palette. The wooden handle (`#E3B57F`, `#BB8151`, `#936244`), blue metal, pale swing trail, yellow sparks, gloves, blue sleeves, brown apron, hair, headband, trousers, footwear and red mouth interior remain original.

Winter differs from Autumn: every seated-work loop frame exposes two rear-neck pixels. The first ending frame also recolors two pixels; the start and second ending frame each recolor six. These were checked against fresh Winter art, rather than retaining Autumn's empty loop mask.

The five regions add 241 explicit component seeds, selecting 389 pixels per target with the existing four skin shades. No source colors, groups or target mappings change. All 350 earlier region objects, source pins and seeds remain exact. All 3,500 earlier original/variant PNG and metadata files match `generated/characters-reina-juniper-march-winter-reading-trial/characters/march` byte for byte.

`tests/march_winter_specials.rs` checks 53 literal source-art landmarks, exact source-or-target pixels, alpha, dimensions, raw metadata, strict source hashes and equal selections across all four targets. Temporary controls failed at their intended assertions: omitted rear-neck skin in work loop frame index 0 `[39,39]`, and recolored wood in hammer frame index 0 `[45,45]`. The final corpus check, normal test, targeted Clippy and formatting passed. All 1,420 final variants strictly validate and match the reviewed candidate bytes. Selection SHA-256: `8756951044c9166def2ccd02cfb23137c1014866d365c8161357c5778459b506`.

The full gallery covers every source frame and all seven native West mirrors across five detail pages. Crop `[0,22,80,44]` includes the complete hammer swing and sparks; actual source bounds are `[29,25,76,63]`. Exact checks verify every displayed pixel, source/palette/frame binding, hash, raw sidecar, crop and mirror: 5,280,000 gallery pixels plus 880,000 summary pixels. Chromium passed all six pages, 31 decoded images and their links. Static checks do not claim interactive gameplay or demonstrate sound; shared integration handles native playback and isolated installation separately.

```sh
nix-shell --pure --run 'cargo test --test march_winter_specials'
nix-shell --pure --run 'cargo test --test march_winter_specials -- --ignored'
nix-shell --pure --run 'cargo clippy --test march_winter_specials -- -D warnings'
target/release/mistria-palette build-presets \
  --original extracted/march-winter-specials-study \
  --presets palettes/sets/march-world-trial.json \
  --output generated/march-winter-specials-trial
```

Local evidence under `tmp/march-winter-specials-author-*` includes raw metadata, source inventory, actual all-target sheets, controls, frozen hashes and validation/preservation/browser reports. Game images and generated artifacts remain ignored; the source archive is read-only.
