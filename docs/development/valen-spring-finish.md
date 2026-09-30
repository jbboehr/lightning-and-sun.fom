# Valen's Spring healing and charm sprites

The user approved the offline artwork on 2026-09-29.

Six strips finish all 40 PNG strips in Valen's Spring folder. Healing has
1/4/1 East-facing start/loop/end frames; charm has 3/4/2 South-facing frames.
The profile grows from 126 to 132 sources and 528 variants. All previous
region objects, pins, 11 source roles, six groups and target mappings remain
exact. The preset set and canonical Debug Blue recipe are unchanged.

Sources are named `spr_npc_valen_specialanimation_spring_heal_{start,loop,end}_east.png`
and `spr_npc_valen_specialanimation_spring_charm_{start,loop,end}_south.png`
under `assets/animations/NPCs/Valen/Sprites/Spring/`. All six fresh raw
sidecars match independent archive reads: 80×80 frames, Default atlas and
numeric 40/54 origin. The read-only archive `tmp/fields-of-mistria/assets.zip`
retains SHA-256 `8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Folder inventory and raw metadata are recorded in
`tmp/valen-spring-finish-author-{folder,metadata}.json`.

The 420 new component seeds select 771 skin pixels per target. Faces,
necks, fingers, wrists and exposed ankles change. Gold goggles, the amber
and blue healing prop, pale cloth, eyes, hair and clothing remain original.
The healing prop's `#FFC962`, `#CF8039`, `#934D00` and `#7D391E` differ from
skin; its blue portion uses `#A6B7E5` and `#596CA1`. The cloth uses `#F5F5F5`,
`#C3D1DD` and `#547BB0`, separate from both skin and the gold goggles.

Charm-end frame 1 has one `#B65932` hand-shadow pixel at zero-based local
coordinate `(33,48)`, below the held goggles. It uses existing source role 3;
the adjacent gold stays original. This small region was checked explicitly
after the first review. No new role or material-edge compromise is needed.
Every actual frame was inspected in Vanilla and all four target palettes.

| Strip | Skin pixels per frame |
| --- | --- |
| Healing start / end | 63 / 63 |
| Healing loop | 47, 43, 46, 47 |
| Charm start | 48, 50, 48 |
| Charm loop | 47, 47, 47, 48 |
| Charm end | 60, 67 |

- [Five-choice summary](../../generated/valen-spring-finish-preview/summary.png)
  samples healing loop frame 2, charm start frame 2 and charm end frame 1.
- [Complete Vanilla/Blue review](../../generated/valen-spring-finish-preview/blue-review/index.html)
  contains all 15 source frames and six native West healing mirrors.
- [Shared batch review](../../generated/balor-summer-valen-heal-eiland-magnify-preview/index.html)
  links to this batch's Balor, Valen and Eiland reviews.

The gallery contains nine sheets across 11 HTML pages, totaling 446,812
bytes; its largest file is 58,237 bytes. Checks compare 4,032,288 exact
preview pixels, 42 full-case bindings plus enlarged details and 15 summary
bindings. The full crop `[25,20,32,39]` contains every opaque pixel, including
mirrors. Raw metadata, frame counts and palette/source bindings match.
Chromium loaded all local links and images without horizontal overflow.

The new material test and all five retained corpus suites pass, as does
targeted Clippy. The new test checks 66 literal material landmarks, all
pixels in four targets, per-frame counts, alpha and metadata. Four effective
omission controls remove the rare hand shadow, free hand, gripping hand and
finger outline. A spill control maps gold and proves that two protected
goggle pixels change. Retained tests change only corpus paths and total count.

All 528 variants pass strict validation. All 1,260 earlier original/variant
PNG and metadata files remain byte-identical; all 1,056 final variant files
match the inspected candidates. Evidence is under
`tmp/valen-spring-finish-author-` with `focused.log`, `retained.log`,
`preservation.log`, `preview.log`, `preview-browser-check.json` and
`frozen-inputs.sha256`. The complete corpus is
`extracted/valen-spring-finish-study`; artwork stays ignored.

See [shared integration](balor-summer-valen-heal-eiland-magnify.md) for native
phase/mirror probes and isolated installation. Static previews do not
establish live scheduling, timing or transitions. This slice changes no
runtime code; other outfits remain for later batches.
