# Ryis's Autumn action, sleep and kiss

This slice adds five strips: 26 source frames and twelve native West mirrors.
The profile now has 200 sources and 800 variants. All 195 prior regions,
other profile fields, source colors and target mappings are unchanged.

The read-only `tmp/momi-lab/assets.bak.zip` was exported into
`extracted/ryis-seasonal-special-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Autumn/` use the prefix
`spr_npc_ryis_autumn_`: action North/South/East, sleep East and kiss East.
All retain 80×80 frames, Default atlas and Middle/54 origin. Exact sidecars
in `tmp/ryis-seasonal-special-metadata.json` match the independent root
inventory. Native cycles are linear; action retains
`last_frame_hold = [240,360]` and `on_pause_speaking = "idle"`.

| Strip | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| Action East | 7 | `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` | 46, 44, 46, 44, 46, 46, 47 |
| Action North | 7 | Same | 23, 24, 24, 24, 24, 24, 26 |
| Action South | 7 | Same | 51, 51, 51, 51, 51, 51, 52 |
| Sleep East | 1 | Engine default | 48 |
| Kiss East | 4 | `[0.15,0.15,0.8,0.15]` | 46, 49, 54, 53 |

The 69 component seeds select 1,096 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in
Vanilla and all four targets. Exposed fingertips, including the extended
action hand and raised sleep hand, are included. Yellow gloves (`F4CD86`,
`E7A063`), light cuffs, blue coat and scarf, covered legs and brown boots remain
original. The `5E423B` rear-head patch remains short hair. Black closed-eye
lines and the kissing-mouth outline are preserved. Evidence:
`tmp/ryis-seasonal-special-components.json` and
`tmp/ryis-seasonal-special-art-{cycle}-{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-seasonal-special-preview/summary.png)
shows one sample per cycle. The [complete Vanilla/Blue review](../../generated/ryis-seasonal-special-preview/blue-review/index.html)
covers all 38 cases across six pages, with at most eight cases per page. Crop
`[29,23,24,34]` includes every visible pixel and the extended fingers. Exact
checks cover 6,201,600 full-review pixels, 440,640 summary pixels, all fifteen
sample bindings, source metadata and West reversal. Chromium checked every
page, image and link. Evidence: `tmp/ryis-seasonal-special-preview-check.log`
and `tmp/ryis-seasonal-special-preview-browser-check.json`.

The opt-in `tests/ryis_seasonal_special.rs` passed all new pixels in all four
targets, per-frame counts, sidecars and nineteen literal skin/material landmarks
(`tmp/ryis-seasonal-special-corpus-test.log`). The final
`generated/ryis-seasonal-special-trial` preserves all 1,950 prior PNG/metadata
files byte-for-byte; all 800 variants match the author outputs
(`tmp/ryis-seasonal-special-preservation.log`). Profile SHA-256:
`6cde60c2ae0bc8ea8abe6b79527df5851b16d84f092aa6eae0793b25909ea428`.

See [the combined integration report](world-seasonal-special.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user accepted
this offline artwork on 2026-09-16.
