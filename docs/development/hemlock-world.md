# Hemlock: complete four-season world candidate

2026-11-06. **Authored and validated; displayed world artwork accepted.** The approved
Errol/Hemlock batch adds all 120 omitted Hemlock world strips through new
`hemlock-world-trial` profile/set files. The 32 portrait regions, seven prior
source roles/groups and four preset IDs, labels and existing mappings remain
exact. Shared selection, registry, planning and full collection integration
belong to the integrator. No runtime code or previously accepted artwork changes.

## Source and materials

Source archive: `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The complete 152-strip export contains 304 PNG/metadata members, all byte-identical
to that archive. Each source PNG is pinned in the profile. The native NPC
definition and source inspection reports remain under
`local/authoring/hemlock-world/`; no archive was copied.

| Outfit | Strips | Native frames | West mirrors | Selected pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 30 | 118 | 33 | 4,923 |
| Spring | 33 | 121 | 33 | 5,381 |
| Summer | 30 | 118 | 33 | 5,927 |
| Winter | 27 | 104 | 33 | 3,898 |
| Total | 120 | 461 | 132 | 20,129 |

Each outfit includes action, blink, drink, eat, idle, sit and walk cycles in its
native directions, plus 20-frame North chopping and East polishing/South writing
start/loop/end cycles. Autumn, Spring and Summer also have lute start/loop/end;
Spring has the three shocked strips. Native metadata fixes all frame counts,
durations, 80×80 geometry, Default atlas and offsets (40,54). West is the native
mirror of East; no invented West source strips are registered.

There are 306 exact decoded unique native frames and 155 duplicate occurrences,
100 unique East-derived West frames, and 228 unique native mask shapes. The
11,582 new component seeds select exposed face/ears, eyelid skin, cheeks and
chin, neck, arms/hands, ankles and Summer toes. Chopping hand grips, writing and
lute fingers, raised shocked hands and the ordinary skin beside open mouths
are included. Every native frame has skin: selected counts range from 8 to 81.
No empty mask or no-change disposition is needed.

World skin is `#C48E68` light, `#B36844` medium, `#8E4538` shadow and `#712922`
deep. Two singleton groups are appended: `#C48E68` uses each preset's existing
light target; `#8E4538` uses its existing shadow target. The medium/deep world
shades already exist. Existing portrait roles and mappings keep their positions.
Source inspection found no clothing/object use of these four world shades.

Gray hair, sideburn/stubble shading, eye whites/pupils, true mouth interiors,
clothes, cuffs, footwear, axe/effects, lute/notes, polishing cloth, book/quill and
drink/food remain original. In particular, the pink Autumn blouse, Spring
necktie/apron, Summer tan vest/white sleeves and Winter pink scarf/cuffs are
fabric, confirmed against all four original neutral portrait strips. The
Winter coat's `#4C242C` is an existing **portrait** deep role, but is deliberately
unselected in world masks. Open eating/lute/shocked mouths retain dark
`#410808` and red `#9E2626`; adjacent brown skin follows the target. Existing
portrait hair/stubble treatment and its recorded appearance limits are preserved.

## Review artifacts and inspection

- [Five-choice SAMPLE](../../generated/hemlock-world-finish-review/summary.png):
  four outfit rows; Vanilla, Debug Blue, Hayden, Ryis and Seridia columns. Native
  raster checks prove the full **HEMLOCK WORLD SPRITES — SAMPLE ONLY** heading
  and all five column identities, with zero mismatches over 88,000 header pixels.
- [Complete every-frame gallery](../../generated/hemlock-world-finish-review/index.html):
  461 native frames plus 132 West mirrors, 593 cases across 210 small case pages
  and four outfit indexes. Vanilla is left and Debug Blue right. Each side has
  the entire 80×80 source frame and enlarged face/hand/foot details.
- [Component authoring gallery](../../generated/hemlock-world-mask-review/index.html):
  all 461 occurrences resolve against the new profile; zero conflicting or
  unresolved groups. Its “reused” count means reconstruction from the candidate
  reference masks, and carries no historical art acceptance.
- Bundle: `generated/hemlock-world-finish-build/`. Complete source export:
  `extracted/hemlock-world-finish-study/`, with stable link
  `extracted/test-corpus/hemlock`.

The author inspected 20 contact pages covering all 306 unique native frames,
plus seven actual-bundle West contact pages covering all 100 unique mirrors;
decoded identity accounts for every duplicate occurrence. Coordinate/color
grids and enlarged originals additionally covered all outfits, eyes, mouth
fringes/interiors, hands, ankles/toes, chopping grips, lute, polishing, writing
and shocked poses. Representative final gallery detail pages and all five
sample choices were also inspected. This author assessment is separate from
user art acceptance and natural gameplay.

