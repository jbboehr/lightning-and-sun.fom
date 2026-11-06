# Terithia: complete four-season world candidate

2026-11-06. **Authored and validated; displayed world artwork accepted.** This approved
finish batch adds all 131 omitted Terithia world strips through the new
`terithia-world-trial` profile and set. All 32 portrait regions, eleven source
roles, eleven singleton color groups, four preset IDs/labels and every existing
target mapping remain exact. No source color or palette choice is added.
Shared registry, selection, planning and full collection verification belong to
the integrator; this record covers the character candidate.

## Source and material decisions

Source: `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The complete 163-strip export contains 326 original PNG/metadata members,
checked byte-for-byte against that archive. Every PNG is pinned in the profile.
The native NPC definition and source hashes remain under
`local/authoring/terithia-world/`; no archive copy was created.

| Outfit | Strips | Native frames | West mirrors | Skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 32 | 105 | 43 | 5,277 |
| Spring | 35 | 108 | 43 | 6,452 |
| Summer | 32 | 105 | 43 | 6,535 |
| Winter | 32 | 105 | 43 | 4,663 |
| Total | 131 | 423 | 172 | 22,927 |

Each outfit covers action, blink, drink, eat, idle, sit and walk in native
directions, plus East/South bait and fishing start/loop/end cycles. Spring also
includes all three shocked strips. Original metadata specifies 80×80 frames,
native durations, Default atlas and offsets (40,54). West is the NPC's native
horizontal mirror of East; there are no invented West source strips.

There are 301 exact decoded unique native frames and 122 duplicate occurrences;
the 172 West cases contain 132 unique mirrored bodies. The 12,667 component seeds
produce 252 distinct native mask shapes. Every frame has exposed skin, from
4 to 97 selected pixels; no empty mask or no-change disposition is needed.

World skin uses existing `#D29A7D` light, `#AD7465` shadow, `#7A4D45` deep/scar and
`#583131` deepest roles. Their existing targets are retained, including the two
deep shades sharing the same darkest target. Selected pixels cover exposed
face/scar shading, eyelid and mouth fringes, neck, arms, fingers, legs and toes.
Gray hair, seasonal hats, eyes, genuine dark/red mouth interiors, clothes, bag,
belt, sandal straps, Winter boots, rod, bait/fish and drink/food remain original.

The two deep colors also occur on bags, belts and sandal straps. A broad initial
color candidate incorrectly included these materials. Source coordinate grids
and final original/Blue contacts distinguish **2,281 preserved shared-color
pixels**: 650 `#7A4D45` and 1,631 `#583131`. The completed candidate excludes them;
literal adjacent hand/bag and foot/strap assertions test that separation. Summer
uses different bag shades, which remain original too. Source-inspected material
coordinates and fixed per-frame skin counts are independent of recipe seeds.

## Complete review and author inspection

- [Five-choice SAMPLE](../../generated/terithia-world-finish-review/summary.png):
  Autumn, Spring, Summer and Winter rows; Vanilla, Debug Blue, Hayden, Ryis and
  Seridia columns. A literal raster proof verifies all 88,000 header pixels,
  including the complete title and all five labels, with zero mismatches.
- [Complete every-frame gallery](../../generated/terithia-world-finish-review/index.html):
  all 423 native frames and 172 West mirrors, **595 cases** across 235 small
  comparison pages and four outfit indexes. Vanilla is left, Debug Blue right;
  both include the full 80×80 frame and enlarged body/face/hand/foot details.
- [Component reconstruction](../../generated/terithia-world-mask-review/index.html):
  all 423 native occurrences reconstruct from final candidate reference masks,
  with zero conflicting groups or unresolved review groups. This is candidate
  reuse, not a record of user approval.
- Final bundle: `generated/terithia-world-finish-build/`. Source export:
  `extracted/terithia-world-finish-study/`, with stable corpus link
  `extracted/test-corpus/terithia`.

The author inspected every unique original/Blue native body in 26 contact pages
and every unique West mirror in eleven more. Each contact contains the full
80×80 frames plus 6× body details; exact decoded identity accounts for duplicate
occurrences. Source grids separately check face/scars, eating mouths, moving
hands, fishing grips, bag/belt shadows, sandals and extended fishing-start toes.
The full frames retain the complete rod and line, including pixels outside the
body detail crop. All selected skin fits inside the enlarged body detail crop;
leaning fishing-start toes reach row 57. All five sample choices were inspected.

