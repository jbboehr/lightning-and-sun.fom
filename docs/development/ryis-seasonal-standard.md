# Ryis's Autumn blink, sit, eat and drink

This slice adds 11 Autumn strips: 31 source frames and 12 native West
mirrors. The profile now has 195 sources and 780 variants. All 184 prior
region objects, other profile fields, source colors and target mappings remain
unchanged.

The read-only `tmp/momi-lab/assets.bak.zip` was exported into
`extracted/ryis-seasonal-standard-study`. New sources are under
`assets/animations/NPCs/Ryis/Sprites/Autumn/`: blink East/South and sit/eat/drink
North/South/East. All use 80×80 frames, Default atlas and Middle/54 origin.
Blink has three frames with durations `[0.075,0.125,0.075]`; sit uses the
single-frame defaults. Eat East/South has five frames with durations
`[0.125,0.15,0.175,0.125,0.6]`; eat North and drink have three frames at `1.0`.
Raw sidecars are retained in `tmp/ryis-seasonal-standard-metadata.json`.

The 91 component seeds select 1,374 skin pixels per target using the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Face, closed eyelids, ears, neck
and exposed fingers change. Yellow gloves and props, light cuffs, blue coat
and scarf, covered legs and brown boots stay original. The rear-head `5E423B`
patch remains short hair; `410808` and `C83E37` mouth details remain unchanged.
Every frame was inspected in Vanilla and all four targets. Component inventory
and art sheets are `tmp/ryis-seasonal-standard-components.json` and
`tmp/ryis-seasonal-standard-art-{cycle}-{direction}.png`.

| Strip | Skin pixels per source frame, per target |
| --- | --- |
| Blink East | 51, 55, 51 |
| Blink South | 56, 60, 56 |
| Sit East / North / South | 46 / 26 / 52 |
| Drink East | 43, 47, 43 |
| Drink North | 24, 24, 24 |
| Drink South | 51, 57, 51 |
| Eat East | 43, 43, 42, 47, 45 |
| Eat North | 24, 24, 24 |
| Eat South | 52, 55, 45, 61, 52 |

The [five-choice summary](../../generated/ryis-seasonal-standard-preview/summary.png)
shows four action samples. The [complete Vanilla/Blue review](../../generated/ryis-seasonal-standard-preview/blue-review/index.html)
covers all 43 cases across eight pages, at most eight cases per page. Crop
`[29,23,22,34]` includes every visible pixel. Independent checks verified
6,432,800 full-review pixels, 538,560 summary pixels, all 20 sample bindings,
raw metadata and West reversal. Chromium checked every page, image and link.
Evidence: `tmp/ryis-seasonal-standard-preview-check.log` and
`tmp/ryis-seasonal-standard-preview-browser-check.json`.

The opt-in `tests/ryis_seasonal_standard.rs` passed: every new pixel in all
four targets, per-frame counts, sidecars and 24 literal skin/material landmarks
(`tmp/ryis-seasonal-standard-corpus-test.log`). The final
`generated/ryis-seasonal-standard-trial` preserves all 1,840 prior PNG/metadata
files byte-for-byte, and all 780 variants match the author outputs
(`tmp/ryis-seasonal-standard-preservation.log`). Profile SHA-256:
`140483fea3efdfd5571edafe6bde6231df302ddc419b01a9ee45f1c8b286b429`.

See [the combined integration report](world-seasonal-standard.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user accepted
this offline artwork on 2026-09-16.
