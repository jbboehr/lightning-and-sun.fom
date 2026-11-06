# Vera: complete four-season world candidate

2026-11-05. **Authored, integrated and validated; displayed artwork accepted.**
This approved P2 batch adds all 60 world strips to a new `vera-world-trial`
profile/set. All 32 existing portrait regions, six source colors, singleton
groups and four preset mappings remain exact. The integrator owns shared
registry/count/collection changes. No runtime code or accepted artwork changes.

## Source and material decisions

The source archive is `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 184 PNG/metadata members in the complete export match the archive byte-for-byte;
every source PNG is hash-pinned in the profile. Native NPC source and metadata
evidence remain under `local/authoring/vera-world/`.

Each outfit has 15 strips: action East/North/South (seven frames each), blink
East/South (three), eat South (five), idle and sit East/North/South (one), and
walk East/North/South (four). The native Vera definition and animation metadata
establish all four outfits and cycles. There are 200 frame occurrences, 121
exact decoded unique frames and 79 duplicates. There were no accepted world
masks to reuse; repeated newly authored candidates do not establish acceptance.
The final selections have 111 distinct mask shapes.

World skin uses four existing shades: `#B68454` light, `#A15F3F` medium,
`#7F402B` shadow and `#541C1E` deep. No colors or mapping roles were added.
The 6,625 new seeds select 11,105 pixels per target: Autumn 2,828, Spring 2,995,
Summer 2,920 and Winter 2,362.

Selected materials include face, ears, eyelid skin and face contours, neck,
exposed shoulders and arms, hands, ankles and visible toes. Winter's small skin
cutouts between the collar and sleeves are included, as do exposed ankles
above the boots. Spring/Summer open footwear retains its straps while exposed
ankles/toes follow the target. Autumn's exposed toes also change. Every strip,
including all North and eating cases, has visible skin; no empty masks or
unselected no-change strips are needed.

Magenta hair, eyes, earrings, mouth interiors, food, seasonal garments and
footwear materials keep their source pixels. Summer's peach collar/scarf colors
`#FFD6BF`, `#EF9F88` and `#D77C6D` are fabric, confirmed by the source Summer
portrait; they remain original beside the recolored exposed shoulders. Winter's
pale cape and brown coat/boots (`#5F3931`, `#7F5443`) also remain original. All
world occurrences of the four skin shades are anatomical skin or its fine
shading in this corpus. No matching-color clothing exclusion was required;
component masks still constrain application.

## Candidate review and coverage

- [Five-choice SAMPLE overview](../../generated/vera-world-finish-review/summary.png):
  one front idle frame per outfit, columns Vanilla, Debug Blue, Hayden, Ryis and
  Seridia. The visible heading reads **VERA WORLD SPRITES**.
- [Complete every-frame review](../../generated/vera-world-finish-review/index.html):
  200 native frames plus 64 native West mirrors, 264 cases over 100 small case
  pages and four outfit indexes. Vanilla is left and Debug Blue right; both show
  the complete visible sprite with enlarged face/hand details.
- [Component authoring gallery](../../generated/vera-world-mask-review/index.html):
  all 200 occurrences reconstruct from the candidate with no conflicts or
  unresolved groups. Reference-mask status is tool evidence, not user approval.
- Bundle/package: `generated/vera-world-finish-build/`. Complete source export:
  `extracted/vera-world-finish-study/`, linked from `extracted/test-corpus/vera`.
  Historical accepted portrait sources and outputs remain untouched.

The author inspected every native source/candidate case and all West
source/Blue mirrors in seasonal sheets. Coordinate/color views additionally
covered face/eyes, hands, eating poses, Summer collar and feet, and Winter
shoulder/ankle boundaries. Source eating poses raise the forehead to row 30;
the full sprite crop includes that and every other opaque pixel. Author
inspection is separate from user acceptance and natural gameplay.

The gallery helper reconciles all cases with source metadata, proves complete
visible bounds in both directions, and checks 31,398,912 enlarged preview pixels
against the actual current bundle. This includes 528 full Vanilla/Blue views,
528 detail views and 20 sampled palette cells. CPU Chromium loaded all 105
desktop pages plus five mobile indexes: all links/images loaded and no viewport
overflowed. The 110 page loads took 61.31s. Scoped text checks found no copied
other-character headings, and the rendered summary title was visually inspected.

## Focused checks and limits

Fresh checks passed:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --release --locked --test vera_world -- -D warnings && cargo test --release --locked --test vera_world --test vera_portraits -- --include-ignored'
target/release/mistria-palette build-presets \
  --original extracted/vera-world-finish-study \
  --presets palettes/sets/vera-world-trial.json \
  --output generated/vera-world-finish-build
target/release/mistria-palette review-batch \
  --archive local/game/assets.zip \
  --config palettes/review/vera-world.json \
  --output generated/vera-world-mask-review
```

Use fresh output names when reproducing. Two regular tests preserve portrait
definitions/roles and complete seasonal membership, then exercise native package
frames, timing, offsets, atlas and pixels across all 60 synthetic strips.
The opt-in source test checks every world RGBA pixel in all four targets using
independent material expectations, fixed per-frame counts and literal source
skin/protected landmarks. These include fine face shades, Summer arms/collar,
Winter shoulder cutouts, ankles/toes, earrings, eyes, hair and seasonal clothes.
All metadata bytes are preserved. A prior-profile rebuild compares all 256
retained portrait PNG/metadata files byte-for-byte. The existing Vera portrait
material and speaking-mouth regression test also passed unchanged.

Each preset passed CLI exact-recipe validation over all 92 source strips,
including geometry, alpha, metadata and source hashes. Two deliberate negative
recipes failed at the intended independent landmarks: removing Winter shoulder
skin failed at `(37,42)`; selecting Summer peach collar fabric failed at the
same local coordinate in the Summer idle sprite. The final candidate passed
after both negative probes. Their profiles, sets and failure logs remain in the
authoring directory. `FOM_VERA_WORLD_PRESETS` selects a negative recipe only for
the opt-in world-material test.

Profile SHA-256:
`1acb238a0a02d8c2c86055b1a14d478f9b224cd0e838958a9e91fe8e2931c267`.
Set SHA-256:
`11a3e3b35f0ac24d711a7fd9cdffb17257766661c52214c250f648bf78a53412`.
The review's `check-results.json` pins inputs and coverage. Reproducible Rust
author/gallery helpers, source grids, native NPC definition, tests, validation
reports and measurements are retained under `local/authoring/vera-world/`.

The source-export start to checked-candidate checkpoint took 5m15s elapsed,
including source study, authoring, all-case inspection, focused tests and browser
checks; documentation and combined integration are excluded. The final source
test took 0.61s, gallery generation/pixel checks 1.76s, and four CLI validations
plus archive/source checks 1.47s. Human review time is unmeasured. No local GPU
or local model inference was used.

The user accepted the displayed artwork with "looks good to me" after the
consolidated review. Inspection depth is unspecified. The parent
[batch record](third-finish-batch-2026-11-05.md) owns full collection checks,
isolated install/remove and the frozen accepted snapshot;
live rendering/input and gameplay remain release gates. Other browsers remain
unverified. Game-derived sources, images and packages stay ignored. The retained
source, bundle, review, authoring gallery and evidence occupy about 23 MiB;
inspection sheets and executable helpers remain under the project `tmp/`.
