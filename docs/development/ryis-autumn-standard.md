# Ryis's Winter blink, sit, eat and drink

This slice adds 11 Winter strips: 31 source frames and 12 native West
mirrors. The profile now has 228 sources and 912 variants. All 217 prior
regions, other profile fields, source colors and target mappings are unchanged.
The `autumn-standard` artifact prefix follows the shared batch name; Ryis's
content is Winter.

Sources were exported from the read-only `tmp/momi-lab/assets.bak.zip` into
`extracted/ryis-autumn-standard-study`. New sources are under
`assets/animations/NPCs/Ryis/Sprites/Winter/`: blink East/South and sit/eat/drink
North/South/East. All use 80×80 frames, Default atlas and Middle/54 origin.
Blink has three frames with durations `[0.075,0.125,0.075]`; sit uses the
single-frame defaults. Eat East/South has five frames with durations
`[0.125,0.15,0.175,0.125,0.6]`; eat North and drink have three frames at `1.0`.
Raw sidecars in `tmp/ryis-autumn-standard-metadata.json` match the independent
root inventory.

The 89 component seeds select 1,348 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in Vanilla
and all four targets. Exposed fingers remain distinct from dark `353A50`
gloves and cups. The orange coat, scarf, pink shirt, blue trousers and brown
boots stay original. The broad `5E423B` rear-head patch remains short hair;
`410808` and `C83E37` eating mouth details remain unchanged. Evidence:
`tmp/ryis-autumn-standard-components.json` and
`tmp/ryis-autumn-standard-art-{cycle}-{direction}.png`.

| Strip | Skin pixels per source frame, per target |
| --- | --- |
| Blink East | 49, 53, 49 |
| Blink South | 54, 58, 54 |
| Sit East / North / South | 44 / 26 / 50 |
| Drink East | 45, 48, 45 |
| Drink North | 24, 24, 24 |
| Drink South | 50, 56, 50 |
| Eat East | 43, 44, 41, 47, 43 |
| Eat North | 24, 24, 24 |
| Eat South | 50, 52, 44, 59, 50 |

The [five-choice summary](../../generated/ryis-autumn-standard-preview/summary.png)
shows four action samples. The [complete Vanilla/Blue review](../../generated/ryis-autumn-standard-preview/blue-review/index.html)
covers all 43 cases across eight pages, with at most eight cases per page.
Crop `[29,23,22,34]` includes every visible pixel. Independent checks verified
6,432,800 full-review pixels, 538,560 summary pixels, all 20 sample bindings,
metadata and West reversal. Chromium checked every page, image and link.
Evidence: `tmp/ryis-autumn-standard-preview-check.log` and
`tmp/ryis-autumn-standard-preview-browser-check.json`.

The opt-in `tests/ryis_autumn_standard.rs` passed every new pixel in all four
targets, per-frame counts, sidecars and 26 literal material landmarks
(`tmp/ryis-autumn-standard-corpus-test.log`). The final
`generated/ryis-autumn-standard-trial` preserves all 2,170 prior PNG/metadata
files byte-for-byte; all 912 variants match the author outputs
(`tmp/ryis-autumn-standard-preservation.log`). Profile SHA-256:
`cc1ece7546096996e4ec30076ca7e5961bf75dc7582301863ee77ba252215abe`.

See [the combined integration report](world-autumn-standard.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user approved
this slice for commit on 2026-09-25.
