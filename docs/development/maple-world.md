# Maple: complete four-season world candidate

2026-11-06. **Authored and validated; displayed world artwork accepted.** This approved
Luc/Maple batch adds all 101 omitted Maple world strips through a new
`maple-world-trial` profile and set. All 32 portrait regions, ten previous source
roles, nine groups, four preset IDs/labels and previous mappings remain exact.
Shared registry, selection, planning and full collection verification belong to
the integrator. No runtime code or accepted baseline is changed.

## Source and materials

Source: `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The complete 133-strip export contains 266 original PNG/metadata members,
independently checked byte-for-byte against that archive. The new profile pins
every PNG. Native NPC definitions and scoped evidence remain under
`local/authoring/maple-world/`; no archive copy was created.

| Outfit | Strips | Native frames | West mirrors | Selected skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 23 | 71 | 24 | 2,360 |
| Spring | 29 | 80 | 24 | 3,233 |
| Summer | 26 | 77 | 24 | 3,427 |
| Winter | 23 | 71 | 24 | 2,184 |
| Total | 101 | 299 | 96 | 11,204 |

Each outfit covers action, blink, drink, eat, idle, sit and walk in the native
directions, plus scratch-head start/loop/end. Spring and Summer include doll
playing start/loop/end; Spring includes the three shocked strips. Metadata fixes
80×80 frames, native durations, Default atlas and offsets (40,54). West is the
NPC's native mirror of East; there are no invented West source strips.

There are 186 exact decoded unique native frames, 113 duplicate occurrences,
65 unique West mirrors and 178 native mask shapes. The 6,442 new component seeds
select face and eyelid skin, ears, neck, hands and exposed Spring/Summer legs.
Every native frame contains skin, from 2 to 73 selected pixels; no empty mask or
no-change disposition is needed.

World skin is `#96623B` light, `#83462E` medium, `#713527` shadow and `#57271D`
deep. Only `#96623B` is appended, in one singleton group, using each preset's
existing light target. The other three roles and targets already exist. Brown
hair, seasonal bows, eyes, true mouth interiors, clothing, socks/shoes,
drink/food and the doll retain original material colors. Eating/shocked mouths
retain dark `#410808` and red `#C83E37`; adjacent ordinary skin changes.

The doll deliberately remains brown: it shares Maple's light/medium shades.
Source coordinate grids and all twelve doll-playing occurrences identify
**168 doll-face pixels** (84 per outfit) separately from Maple's moving hands
and legs. The initial broad color selection incorrectly included those pixels;
the completed candidate excludes all doll faces. Literal doll-face and adjacent
hand checks cover this distinction independently of component seeds. Hair and
the pink doll dress are also preserved. The source grids, corrected contacts,
negative spill recipe and failure log retain this evidence.

## Review and author inspection

- [Five-choice SAMPLE](../../generated/maple-world-finish-review/summary.png):
  Autumn, Spring, Summer and Winter rows; Vanilla, Debug Blue, Hayden, Ryis and
  Seridia columns. A literal raster proof verifies the entire sample heading
  and five labels: zero mismatches across 88,000 header pixels.
- [Complete every-frame gallery](../../generated/maple-world-finish-review/index.html):
  all 299 native frames and 96 West mirrors, 395 cases across 157 small case
  pages and four outfit indexes. Vanilla is left, Debug Blue right; each side
  includes the entire 80×80 frame and enlarged face/hand/leg details.
- [Component authoring reconstruction](../../generated/maple-world-mask-review/index.html):
  all 299 occurrences reconstruct from candidate reference masks, with zero
  conflicts or unresolved review groups. Reuse does not imply prior acceptance.
- Bundle: `generated/maple-world-finish-build/`. Complete source export:
  `extracted/maple-world-finish-study/`, with stable corpus link
  `extracted/test-corpus/maple`.

The author inspected all 186 unique native original/candidate bodies and all
65 unique West mirror bodies in 16 native and six mirrored contact pages.
Those 6× details contain every opaque pixel; decoded identity accounts for all
duplicates. Source coordinate grids and representative full-frame/detail pages
additionally check mouth fringes/interiors, eyelids, scratch-head hands, raised
shocked hands, doll faces/adjacent hands and Spring/Summer legs. The corrected
doll-playing contacts and final full/detail page were inspected again after
excluding doll skin. All five sample choices were inspected.

