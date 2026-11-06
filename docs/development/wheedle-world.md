# Wheedle: complete four-season world candidate

2026-11-05. **Authored and validated; displayed artwork accepted, inspection depth unspecified.**
This approved [finish-plan](../plans/finish.md) batch adds all 56 seasonal world
strips to `wheedle-world-trial`. The prior 32 portrait regions, ten source-color
roles, singleton groups and preset mappings remain exactly as before. Shared
registry selection, collection counts and full installation checks belong to the
integrator. No runtime code changes are included here.

## Source and material decisions

The preserved source archive is SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The export pins all 88 PNGs and metadata files in
`local/authoring/wheedle-world/export.json`; each profile region pins the PNG and
its dimensions. Native NPC definitions are retained beside the export record.

Each seasonal outfit has action East/North/South (seven frames each), blink
East/South (three each), idle East/North/South and seated East/North/South (one
each), plus walk East/North/South (four each). These are 180 native frame
occurrences, 116 exact decoded unique frames and 64 duplicate occurrences.
There were no previously approved world masks to reuse. All 56 strips contain
visible skin, including four ear pixels in the Winter North frames; no empty
masks or omitted no-change paths are required.

World skin adds four singleton colors using the existing target roles:

| Source | Target role | Selected material |
| --- | --- | --- |
| `#F6C897` | Light | Lit face, ears, hands, Summer chest and ankles |
| `#DCA77C` | Middle | Face, hands and exposed Summer skin |
| `#C18561` | Shadow | Face and hand shading, exposed Summer skin |
| `#7F523E` | Deep | Bare-finger contours and two Summer ankle shadows |

The first ten colors and every old mapping are retained unchanged. Main skin
colors also appear on Summer's exposed ankles just below the trouser cuffs;
those follow the skin palette. The Summer portrait independently establishes the
open chest, and the world source retains that opening. Winter hands are golden
gloves and remain original.

The deep world shade is shared with brown hair, beard, and a Summer seated-East
trouser shadow. Those materials remain original, including the back-facing hair
at local row 40 and the Summer seated-East trouser pixel `(40,45)`. The hand
shadows are selected separately, including the low central palm at `(40,47)` in
East action frames 1 and 6 and the inward hand at `(38,46)` in South action
frames 3 and 5. Summer walk East frames 2 and 4 include exposed ankle shadows at
`(42,51)` and `(44,51)`. Face hair, eye whites/gray/black details, mouth lines,
outfit patterns, bow ties, cream trousers, shoes and all other source materials
retain their pixels.

The profile uses 2,697 new component seeds and changes 4,314 source pixels per
variant: Autumn 1,140; Spring 1,070; Summer 1,380; Winter 724. Another 2,731
occurrences of configured world colors remain protected. Selection is stored as
source-bound connected components, with no runtime coordinate heuristic.

## Review artifacts and verification

- [Five-choice SAMPLE summary](../../generated/wheedle-world-finish-review/summary.png).
- [Complete offline review](../../generated/wheedle-world-finish-review/index.html):
  all 180 native frames and 64 West mirrors, split into four outfits and 92 small
  case pages. Vanilla is left and Debug Blue right, with full visible sprites and
  enlarged face/hand details. The full views also include the Summer ankles.
- [Component authoring gallery](../../generated/wheedle-world-mask-review/index.html):
  all 180 current candidate masks reconstruct without conflicts or unresolved
  groups. This is authoring-tool evidence, not user acceptance.
- Complete sources: `extracted/wheedle-world-finish-study/`, also available through
  `extracted/test-corpus/wheedle`.
- Current generated variants and MOMI package:
  `generated/wheedle-world-finish-build/`.

The gallery helper reconciles every native frame and mirrored case against
metadata, asserts that full views hide no opaque source pixel, and verifies
29,079,552 enlarged preview pixels against the current bundle. It requires all
488 full Vanilla/Blue bindings, 488 detail bindings and 20 five-choice sample
cells. The author inspected the complete native/mirrored Source/Blue corpus and
rechecked enlarged hand and Summer ankle cases after correcting the selection.
This is author inspection, not user acceptance.

CPU Chromium loaded all 97 desktop pages and five mobile index pages: all local
links and images exist/load, with no horizontal overflow. The 102 page loads took
50.053 seconds; GPU use was disabled. Other browser engines and natural in-game
rendering/input remain unverified.

Fresh focused checks passed:

```sh
nix-shell --pure --run 'cargo clippy --release --locked --test wheedle_world -- -D warnings'
nix-shell --pure --run 'cargo test --release --locked --test wheedle_world --test wheedle_portraits -- --include-ignored'
```

Two regular world tests preserve all prior recipes and package all 56 synthetic
strips while retaining native frame counts, durations, offsets and atlas. The
source-backed test checks every world RGBA pixel in all four presets against
source-material expectations and fixed per-frame counts. Its hand-shadow
expectations are literal anatomical coordinates independent of the authoring
helper's spatial selection. It preserves all metadata bytes and rebuilds the
prior portraits to compare all 256 prior PNG/metadata outputs byte-for-byte.
The retained portrait skin/beard/glove regression also passes unchanged.

The material regression failed before correction on Autumn East action frame 1,
local `(40,47)`, where the initial spatial mask omitted a dark palm pixel. The
corrected candidate passes, including inward hand contours and Summer ankle
shadows. A separate negative candidate deliberately selects an Autumn hair
component; the same test fails at idle South `(37,30)`, proving that shared-color
material leakage is detected. Both failure logs and exact control inputs are
retained under `local/authoring/wheedle-world/`. No negative candidate is selected
by the registry or used for the final gallery.

Each final target also passed CLI exact-recipe validation over all 88 selected
strips with no source-hash override. The four validation runs took 0.218 seconds
combined. The focused source world test took 0.58 seconds after compilation.
Full collection tests and installation/removal are integration checks and are
not claimed by this character batch.

## Measurements and limits

Export and archive hashing took 0.600 seconds. The export-complete timestamp was
`2026-11-06T01:50:42Z`; the first passing corrected material test completed at
`01:57:09Z`, an observed 6m27s interval covering source inspection, component
selection, candidate inspection, the palm regression and correction. Later
focused checks and complete browser checks are recorded separately. This is
agent elapsed time for a small repeated-pose character, not human-review time or
a reliable forecast for more complex characters.

All work used CPU image operations, native Rust dependencies and standard Python
orchestration; no Pillow, model inference or local GPU was used. The final source,
package, review, component gallery and retained authoring evidence total about
21 MiB, excluding small temporary inspection sheets and shared build caches.
Historical portrait sources and accepted outputs were not replaced.

The final profile SHA-256 is
`565d82b8c21851b89ac91d8c2a4a2a3298124deb2c826c2a43271a5620d2fc8b`;
the preset set SHA-256 is
`a8a291f45a5fdca64179df19833d992a1b1a6744b881c9f2ce13f34f4dde16e8`.
All game imagery and generated outputs remain ignored/local. The parent batch later passed combined integration and full install/remove;
displayed artwork is accepted, while live gameplay remains unverified.

After the combined second-batch review, the user reported Reina mouth issues
and no other issues. This records acceptance of the displayed artwork for this
character, without inferring full-gallery inspection depth or live-game acceptance.
