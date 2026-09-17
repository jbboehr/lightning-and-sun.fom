# Ryis's remaining Summer special animations

This slice adds eleven strips and 37 source frames, plus twelve native West
mirrors. The profile now has 178 sources and 712 variants. All 167 earlier
region objects, source colors and target mappings remain unchanged.

The read-only archive is `tmp/momi-lab/assets.bak.zip`; extraction is
`extracted/ryis-summer-expansion-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Summer/` use the prefix
`spr_npc_ryis_specialanimation_summer_`. Exact sidecars are recorded in
`tmp/ryis-summer-expansion-metadata.json` and match the independent root inventory.
All retain 80×80 frames and Default atlas. Reading uses Middle/54 origin;
the other eight strips preserve numeric `[40.0,54.0]`. Reading and writing are
native complex sequences; the other cycles are linear.

| Source suffix | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| `hammer_east` | 7 | `0.1` | 59, 50, 50, 52, 53, 54, 54 |
| `saw_east` | 4 | `[0.325,0.125,0.3,0.125]` | 53, 55, 58, 55 |
| `siteyesclosed_east` | 1 | Engine default | 61 |
| `siteyesclosed_south` | 1 | Engine default | 69 |
| `wipebrow_south` | 6 | `[0.15,0.15,0.45,0.1,0.075,1.0]` | 49, 59, 64, 72, 70, 69 |
| `read_sit_start_south` | 3 | `0.1` | 59, 45, 49 |
| `read_sit_loop_south` | 4 | `[3.0,0.1,3.0,0.1]` | 48, 57, 48, 57 |
| `read_sit_end_south` | 3 | `0.1` | 53, 41, 59 |
| `write_start_south` | 2 | `0.15` | 66, 64 |
| `write_loop_south` | 4 | `0.15` | 64, 65, 64, 65 |
| `write_end_south` | 2 | `0.15` | 63, 66 |

The 232 seeds select 2,139 skin pixels per target with the unchanged ramp
`B06C57`, `854D3C`, `63342A`, `491F1B`. Previously accepted seed coordinates
were reused only where they fall inside a reviewed Summer skin component;
all new masks were checked against the actual Summer sprites. All 37 frames
were inspected in Vanilla and all four targets. Fingers around the tools,
book, clipboard and brow glove are included, as are exposed lower legs.
Dark gloves (`353A50`), hammer handle (`936244`), clipboard (`7D4F3D`), blue
book cover, cream pages, mouth details, hair, clothing, footwear and impact
effects remain original. Evidence: `tmp/ryis-summer-expansion-components.json`
and `tmp/ryis-summer-expansion-art-{cycle}-{direction}-{page}.png` (sequence
names include their phase).

The [five-choice summary](../../generated/ryis-summer-expansion-preview/summary.png)
shows hammering, reading and writing. The [complete Vanilla/Blue review](../../generated/ryis-summer-expansion-preview/blue-review/index.html)
covers all 49 cases across eight pages, with at most eight cases each. Crop
`[27,23,52,43]` includes every visible pixel, including hammer effects through
`[76,63]`. Exact checks cover 21,912,800 full-review pixels at 10×, 536,640
summary pixels at 4×, all fifteen sample bindings, metadata/durations, origins
and West reversal. Chromium decoded every image and checked pages and links.
Evidence: `tmp/ryis-summer-expansion-preview-pixel-check.log` and
`tmp/ryis-summer-expansion-preview-browser-check.json`.

The opt-in `tests/ryis_summer_expansion.rs` passed all new pixels in all four
targets, per-frame counts, alpha, sidecars and twenty-five literal material
boundaries (`tmp/ryis-summer-expansion-corpus-test.log`). The final
`generated/ryis-summer-expansion-trial` preserves all 1,670 prior Ryis PNG and
metadata files byte-for-byte; all 712 variants equal the inspected author
outputs (`tmp/ryis-summer-expansion-comparison.log`). Profile SHA-256:
`63b0933b58c3523f28579a63a421d5176104ff309ca6a666419eb2534c185d0e`.

See [the combined integration report](world-summer-expansion.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish game timing, natural schedules or live gameplay. User artwork
acceptance was received on 2026-09-16.
