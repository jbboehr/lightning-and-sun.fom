# Stillwell: complete four-season world candidate

2026-11-05: authored and locally validated. **Displayed world art was accepted
with qualification on 2026-11-06; live play remains unverified.**
This approved [finish-plan](../plans/finish.md) candidate adds all 68 omitted world
strips. It preserves the prior 36 portrait regions, six color roles/groups and
preset mappings exactly. New world colors use the existing roles. Shared
registry, collection selection, planning and integrated checks belong to the
parent integrator; this batch changes no runtime behavior or accepted artwork.

## Sources and material decisions

Source archive SHA-256:
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The fresh whole-character corpus contains 104 PNG/metadata pairs, pinned in
`local/authoring/stillwell-world/export.json`. The profile pins every PNG hash and
size. Native NPC definitions and metadata properties are retained beside the
export record; the source archive itself was not copied or modified.

Each season contains action East/North/South (seven frames each), blink East/South
(three), eating East/South (five), eating North (three), idle and seated
East/North/South (one), and walk East/North/South (four). Eating North uses scalar
`duration=1`; eating East/South use the native five-value duration array. These
68 strips contain 232 native frame occurrences, 164 exact decoded unique frames
and 68 duplicate occurrences. East animation strips add 84 native West mirrors.
No previously accepted world masks existed; reused frame selections are newly
authored candidate masks.

The world ramp differs from the portrait ramp. Four new singleton colors follow
the existing target light/middle/shadow/deep roles:

| World source | Existing role | Selected material |
| --- | --- | --- |
| `#E7CE9F` | Light | Lit face, neck, hands and exposed body skin |
| `#CAA17E` | Middle | Face, neck, chest and hand shading |
| `#A06955` | Shadow | Face/chin, mouth-adjacent skin and finger shading |
| `#5A2F2C` | Deep | Narrow hairline/eye/face and finger skin contours |

All six prior portrait roles and their target values remain unchanged. Every
occurrence of the four new world colors is exposed skin in this source corpus;
there are no shared-color garment exclusions. Every strip contains skin,
including visible hands in the back-facing and seated/eating North animations.
No empty masks or unaccounted omissions are needed.

The source study includes original Summer and Winter portraits to distinguish
exposed shoulders/arms, chest gaps, fingertips and fingerless gloves from the
outfits. Those visible skin areas follow the palette. Blue hair and bangs, gray
and pale eye details, silver necklaces/clasps, real mouth interiors, gloves,
seasonal outfits and footwear retain their original pixels. Native East/South
eating frames include `#9E2626` red and `#410808` dark mouth interiors; both remain
source-colored. Surrounding cheek/chin skin and raised fingers recolor. Normal
black linework is preserved. Existing portrait lip treatments are unchanged.

The 4,863 new component seeds select 7,004 source pixels per target: Autumn 1,667,
Spring 1,811, Summer 1,859 and Winter 1,667. The profile stores hash-bound
components; no runtime spatial heuristic or global color replacement is added.

## Candidate review and verification

- [Five-choice SAMPLE summary](../../generated/stillwell-world-finish-review/summary.png),
  explicitly titled STILLWELL and sampling all four outfits.
- [Every-frame offline review](../../generated/stillwell-world-finish-review/index.html):
  232 native frames plus 84 West mirrors, separated into four outfits and 120
  small case pages. Vanilla is left and Debug Blue right. Each side includes a
  full visible sprite plus enlarged face/hand details.
- [Component authoring gallery](../../generated/stillwell-world-mask-review/index.html):
  all 232 candidate frame masks reconstruct with no conflicting/unresolved groups.
  This tool evidence does not establish artwork acceptance.
- Sources: `extracted/stillwell-world-finish-study/`, also available through
  `extracted/test-corpus/stillwell`. Historical accepted portrait corpora and
  outputs were not replaced.
- Bundle: `generated/stillwell-world-finish-build/`.
- Reproducible helpers, source pixel grids, negative controls, measurements and
  frozen verification manifest: `local/authoring/stillwell-world/`.

