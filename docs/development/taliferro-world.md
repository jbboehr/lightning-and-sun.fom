# Taliferro: complete four-season world candidate

2026-11-05. **Authored, integrated and validated; displayed artwork accepted.**
This approved [finish-plan](../plans/finish.md) batch adds all 60 world strips to
`taliferro-world-trial`: 15 native strips per season, including eating. It retains
all 36 existing portrait regions, eight source-color roles, singleton groups and
four preset mappings exactly. No earlier accepted artwork or runtime behavior
changes. The integrator owns shared registry/collection selection and checks.

## Source and material decisions

The preserved archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
A fresh whole-character export contains all 96 selected PNG/metadata pairs. Each
PNG is pinned in the profile; both PNG and metadata hashes, native properties and
archive identity are recorded in `local/authoring/taliferro-world/export.json`.
The native NPC definition is preserved beside it.

Every season has action East/North/South (seven frames each), blink East/South
(three each), eating South (five), idle and seated East/North/South (one each),
and walk East/North/South (four each). The complete world set has 200 native frame
occurrences, 124 exact decoded unique frames and 76 duplicate occurrences. The
native East strips supply another 64 West mirrors. No previously accepted world
masks existed to reuse; duplicate reuse here is of newly authored candidate art.

World skin uses the existing `#FFDBB8`, `#F2A174`, `#C4613B` and `#762E21`
light/middle/shadow/deep roles. No added colors or altered role semantics are
needed. The other four existing portrait blend colors do not occur in these
world strips. Source inspection of all outfits and native eating frames found
that all occurrences of the four world shades are anatomical skin: face, ears,
eyelid/cheek contours, neck, hands, bare fingers and exposed Summer forearms.
Every strip contains visible skin, including back-facing ears, nape and hands;
none requires an empty mask or omission.

The dark facial contours remain distinct from actual brown hair/eyebrow edges
`#76462A`, while gold hair, bright hair highlights, eye whites, gray details and
black eye cores stay original. Native eating frame 3 has a large dark red mouth
interior: its `#9E2626` red and `#410808` dark interior remain source-colored;
the surrounding skin and raised hand follow the palette. Other native mouth
colors, cravats, clasps, buttons, Summer cream trousers, patterned outfits and
footwear also retain all source pixels. The world masks introduce no change to
the accepted portrait eyebrow treatment.

The 4,654 new component seeds select 8,883 skin pixels per target: Autumn 2,194;
Spring 2,287; Summer 2,290; Winter 2,112. No configured world-shade occurrences
require non-skin exclusions in this corpus. Component selections are source-bound
and contain no runtime spatial heuristic. The complete every-pixel source tests
still protect every unrelated color, rather than relying only on the configured
shades or current seed list.

## Review artifacts

- [Five-choice SAMPLE overview](../../generated/taliferro-world-finish-review/summary.png).
- [Complete offline review](../../generated/taliferro-world-finish-review/index.html):
  200 native frames plus 64 West mirrors, separated by outfit into 100 small case
  pages. Vanilla is left and Debug Blue right, with full visible sprites plus
  enlarged face/hand details. Eating and its changing head/hand positions are
  included in every season.
- [Component authoring gallery](../../generated/taliferro-world-mask-review/index.html):
  all 60 strips and 200 candidate masks reconstruct with no unresolved/conflicting
  groups. Tool reconstruction does not establish artwork acceptance.
- Sources: `extracted/taliferro-world-finish-study/`, also available through the
  fresh `extracted/test-corpus/taliferro` link. Historical portrait corpora and
  accepted outputs are retained.
- Current variants/package: `generated/taliferro-world-finish-build/`.
- Reproduction helpers, source pixel grids, negative controls, logs and frozen
  verification manifest: `local/authoring/taliferro-world/`.