The gallery binds 80,263,680 rendered pixels to final source/bundle images:
790 full source/Blue views, 790 enlarged details and 20 sampled five-choice cells.
All case identities/counts agree with original animation metadata and native
mirrors; full-frame crops hide no visible art. CPU Chromium passes 167 page loads: every
desktop page and all five mobile indexes with GPU disabled, with all images/links
present and no viewport overflow. The final browser
result and complete file fingerprints are in `check-results.json` beside the
gallery. Author inspection and numerical checks are separate from user approval
and natural gameplay.

## Focused verification

Fresh release checks passed:

```sh
nix-shell --pure --run 'rustfmt --check --edition 2024 tests/maple_world.rs && cargo clippy --locked --test maple_world -- -D warnings'
nix-shell --pure --run 'cargo test --release --locked --test maple_world --test maple_portraits -- --include-ignored'
target/release/mistria-palette build-presets \
  --original extracted/maple-world-finish-study \
  --presets palettes/sets/maple-world-trial.json \
  --output generated/maple-world-finish-build
python3 local/authoring/maple-world/verify.py
python3 local/authoring/maple-world/verify-retained.py
node local/authoring/maple-world/browser.mjs
```

Use fresh output directories for CLI rebuilds/reviews. Two regular tests check
complete source membership, preserved portrait roles and all 101 native package
registrations, including synthetic frames/timing/offset/atlas/pixel round trips.
The opt-in material test checks every RGBA pixel in all four targets with
source-based material expectations, fixed per-frame skin counts and literal
anatomical/protected landmarks. It checks original metadata and compares all
256 retained portrait PNG/metadata files against a prior-profile rebuild. An
independent comparison checks the actual final bundle against that same prior
profile; all 256 files are identical. Both unchanged Maple portrait material
tests pass. All four target variants pass exact CLI recipe validation across
all 133 strips, and all source members agree with the original archive.

Two isolated negative controls produce useful literal material failures:
omitting the Autumn front-idle hand component fails at `(35,47)`; selecting the
Spring doll's light face component fails at `(39,45)`. Both fail independently
of a seed-list comparison. Neither changes the candidate. The opt-in test uses
`FOM_MAPLE_WORLD_PRESETS` to select these retained negative recipes.

Profile SHA-256:
`9eac8183913e25e27dedcbb5056dcce2160bf7ac78585164104f41038e09b385`.
Set SHA-256:
`8a567676369d75cf296568a488a0ff98f28b3a138c95f8e9f129b24a49ca9ae1`.
Test SHA-256:
`e026a10ac0e207e1d5eb27858f778b61b67b898536dcdd14bfc058924a4fb916`.
Summary SHA-256:
`dfe3c2efbf45cc430636745fad6f5e1b9a8476512865ca598a3d0220e01b03d8`.

The authoring directory retains source inventories/grids, material counts,
focused check/negative logs, gallery bindings, browser reports and final input/
output fingerprints. A superseded 16 MiB initial bundle was removed after its
shared-doll-color defect was resolved; the final bundle and reproduction
sources remain. Complete measurements and scoped artifact sizes are retained
with `check-results.json`. Candidate authoring/checks took 660.1 seconds of agent
wall time including source inspection and overlapping compilation/browser work;
manual inspection was not separately timed. The final browser pass took 108.0
seconds. Approximately 2 MiB of source export, 16 MiB of bundle, 7 MiB of final
gallery and 30 MiB of source grids/contact evidence remain. Six disposable
compiled helpers were removed; their Rust sources are retained. No source snapshot or accepted baseline was removed.

The [combined batch record](sixth-finish-batch-2026-11-06.md) owns standard
combined checks, full collection installation/removal and one consolidated
review, all completed. The user accepted displayed Luc/Maple world artwork on
2026-11-06; inspection depth is unspecified and no live rendering/input/gameplay
acceptance is inferred. The accepted regression snapshot is
`local/fixtures/finish-maple/`; earlier approval limits remain unchanged. No local
GPU or local model inference was used.