The gallery reconciles every case with native source metadata and verifies
120,104,448 rendered pixels against the actual final source/bundle, including
1,186 complete source/Blue views, 1,186 enlarged details and 20 sample cells.
Every full frame includes all opaque pixels, including effects outside the body.
CPU Chromium loaded all 215 desktop pages and five mobile indexes: all links
and images loaded, with no viewport overflow. No copied character titles remain.

## Focused verification and negative controls

Fresh checks passed:

```sh
nix-shell --pure --run 'rustfmt --check --edition 2024 tests/hemlock_world.rs && cargo clippy --locked --test hemlock_world -- -D warnings'
nix-shell --pure --run 'cargo test --locked --test hemlock_world --test hemlock_portraits -- --include-ignored'
target/release/mistria-palette build-presets \
  --original extracted/hemlock-world-finish-study \
  --presets palettes/sets/hemlock-world-trial.json \
  --output generated/hemlock-world-finish-build
python3 local/authoring/hemlock-world/verify.py
python3 local/authoring/hemlock-world/verify-retained.py
node local/authoring/hemlock-world/browser.mjs
```

Use fresh output names when reproducing CLI builds/reviews. The two regular new
tests check complete membership, preserved portrait roles and all 120 native
package registrations with synthetic frame/timing/offset/atlas/pixel checks.
The opt-in material test checks every RGBA pixel in all four targets against
source material expectations independent of author seeds, fixed per-frame
counts, literal anatomical/protected landmarks and literal native metadata.
It also compares 256 retained portrait PNG/metadata files with a prior-profile
rebuild. An independent release-build comparison against the actual final
bundle confirms those same 256 files are byte-identical. The existing Hemlock
portrait material test passes unchanged. All four choices pass exact CLI
recipe validation over all 152 source strips.

Two isolated negative recipes produce useful failures without changing the
candidate: omitting the Autumn front-idle hand component fails at `(32,44)`;
selecting Winter coat material with the already-mapped `#4C242C` fails at
`(36,42)`. Both failures are the independent literal material assertions, not a
seed-list comparison. Mutated profiles/sets and failure logs remain in the
authoring directory. `FOM_HEMLOCK_WORLD_PRESETS` selects them for the opt-in test.
No source material defect required changing prior art.

Profile SHA-256:
`2a54f4c488e49e9fd32e2707b7796de1a6be4017f121b6d6b13501067ac10013`.
Set SHA-256:
`348be57f587d88ef50e698491a9ee727c029a050e26d392b0c19ed8e5467f03a`.
Test SHA-256:
`5304426120cfe2b7be3353110ea5c2701cdef0bc16d21818085a5203c55f0396`.
Summary SHA-256:
`971481803c5b87d498e44d5e04d86bf781882be2b78ce3f86f5d9ff69a0d6ef3`.
`check-results.json` beside the gallery pins review/config/coverage hashes,
counts, measurements and limits; the authoring directory retains the complete
input/output hash manifest and all focused reports.

The measured interval from source work start (08:02:34 UTC) to checked candidate
(08:19:11.901 UTC) is 997.901 seconds. This is agent wall time including source
inspection, authoring, compiler work and overlapping checks; manual inspection
was not separately timed. Export took 0.348 seconds; seed generation 0.177;
final gallery generation/binding 6.189; independent validation/negative probes
30.145; the material/retention test group 21.17; the unchanged portrait test
1.72; and 220 browser loads 137.1. These overlapping timings are not additive.

Retained ignored artifacts are approximately 3.2 MiB of authoring reports/helpers,
2.4 MiB of source export, 20 MiB of bundle, 9.7 MiB of final gallery, 5.5 MiB of
component review, and 57 MiB of scoped source grids/originals/contact evidence in
`tmp/hemlock-world-*`. Temporary prior-build copies were automatically removed;
no accepted baseline, archive or unrelated scratch data was replaced or deleted.

Full standard checks, combined collection installation/removal and consolidated
review passed; the [batch record](fifth-finish-batch-2026-11-06.md) owns that evidence
and the user's 2026-11-06 acceptance of displayed Errol/Hemlock world artwork.
Inspection depth is unspecified. These CPU checks and displayed-art approval do
not assess physical rendering, input or natural gameplay. The accepted regression
snapshot is `local/fixtures/finish-hemlock/`; all earlier recorded approval
qualifications remain unchanged.
