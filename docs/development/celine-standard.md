# Celine's Spring action, sleep and kiss sprites

This slice adds five normal Spring strips: `action_{north,south,east}`,
`sleep_east` and `kiss_east`. Their 26 frame occurrences contain 17 distinct
frames; native West mirroring adds 12 cases. The separate `garden_kiss` artwork
is excluded.

## Source and boundaries

Original paths use
`assets/animations/NPCs/Celine/Sprites/Spring/spr_npc_celine_spring_`.
Action has seven frames per direction, kiss four, and sleep one. All use
80×80 frames, Default atlas and Middle/54 origin. Variable action/kiss durations
and sleep's single-frame defaults remain byte-identical. Raw sidecars are in
`tmp/celine-standard-metadata.json`.

The source is the read-only `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The local corpus is `extracted/celine-standard-study`.

The profile grows from 200 to 205 regions. All previous region objects, seven
color groups and source/target mappings remain unchanged. The existing world
skin colors `FCD9B3`, `F0B988` and `D37A57` retain their light/middle/shadow roles;
reviewed `672115` skin contours retain their separate deep-shade group. Orange
hair, boots, belt, clothing, eyes and actual mouth details stay original.

Hand contours are selected separately from nearby identical hair and belt
colors. Examples include the lowered East action hand `[40,48]`, extended
fingertip `[48,45]`, tiny North hand `[35,46]` and kiss hand `[36,46]`.
Sleep's dark hair end `[35,44]` and belt `[38,45]` remain protected.
These are frame-local coordinates in their respective reviewed poses.

The 266 new seeds select 897 pixels per target: 658 core skin pixels and
239 outline pixels; another 306 matching outline pixels stay protected.

| Strip | Frames | Changed pixels per target |
| --- | ---: | ---: |
| Action East | 7 | 297 |
| Action North | 7 | 37 |
| Action South | 7 | 324 |
| Kiss East | 4 | 189 |
| Sleep East | 1 | 50 |

Profile SHA-256:
`753ec94d9970316324e90a60edd4037864e34b9f07281eabd12c5ca8b4d13857`.
The stylized description now names the added actions; its mapping and the
preset set are unchanged.

## Review and evidence

Every new source frame and all four target versions were inspected, including
moving hands, facial contours and the sleep/kiss poses. All 820 variants validate
exactly. The standalone `generated/celine-standard-trial` matches the inspected
candidate, and all 2,000 previous original/variant PNG and metadata files remain
byte-identical. All 36 duplicate-frame target comparisons pass.

The focused opt-in `tests/celine_standard.rs` passes all four targets, checking
30 literal skin/material landmarks, every new pixel, alpha, metadata, core-shade
coverage, common selection and per-strip counts. Evidence:
`tmp/celine-standard-test-final.log`, `tmp/celine-standard-final-audit.json`,
`tmp/celine-standard-mask-decisions.json` and `tmp/celine-standard-duplicates.log`.

- [Five-choice summary](../../generated/celine-standard-preview/summary.png):
  Action South/East, Sleep and Kiss samples.
- [Complete Vanilla/Blue review](../../generated/celine-standard-preview/blue-review/index.html):
  every source frame and native West mirror on six pages, at most eight cases
  per page.

Fresh crop `[29,23,24,34]` contains every visible pixel in all choices. Exact
binding checks cover 6,201,600 full-review pixels, 261,120 summary pixels,
all 20 sample bindings, metadata and West reversal. Chromium decoded all 38
comparison images and the summary, checked the six pages against the coverage
manifest and resolved all links. Evidence:
`tmp/celine-standard-preview-pixel-check.log` and
`tmp/celine-standard-preview-browser-check.json`.

The user accepted the offline artwork review. Static review does not show game timing,
separate held-item rendering or live behavior. Shared runtime, installation and
full verification are recorded in [the parallel batch notes](spring-world-standard.md).
All extracted artwork, generated previews and local evidence remain ignored.
