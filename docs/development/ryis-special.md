# Ryis's remaining Spring special animations

This slice adds eight strips and 27 source frames, plus 12 native West mirrors.
The profile now has 145 sources and 580 variants. All 137 previous region
objects, source colors and target mappings remain unchanged.

The read-only archive is `tmp/momi-lab/assets.bak.zip`; extraction is
`extracted/ryis-special-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Spring/` use the prefix
`spr_npc_ryis_specialanimation_spring_`. Exact sidecars and frame counts are
recorded in `tmp/ryis-special-metadata.json`. These strips retain 80×80 frames,
Default atlas and numeric offset `[40.0,54.0]`; the horizontal origin is stored
as a number rather than the earlier `Middle` string.

| Source suffix | Frames | Native duration | Skin pixels per target |
| --- | ---: | --- | ---: |
| `hammer_east` | 7 | `0.1` | 340 |
| `saw_east` | 4 | `[0.325,0.125,0.3,0.125]` | 195 |
| `siteyesclosed_east` | 1 | Engine default | 56 |
| `siteyesclosed_south` | 1 | Engine default | 64 |
| `wipebrow_south` | 6 | `[0.15,0.15,0.45,0.1,0.075,1.0]` | 320 |
| `write_start_south` | 2 | `0.15` | 113 |
| `write_loop_south` | 4 | `0.15` | 216 |
| `write_end_south` | 2 | `0.15` | 113 |

The 114 seeds select 1,417 skin pixels per target using the unchanged ramp
`B06C57`, `854D3C`, `63342A`, `491F1B`. All 27 frames were inspected in Vanilla
and all four targets. Tools, gloves, clothing, hair, clipboard, pencil and
impact effects remain original. In particular, the brown hammer handle and
clipboard use `936244` and `7D4F3D`, distinct from skin. Detached fingers at the
hammer, saw, brow glove and clipboard are included. Component inventory and
inspected sheets: `tmp/ryis-special-components.json` and
`tmp/ryis-special-art-{cycle}-{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-special-preview/summary.png)
is about 52 KB. The [complete Vanilla/Blue review](../../generated/ryis-special-preview/blue-review/index.html)
covers all 39 cases across six pages, at most eight cases per page; about
1.1 MB in total. Crop `[27,23,52,43]` retains the hammer effects through source
coordinates `[76,63]`. Exact checks cover 17,440,800 full-review pixels at 10×,
536,640 summary pixels at 4×, all 15 sample bindings, native metadata, clipping
and West reversal. Chromium decoded every image and checked every page and
link. Evidence: `tmp/ryis-special-preview-pixel-check.log` and
`tmp/ryis-special-preview-browser-check.json`.

The opt-in `tests/ryis_special.rs` passes every new pixel across all four
targets, per-frame counts, alpha, sidecars and 12 literal material boundaries
(`tmp/ryis-special-focused.log`). The final `generated/ryis-special-trial`
preserves all 1,370 prior Ryis PNG/metadata files byte-for-byte; all 580 variants
equal the inspected author outputs (`tmp/ryis-special-comparison.log`).
Profile SHA-256:
`d78711fe0cee179e4d25450e3504fffc9a58e57965c5d0d2f8a098f82968859d`.

See [the combined integration report](spring-world-special.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural schedules or live gameplay. The user accepted the offline artwork review.
