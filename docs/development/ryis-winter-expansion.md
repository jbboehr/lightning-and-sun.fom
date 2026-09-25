# Ryis's remaining Winter special animations

This slice adds eleven strips: 37 source frames and twelve native West mirrors.
The profile now has 244 sources and 976 variants, completing the 33 Winter
world strips. All 233 prior regions, other profile fields, source colors and
target mappings remain unchanged.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-winter-expansion-study`. Every prior Ryis PNG hash and all
466 original PNG/metadata files match the retained preceding combined build.
Sources under `assets/animations/NPCs/Ryis/Sprites/Winter/` use the prefix
`spr_npc_ryis_specialanimation_winter_`. Exact sidecars are in
`tmp/ryis-winter-expansion-metadata.json`. All use 80×80 frames and Default
atlas. Reading preserves Middle/54 origin; the other eight strips preserve
numeric `[40.0,54.0]`. Reading and writing are native complex sequences;
the other cycles are linear.

| Source suffix | Frames | Duration | Skin pixels per frame, per target |
| --- | ---: | --- | --- |
| `hammer_east` | 7 | `0.1` | 51, 50, 50, 52, 52, 52, 52 |
| `saw_east` | 4 | `[0.325,0.125,0.3,0.125]` | 42, 45, 45, 45 |
| `siteyesclosed_east` | 1 | Engine default | 52 |
| `siteyesclosed_south` | 1 | Engine default | 58 |
| `wipebrow_south` | 6 | `[0.15,0.15,0.45,0.1,0.075,1.0]` | 31, 44, 50, 57, 51, 50 |
| `read_sit_start_south` | 3 | `0.1` | 49, 44, 50 |
| `read_sit_loop_south` | 4 | `[3.0,0.1,3.0,0.1]` | 42, 54, 42, 54 |
| `read_sit_end_south` | 3 | `0.1` | 54, 40, 49 |
| `write_start_south` | 2 | `0.15` | 56, 54 |
| `write_loop_south` | 4 | `0.15` | 50, 50, 51, 51 |
| `write_end_south` | 2 | `0.15` | 54, 56 |

The 115 component seeds select 1,829 skin pixels per target using the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in
Vanilla and all four targets. Detached fingers around gloves, tools, books and
the clipboard are included. Dark gloves, orange coat and cuffs, blue scarf,
covered legs and brown boots remain original. Hammer handle `936244`,
clipboard `7D4F3D`, blue book covers, cream pages, pencil, mouth details, hair
and tool effects are preserved. Evidence:
`tmp/ryis-winter-expansion-components.json` and
`tmp/ryis-winter-expansion-art-{cycle}-{direction}-{page}.png`; sequence
cycle names include their phase.

The [five-choice summary](../../generated/ryis-winter-expansion-preview/summary.png)
shows hammering, reading and writing. The [complete Vanilla/Blue review](../../generated/ryis-winter-expansion-preview/blue-review/index.html)
covers all 49 cases across fourteen pages, with at most seven cases per page.
Full-review crop `[10,20,60,39]` includes the hammer swing, saw blade and West
mirrors. Each case also has enlarged face details. The summary uses the
East/South crop `[25,20,45,39]` at 4× scale. Exact checks verified 7,618,320
preview pixels, all 98 full-frame and fifteen summary bindings, source metadata
and West reversal. Chromium checked all sixteen pages, including both indexes:
every image and local link loaded, with no horizontal overflow. Evidence:
`tmp/ryis-winter-expansion-preview-check.log` and
`tmp/ryis-winter-expansion-preview-browser-check.json`.

The opt-in `tests/ryis_winter_expansion.rs` passed all new pixels in all four
targets, per-frame counts, sidecars and 29 literal material landmarks
(`tmp/ryis-winter-expansion-corpus-test.log`). The final
`generated/ryis-winter-expansion-trial` preserves all 2,330 prior PNG/metadata
files byte-for-byte; all 976 variants pass exact-palette validation
(`tmp/ryis-winter-expansion-preservation.log`). Profile SHA-256:
`c1de4b262366c674fb81b908b1c87cdf60624a217cf066b7703445c146f30fe1`.

See [the combined integration report](world-winter-expansion.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user approved this slice for commit on 2026-09-25.