The gallery binds **132,768,640 rendered pixels** to final source/bundle images:
1,190 full views, 1,190 enlarged details and twenty five-choice sample cells.
Every case identity/count agrees with original metadata and native mirror rules;
full-frame crops hide no visible artwork. CPU Chromium passes **245 page loads**:
every desktop page and all five mobile indexes, with GPU disabled, images and
links present and no viewport overflow. `browser-check.json`, `gallery.log` and
`sample-labels` evidence are retained beside the authoring inputs. These checks
and author inspection do not establish user or natural-gameplay acceptance.

## Focused verification

Fresh focused release checks passed:

```sh
nix-shell --pure --run 'rustfmt --check --edition 2024 tests/terithia_world.rs && cargo clippy --locked --test terithia_world -- -D warnings'
nix-shell --pure --run 'cargo test --release --locked --test terithia_world --test terithia_portraits -- --include-ignored'
target/release/mistria-palette build-presets \
  --original extracted/terithia-world-finish-study \
  --presets palettes/sets/terithia-world-trial.json \
  --output generated/terithia-world-finish-build
python3 local/authoring/terithia-world/verify.py
python3 local/authoring/terithia-world/verify-retained.py
node local/authoring/terithia-world/browser.mjs
```

Use fresh output directories for CLI rebuilds. Two regular tests verify complete
source membership and preservation of prior roles, plus all 131 native package
registrations with synthetic frame/timing/offset/atlas/pixel round trips. The
opt-in source-material test checks every RGBA pixel in every world frame and all
four targets using fixed source-material exclusions, per-frame skin counts and
literal anatomical/protected landmarks. Original dimensions, alpha and metadata
remain exact. All **256 retained portrait PNG/metadata files** match a prior-profile
rebuild, both within the focused test and independently against the actual final
bundle. The retained portrait skin/scar/material test also passes. All four
presets pass exact CLI recipe validation over all 163 strips.

Isolated omission and spill negatives fail at their intended literal material
assertions: removing the Autumn front-idle hand component fails at `(32,46)`;
selecting the adjacent bag/belt component fails at `(39,46)`. Neither changes
the final candidate. `FOM_TERITHIA_WORLD_PRESETS` selects these local negative
recipes. Initial source-landmark transcription errors and the nominal row-55
body bound were corrected against original pixels; the latter now explicitly
covers leaning fishing-start toes at row 57. Those test-authoring corrections
changed no artwork.

Profile SHA-256:
`570f4a546674f682e03ac643c8aaf3e74892237a6e9cbd654a8fdf421eae0360`.
Set SHA-256:
`9cb83b02cbf61759b2b2449865f5a38931c14da789a6e394fa8e19a294c8329b`.
Test SHA-256:
`138e3042f667b162f14f813fc48dc4c17d00472f405f7f7ad3fefe8120fc1d5e`.
Summary SHA-256:
`f3182cdde4a2d3c3cea39c2b75ec7ef29181b962afed362c70ae114ae90d4d0e`.

The authoring directory retains inventories, material counts and source grids,
final contact inspection, focused/negative logs, browser reports, gallery
bindings and final input/output fingerprints. The initial superseded 24 MiB
broad-color bundle remains local as comparison evidence; it is not selected.
Approximately 3 MiB of source export, 24 MiB of final bundle, 12 MiB of complete
gallery and 6 MiB of mask reconstruction also remain. Measurements and exact
scoped retained sizes are in `check-results.json`. Source grids and contact
inspection retain another 55 MiB. Seven disposable compiled helpers (38 MiB)
were removed after verification; their Rust sources, source snapshots and accepted
fixtures remain. Candidate authoring, source inspection and overlapping checks
took 648.9 seconds of agent wall time; manual inspection was not separately
timed. The final CPU browser pass took 118.8 seconds.

The [combined batch record](seventh-finish-batch-2026-11-06.md) owns standard
combined checks, full collection installation/removal and the consolidated art
review, all completed. The user accepted displayed Terithia world artwork on
2026-11-06; inspection depth is unspecified and no live rendering/input/gameplay
acceptance is inferred. The accepted regression snapshot is
`local/fixtures/finish-terithia/`; earlier approval limits remain unchanged. No
local GPU or local model inference was used.