The author inspected all native and mirrored Source/Blue full views and all 316
face/hand detail pairs at 8×, plus the five-choice sample. The complete enlarged
inspection sheets are retained under `tmp/stillwell-world-detail-sheets/`. This records author inspection, not
user acceptance. The gallery checker reconciles every case against native
metadata, asserts that full views hide no opaque source pixel and verifies
37,429,248 enlarged preview pixels against the current bundle. It requires all
632 full Vanilla/Blue bindings, 632 detail bindings and 20 sample cells.

CPU Chromium loaded all 125 desktop HTML pages at a 1200-pixel window and five
mobile index pages at 390 pixels. All local images/links exist and load; no
horizontal overflow was observed. The 130 page loads took 59.606 seconds, with
GPU use disabled. Other browser engines and rendered game/input remain untested.

Fresh focused checks passed:

```sh
nix-shell --pure --run 'cargo test --release --locked --test stillwell_world --test stillwell_portraits -- --include-ignored'
nix-shell --pure --run 'cargo clippy --release --locked --test stillwell_world -- -D warnings'
```

Two regular tests preserve prior recipes and package all 68 synthetic native
strips while preserving 232 native frames, scalar/array durations, offsets and
atlas. The opt-in source test checks every world RGBA pixel in all four targets
against independent source-color/material expectations and fixed per-frame
counts. Its 240 literal source/target landmark pairs cover face/finger shading,
blue hair, gray eye, silver jewelry, eating-mouth interiors, surrounding skin
and raised fingers. All metadata bytes remain unchanged. Rebuilt prior portraits
match all 288 original PNG/metadata outputs byte-for-byte, and the unchanged
retained portrait material test also passes.

The first candidate misread the lit world shade as `#E7C99F`, which does not occur
in the sources. The independent source test failed on the unrecolored light pixel
in Autumn action East frame 2, local `(39,35)`. The source-verified correction to
`#E7CE9F` now passes. Exact initial inputs and the failing log are retained.
Separate negative candidates omit the Autumn idle South palm component or select
its gray eye component; the same material test fails at `(32,45)` and `(37,35)`
respectively. Both controls fail for the intended visible error and are excluded
from the final profile/bundle. Tests compare material output, not the authoring
helper's seed-generation result.

Every final target also passed exact CLI recipe validation over all 104 selected
strips, with no source-hash override. Four validations took 0.256 seconds combined;
the final world test run took 0.54 seconds after compilation. Full collection,
standard combined tests and isolated installation/removal are integration work
and are not claimed by this character batch.

## Measurements and limits

Archive hashing/source export took 1.241 seconds and finished at
`2026-11-06T03:24:22Z`. The passing focused tests and Clippy completed at
`03:27:44Z`, 3m22s later. The complete native/mirror inspection checkpoint was
`03:29:24Z`, 5m02s after export; the complete 316-pair enlarged inspection was
recorded at `2026-11-06T03:32:48.869Z`. Final notes followed. Context reading/helper preparation preceded export. These are measured
agent intervals for repeated-pose content, not human-review throughput or a
forecast for complex forms.

All image work used CPU-native Rust tools with Node orchestration; no Python,
Pillow, model inference or GPU was used. The source, bundle, full review,
component gallery and local evidence total about 22 MiB, plus small temporary
inspection sheets and shared Rust caches. All game imagery/generated packages
remain ignored/local. No accepted fixture or source archive was overwritten.

Final profile SHA-256:
`968ef5b12ca02bd0942e91905186807337519f8a2f6f82416dd5a1df5dadfd62`.
Final preset set SHA-256:
`03ff6ee83e36851e0b720fb4cd26361113a574414d64be414c8ad009ef7fe91e`.
Artwork acceptance was pending at the original authoring checkpoint; the later
assessment below closes that gate. Live gameplay remains unverified.

## Displayed-art acceptance, 2026-11-06

The user replied "looks okay to me" to the consolidated remaining-art review,
including Stillwell's 68 world strips and 316 comparison cases. Record qualified
acceptance of the displayed world artwork; inspection depth is unspecified.
This closes Stillwell's art gate without establishing live-game, menu or input
acceptance. The accepted snapshot is frozen under `local/fixtures/finish-stillwell`.
See the [release-candidate assessment](linux-release-candidate-2026-11-06.md) for
shared review and snapshot/install binding evidence and overall limits. No artwork changed.
