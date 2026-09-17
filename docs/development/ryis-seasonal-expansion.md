# Ryis's remaining Autumn special animations

This slice adds eleven strips: 37 source frames and twelve native West mirrors.
The profile now has 211 sources and 844 variants. All 200 prior regions,
other profile fields, source colors and target mappings remain unchanged.

The read-only `tmp/momi-lab/assets.bak.zip` was exported into
`extracted/ryis-seasonal-expansion-study`. Sources under
`assets/animations/NPCs/Ryis/Sprites/Autumn/` use the prefix
`spr_npc_ryis_specialanimation_autumn_`. Exact sidecars in
`tmp/ryis-seasonal-expansion-metadata.json` match the independent root inventory.
All use 80×80 frames and Default atlas. Reading preserves Middle/54 origin;
the other eight strips preserve numeric `[40.0,54.0]`. Reading and writing are
native complex sequences; the other cycles are linear.

| Source suffix | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| `hammer_east` | 7 | `0.1` | 52, 44, 44, 51, 51, 51, 51 |
| `saw_east` | 4 | `[0.325,0.125,0.3,0.125]` | 43, 47, 47, 47 |
| `siteyesclosed_east` | 1 | Engine default | 54 |
| `siteyesclosed_south` | 1 | Engine default | 60 |
| `wipebrow_south` | 6 | `[0.15,0.15,0.45,0.1,0.075,1.0]` | 32, 44, 50, 59, 53, 52 |
| `read_sit_start_south` | 3 | `0.1` | 51, 44, 50 |
| `read_sit_loop_south` | 4 | `[3.0,0.1,3.0,0.1]` | 44, 56, 44, 56 |
| `read_sit_end_south` | 3 | `0.1` | 54, 40, 51 |
| `write_start_south` | 2 | `0.15` | 59, 55 |
| `write_loop_south` | 4 | `0.15` | 54, 54, 54, 54 |
| `write_end_south` | 2 | `0.15` | 55, 59 |

The 104 component seeds select 1,866 skin pixels per target using the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in
Vanilla and all four targets. Detached fingers around gloves, tools, books and
the clipboard are included. Yellow gloves, light cuffs, blue coat and scarf,
covered legs and brown boots remain original. Hammer handle `936244`,
clipboard `7D4F3D`, blue book covers, cream pages, pencil, mouth details, hair
and tool effects are preserved. Evidence:
`tmp/ryis-seasonal-expansion-components.json` and
`tmp/ryis-seasonal-expansion-art-{cycle}-{direction}-{page}.png`; sequence
cycle names include their phase.

The [five-choice summary](../../generated/ryis-seasonal-expansion-preview/summary.png)
shows hammering, reading and writing. The [complete Vanilla/Blue review](../../generated/ryis-seasonal-expansion-preview/blue-review/index.html)
covers all 49 cases across eight pages, with at most eight cases per page.
Crop `[27,23,42,34]` includes every visible pixel, the hammer swing and saw
blade. Exact checks verified 13,994,400 full-review pixels, 342,720 summary
pixels, all fifteen sample bindings, metadata, origins and West reversal.
Chromium checked every page, image and link. Evidence:
`tmp/ryis-seasonal-expansion-preview-check.log` and
`tmp/ryis-seasonal-expansion-preview-browser-check.json`.

The opt-in `tests/ryis_seasonal_expansion.rs` passed all new pixels in all four
targets, per-frame counts, sidecars and 29 literal material landmarks
(`tmp/ryis-seasonal-expansion-corpus-test.log`). The final
`generated/ryis-seasonal-expansion-trial` preserves all 2,000 prior PNG/metadata
files byte-for-byte; all 844 variants match the author outputs
(`tmp/ryis-seasonal-expansion-preservation.log`). Profile SHA-256:
`82a29c24d7685f76232e617ad88bf01a46755517179d9eeb15f415e0c2bc912c`.

See [the combined integration report](world-seasonal-expansion.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user accepted
this offline artwork on 2026-09-16.