The author inspected every native and mirrored Source/Blue case and enlarged
native eating-mouth cases, plus the five-choice seasonal sample. This is author
inspection, not user acceptance. The gallery checker reconciles every case with
native metadata, requires every full Vanilla/Blue binding and verifies that full
views hide no opaque source pixel. It compares 31,398,912 enlarged preview pixels
with the actual current bundle: 528 full bindings, 528 detail bindings and 20
sample cells.

CPU Chromium loaded all 105 desktop pages at a 1200-pixel window and five mobile
index pages at 390 pixels. All local links and images exist/load, with no
horizontal overflow. The 110 page loads took 74.230 seconds with GPU use disabled.
Other browser engines, rendered game output and physical input remain unverified.

## Focused checks

Fresh checks passed:

```sh
nix-shell --pure --run 'cargo test --release --locked --test taliferro_world --test taliferro_portraits -- --include-ignored'
nix-shell --pure --run 'cargo clippy --release --locked --test taliferro_world -- -D warnings'
```

Two regular world tests preserve the complete prior recipes and package all 60
synthetic native strips with all 200 frames, scalar/array durations, offsets and
atlas properties intact. Eating's five-value duration array is included. The
opt-in world test builds all 96 real sources, checks every world RGBA pixel in
all four targets, pins per-frame material counts, and preserves metadata bytes.
Independent literal landmarks identify deep face/hand skin, actual brown/gold
hair, cravat, red/dark eating-mouth interiors, raised fingers and four seasonal
garment shadows. The world test also rebuilds the former portraits and compares
all 288 prior PNG/metadata outputs byte-for-byte. The unchanged retained portrait
skin/hair/jewelry regression passes too.

Two temporary negative candidates demonstrate useful failures: omitting the
Autumn idle South palm component fails at `(32,45)`; adding the actual brown hair
shade and selecting its component fails at `(37,28)`. Both fail the every-pixel
material assertion, independently of profile reconstruction. Their exact
profiles/sets and failure logs are retained locally; neither affects the final
profile or bundle. No production material correction was required after source
inspection. Early test-authoring checks identified the eating frame's shifted
forehead row and the deliberately different seasonal garment colors; their
literal expectations were corrected from source inspection before the final
checks, without changing the artwork.

All four final presets also passed exact CLI recipe validation over all 96
selected strips with no source-hash override. Combined validation time was 0.343
seconds; the final focused world test run took 0.65 seconds after compilation.
Full collection generation, the standard combined suite and isolated installation
round trip remain integration work and are not claimed by this character batch.

## Measurements and limits

Archive hashing and source export took 0.509 seconds. Export completed at
`2026-11-06T03:03:40Z`; the passing material/retained tests and Clippy finished at
`03:07:49Z`, an observed 4m09s interval. Complete native/mirror and enlarged eating
inspection was recorded by `03:09:26Z`, 5m46s after export. These windows include
source study, component authoring, test preparation and candidate inspection;
context reading and initial helper compilation preceded export. Complete browser
checks and final documentation are additional. These are agent elapsed times for
simple repeated poses, not human-review timing or a forecast for complex forms.

All image work used CPU-native Rust tools with Node orchestration. No Python,
Pillow, model inference or GPU was used. Final sources, bundle, complete review,
component gallery and local authoring evidence total about 22 MiB, plus small
inspection sheets and shared Rust caches. Source and generated game imagery
remain ignored/local. No accepted baseline was overwritten.

Final profile SHA-256:
`5b0b663d434ddf70c8230cd5edbd03b719dbf9ba84c0d684e88c66307fb200fc`.
Final preset set SHA-256:
`4749f6b3f7bf8719fe25a1d4fedbcb785a3a6940d622b050ebdf3405cdd694bd`.
The user accepted the displayed artwork with "looks good to me" after the
consolidated review. Inspection depth is unspecified; live-gameplay acceptance
remains pending. The parent [batch record](third-finish-batch-2026-11-05.md)
owns full collection checks, installation/removal and the accepted snapshot.
