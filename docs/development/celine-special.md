# Celine's remaining normal Spring specials

Eight strips add standing-book start/loop/end, herb preparation, sweeping
start/loop/end and watering East. They contain 37 frame occurrences and
31 distinct frames; native West watering adds four review cases. Garden assets
remain excluded.

## Source and masks

All sources are under `assets/animations/NPCs/Celine/Sprites/Spring/`, prefixed
`spr_npc_celine_specialanimation_spring_`. Book phases use three/four/three frames;
herb uses six; sweep uses one/fifteen/one; water uses four. All have 80×80 frames,
Default atlas and Middle/54 origin. Raw sidecars, including variable durations,
are in `tmp/celine-special-metadata.json`.

The source is read-only `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local extraction: `extracted/celine-special-study`.

The profile grows from 211 to 219 regions. All previous region objects, seven
color groups and source/target mappings remain unchanged. Existing world shades
`FCD9B3`, `F0B988` and `D37A57` retain light/middle/shadow roles; separately grouped
`672115` maps to deep skin on reviewed face and hand components.

The 479 new seeds select 1,671 pixels per target: 1,314 core skin pixels and
357 outline pixels. Another 147 matching outline pixels remain protected.
Moving hand contours are selected separately from belts, hair and boots.
For example, watering frame two `[38,41]` is a hair edge and remains original;
sweeping frame one `[39,48]` is a hand shadow and recolors. Coordinates are
frame-local and frame numbers here start at one.

Book pages/binding, the blue mortar and pestle, green herb flecks, wooden broom,
yellow bristles, dust and golden watering can remain original. The stylized
recipe description now includes the completed normal Spring coverage; its map
and the preset set are unchanged.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Book start / loop / end | 3 / 4 / 3 | 110 / 136 / 110 |
| Herb | 6 | 256 |
| Sweep start / loop / end | 1 / 15 / 1 | 52 / 784 / 62 |
| Water East | 4 | 161 |

Profile SHA-256:
`798f445264b12cd6b6cc50038fed6d9f55d3ec4b3ab0f3db6e87afb5f1a5e922`.

## Review and evidence

Every new source frame and all four target versions were inspected with the
complete tool/dust artwork visible. All 876 variants validate exactly. The
standalone `generated/celine-special-trial` matches the inspected candidate, and
all 2,110 previous original/variant PNG and metadata files remain byte-identical.
All 24 duplicate-frame target comparisons pass.

The focused opt-in `tests/celine_special.rs` passes four literal target ramps,
35 source-art skin/material landmarks, every new pixel, alpha, metadata, complete
core-shade coverage and common selection. Evidence:
`tmp/celine-special-test-final.log`, `tmp/celine-special-final-audit.json`,
`tmp/celine-special-mask-decisions.json` and `tmp/celine-special-duplicates.log`.

- [Five-choice summary](../../generated/celine-special-preview/summary.png):
  standing book, herbs, sweeping and watering samples.
- [Complete Vanilla/Blue review](../../generated/celine-special-preview/blue-review/index.html):
  all 37 source frames and four West watering views on seven pages, at most
  eight cases per page.

Fresh crop `[25,24,39,39]` includes every visible pixel, including the low broom
bristles, drifting dust and long watering spout. Exact binding checks cover
12,472,200 full-review pixels, 273,780 summary pixels, all 20 sample bindings,
original metadata and West reversal. Chromium decoded all 41 comparison images
and the summary, checked the seven pages against the coverage manifest and
resolved their links. Evidence: `tmp/celine-special-preview-pixel-check.log`
and `tmp/celine-special-preview-browser-check.json`.

The user accepted the offline artwork review. Static images do not demonstrate live
timing or automatic state transitions. Shared runtime, installation and full
verification are recorded in [the parallel batch notes](spring-world-special.md).
Extracted artwork, generated previews and local evidence remain ignored.
