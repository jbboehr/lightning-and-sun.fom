# Ryis's Summer blink, sit, eat and drink

This slice adds eleven strips and 31 source frames, plus twelve native West
mirrors. The profile now has 162 sources and 648 variants. All 151 earlier
region objects, source colors and target mappings remain unchanged.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, exported to
`extracted/ryis-outfit-actions-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Summer/` use the prefix
`spr_npc_ryis_summer_`. Exact sidecars are in
`tmp/ryis-outfit-actions-metadata.json`. All retain 80×80 frames, Default atlas
and Middle/54 origin. Native cycles are linear; sit, eat and drink are seated.
Eat and drink retain native `last_frame_hold = [240,360]`.

| Strip | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| Blink East | 3 | `[0.075,0.125,0.075]` | 64, 68, 64 |
| Blink South | 3 | `[0.075,0.125,0.075]` | 73, 77, 73 |
| Sit East | 1 | Engine default | 53 |
| Sit North | 1 | Engine default | 30 |
| Sit South | 1 | Engine default | 61 |
| Eat East | 5 | `[0.125,0.15,0.175,0.125,0.6]` | 47, 51, 40, 51, 51 |
| Eat North | 3 | `1.0` | 31, 28, 31 |
| Eat South | 5 | `[0.125,0.15,0.175,0.125,0.6]` | 61, 57, 58, 65, 61 |
| Drink East | 3 | `1.0` | 51, 57, 50 |
| Drink North | 3 | `1.0` | 31, 28, 31 |
| Drink South | 3 | `1.0` | 60, 65, 60 |

The 197 seeds select 1,628 skin pixels per target using the unchanged ramp
`B06C57`, `854D3C`, `63342A`, `491F1B`. All 31 frames were inspected in Vanilla
and all four targets. Detached fingers, raised hands and exposed lower legs
are included. Dark gloves/cups (`353A50`), pink footwear (`FFA799`), short rear
hair (`5E423B`) and mouth details (`410808`, `C83E37`) remain original, along
with the other clothing, hair and eyes. Evidence:
`tmp/ryis-outfit-actions-components.json` and
`tmp/ryis-outfit-actions-art-{cycle}-{direction}.png`.

The [five-choice summary](../../generated/ryis-outfit-actions-preview/summary.png)
shows one sample per action. The [complete Vanilla/Blue review](../../generated/ryis-outfit-actions-preview/blue-review/index.html)
covers all 43 cases across eight pages, with at most eight cases each. Crop
`[29,24,22,33]` includes every visible pixel. Exact checks cover 6,243,600
full-review pixels at 10×, 522,720 summary pixels at 6×, all twenty sample
bindings, metadata/durations and West reversal. Chromium decoded every image
and checked the pages and links. Evidence:
`tmp/ryis-outfit-actions-preview-pixel-check.log` and
`tmp/ryis-outfit-actions-preview-browser-check.json`.

The opt-in `tests/ryis_outfit_actions.rs` passed all new pixels in all four
targets, per-frame counts, alpha, sidecars and twenty literal skin/material
boundaries (`tmp/ryis-outfit-actions-corpus-test.log`). The final
`generated/ryis-outfit-actions-trial` preserves all 1,510 prior Ryis PNG/metadata
files byte-for-byte; all 648 variants equal the inspected author outputs
(`tmp/ryis-outfit-actions-comparison.log`). Profile SHA-256:
`958c8c64464e4257c72ef3f7851a95455314139e895832b9fd7960819b192753`.

See [the combined integration report](world-outfit-actions.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish game timing, natural schedules or live gameplay. The user accepted
the offline artwork review.
