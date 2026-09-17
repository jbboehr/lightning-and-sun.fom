# Ryis's Summer action, sleep and kiss

This slice adds five strips and 26 source frames, plus twelve native West
mirrors. The profile now has 167 sources and 668 variants. All 162 earlier
region objects, source colors and target mappings remain unchanged.

The read-only archive is `tmp/momi-lab/assets.bak.zip`; extraction is
`extracted/ryis-outfit-special-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Summer/` use the prefix
`spr_npc_ryis_summer_`. Exact sidecars are in
`tmp/ryis-outfit-special-metadata.json`, independently matched to the root raw
inventory. All retain 80×80 frames, Default atlas and Middle/54 origin.
Native cycles are linear. General action retains `last_frame_hold = [240,360]`
and `on_pause_speaking = "idle"`; sleep and kiss use East and native West mirrors.

| Strip | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| Action East | 7 | `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` | 53, 53, 54, 53, 54, 53, 60 |
| Action North | 7 | Same | 38, 38, 39, 38, 39, 41, 42 |
| Action South | 7 | Same | 62, 65, 63, 65, 63, 62, 69 |
| Sleep East | 1 | Engine default | 54 |
| Kiss East | 4 | `[0.15,0.15,0.8,0.15]` | 53, 58, 62, 62 |

The 179 seeds select 1,393 skin pixels per target using the unchanged ramp
`B06C57`, `854D3C`, `63342A`, `491F1B`. All 26 frames were inspected in Vanilla
and all four targets. Extended fingers, the raised sleep hand and exposed lower
legs are included. Dark gloves (`353A50`), footwear (`D96A70`, `FFA799`), short
rear hair (`5E423B`), closed eyes and the black kissing-mouth outline remain
original, along with other hair, eyes and clothing. Evidence:
`tmp/ryis-outfit-special-components.json` and
`tmp/ryis-outfit-special-art-{cycle}-{direction}-{page}.png`.

The [five-choice summary](../../generated/ryis-outfit-special-preview/summary.png)
shows one sample per cycle. The [complete Vanilla/Blue review](../../generated/ryis-outfit-special-preview/blue-review/index.html)
covers all 38 cases across six pages, with at most eight cases each. Crop
`[29,23,24,34]` includes every visible pixel, including the extended fingers.
Exact checks cover 6,201,600 full-review pixels at 10×, 440,640 summary pixels at
6×, all fifteen sample bindings, metadata/durations and West reversal. Chromium
decoded every image and checked the pages and links. Evidence:
`tmp/ryis-outfit-special-preview-pixel-check.log` and
`tmp/ryis-outfit-special-preview-browser-check.json`.

The opt-in `tests/ryis_outfit_special.rs` passed all new pixels in all four
targets, per-frame counts, alpha, sidecars and nineteen literal skin/material
boundaries (`tmp/ryis-outfit-special-corpus-test.log`). The final
`generated/ryis-outfit-special-trial` preserves all 1,620 prior Ryis PNG/metadata
files byte-for-byte; all 668 variants equal the inspected author outputs
(`tmp/ryis-outfit-special-comparison.log`). Profile SHA-256:
`c16f7fafe87eccee1f95cad653263328b96dc1650b0ba165477fbac7bc3e4412`.

See [the combined integration report](world-outfit-special.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish game timing, natural schedules or live gameplay. User artwork
acceptance was received on 2026-09-16.
