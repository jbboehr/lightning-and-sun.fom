# Ryis's Winter idle/walk pilot

This slice adds six Winter strips: 15 source frames and five native West
mirrors. The profile now has 217 sources and 868 variants. All 211 prior
regions, other profile fields, source colors and target mappings are unchanged.
The `autumn-actions` artifact prefix follows the shared batch name; Ryis's
content is Winter.

Sources were exported from the read-only `tmp/momi-lab/assets.bak.zip` into
`extracted/ryis-autumn-actions-study`. The new paths are
`assets/animations/NPCs/Ryis/Sprites/Winter/spr_npc_ryis_winter_{idle,walk}_{north,south,east}.png`.
All use 80×80 frames, Default atlas and Middle/54 origin. Idle uses the
single-frame defaults; walking has four frames at duration `0.15`. Raw sidecars
in `tmp/ryis-autumn-actions-metadata.json` match the independent root inventory.

The 49 component seeds select 601 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every frame was inspected in Vanilla
and all four targets. Exposed fingers remain distinct from dark `353A50`
gloves. Orange coat shades `C16639`, `E08F4E`, `933F2E`, `5F221E` and its brown
hem `925847` stay original, along with the scarf, pink shirt, blue trousers and
brown boots. The broad `5E423B` rear-head patch remains short hair. Evidence:
`tmp/ryis-autumn-actions-components.json` and
`tmp/ryis-autumn-actions-art-{cycle}-{direction}.png`.

| Strip | Skin pixels per source frame, per target |
| --- | --- |
| Idle East / North / South | 45 / 26 / 50 |
| Walk East | 45, 46, 45, 44 |
| Walk North | 26, 25, 26, 25 |
| Walk South | 50, 49, 50, 49 |

The [five-choice summary](../../generated/ryis-autumn-actions-preview/summary.png)
shows idle South, East and North. The [complete Vanilla/Blue review](../../generated/ryis-autumn-actions-preview/blue-review/index.html)
covers all 20 cases across three pages, with at most eight cases per page.
Crop `[29,24,22,34]` includes every visible pixel. Independent checks verified
2,992,000 full-review pixels, 403,920 summary pixels, all fifteen sample
bindings, metadata and West reversal. Chromium checked every page, image and
link. Evidence: `tmp/ryis-autumn-actions-preview-check.log` and
`tmp/ryis-autumn-actions-preview-browser-check.json`.

The opt-in `tests/ryis_autumn_actions.rs` passed every new pixel in all four
targets, per-frame counts, sidecars and 25 literal material landmarks
(`tmp/ryis-autumn-actions-corpus-test.log`). The final
`generated/ryis-autumn-actions-trial` preserves all 2,110 prior PNG/metadata
files byte-for-byte; all 868 variants match the author outputs
(`tmp/ryis-autumn-actions-preservation.log`). Profile SHA-256:
`347d7ac36832f3ddbb900991e58f7c103c3ddf33cc6bee97edc338225ddfe941`.

See [the combined integration report](world-autumn-actions.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. User artwork
acceptance was recorded on 2026-09-19.
