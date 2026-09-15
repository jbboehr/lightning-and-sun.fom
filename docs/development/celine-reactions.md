# Celine's shocked reactions and seated reading

Six normal Spring strips add shocked start/loop/end and seated book
start/loop/end. They contain 13 frame occurrences and ten distinct frames,
all South-facing. Garden reactions remain outside this slice.

## Source and masks

Sources are under `assets/animations/NPCs/Celine/Sprites/Spring/`:
`spr_npc_celine_spring_shocked_{start,loop,end}_south.png` and
`spr_npc_celine_specialanimation_spring_book_sit_{start,loop,end}_south.png`.
The three shocked strips each use a single frame. Reading has three start
frames, four loop frames and three end frames; loop durations are
`[3.0, 0.1, 3.0, 0.1]`. All use 80×80 frames, Default atlas and Middle/54 origin.
Raw sidecars are preserved in `tmp/celine-reactions-metadata.json`.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The local extraction is `extracted/celine-reactions-study`.

The profile grows from 205 to 211 regions. All previous region objects, seven
color groups and palette mappings remain unchanged. The existing world shades
`FCD9B3`, `F0B988` and `D37A57` retain light/middle/shadow roles. The separately
grouped `672115` maps to deep skin on reviewed face, wrist and hand components.
Its hair, belt and boot occurrences remain original.

The raised shocked hands and small exposed leg pixels above the boots recolor;
the open mouth's `410808`/`9E2626` interiors remain unchanged. Reading hands
beside the book recolor. Cream `F6E4D7` pages, `C9AF9C` page shading and
`671D2E`/`9C4D49` binding colors are distinct materials and stay original.

The 178 new seeds select 536 pixels per target: 394 core skin pixels and 142
outline pixels. Another 36 matching outline pixels stay protected.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Shocked start / loop / end | 1 / 1 / 1 | 62 / 66 / 62 |
| Book start / loop / end | 3 / 4 / 3 | 105 / 136 / 105 |

Profile SHA-256:
`3677ed5434121a6e5b76a3f7c65a00d12cc8fec750a07ef7060d517908041dfe`.
Only the stylized description changes outside the profile; its mapping and the
preset set are unchanged.

## Review and evidence

Every new source frame and all four target versions were inspected. All 844
variants validate exactly. The standalone `generated/celine-reactions-trial`
matches the inspected candidate, and all 2,050 previous original/variant PNG
and metadata files remain byte-identical. All twelve duplicate-frame target
comparisons pass.

The focused opt-in `tests/celine_reactions.rs` passes four literal target ramps,
34 source-art skin/material landmarks, every new pixel, alpha, metadata, complete
core-shade coverage and common selection. Evidence:
`tmp/celine-reactions-test-final.log`, `tmp/celine-reactions-final-audit.json`,
`tmp/celine-reactions-mask-decisions.json` and `tmp/celine-reactions-duplicates.log`.

- [Five-choice summary](../../generated/celine-reactions-preview/summary.png):
  shocked poses and book-opening/reading samples.
- [Complete Vanilla/Blue review](../../generated/celine-reactions-preview/blue-review/index.html):
  all 13 frames on four small pages, with no additional mirrored directions.

Fresh crop `[27,22,26,35]` includes all visible artwork. Exact binding checks
cover 2,366,000 full-review pixels, 291,200 summary pixels, all 20 sample bindings
and original metadata. Chromium decoded all 13 comparison images and the summary,
verified all four pages against the coverage manifest and resolved their links.
Evidence: `tmp/celine-reactions-preview-pixel-check.log` and
`tmp/celine-reactions-preview-browser-check.json`.

The user accepted the offline artwork review. Static images do not demonstrate live
timing or automatic state transitions. Shared runtime, installation and full
verification are recorded in [the parallel batch notes](spring-world-reactions.md).
Extracted artwork, generated previews and local evidence remain ignored.
