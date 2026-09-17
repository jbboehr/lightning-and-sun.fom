# Ryis's Autumn idle/walk pilot

This slice adds six Autumn strips: 15 source frames and five native West
mirrors. The profile now has 184 sources and 736 variants. All 178 prior
regions, other profile fields, source colors and target mappings are unchanged.

Sources were exported from the read-only `tmp/momi-lab/assets.bak.zip` into
`extracted/ryis-seasonal-actions-study`. The new paths are
`assets/animations/NPCs/Ryis/Sprites/Autumn/spr_npc_ryis_autumn_{idle,walk}_{north,south,east}.png`.
All use 80×80 frames, Default atlas and Middle/54 origin. Idle uses the
single-frame engine defaults; walking has four frames at duration `0.15`.
Raw sidecars are retained in `tmp/ryis-seasonal-actions-metadata.json`.

The 51 component seeds select 623 skin pixels per target using the existing
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Face, ears, neck and exposed
fingertips change. The blue coat and scarf, light cuffs, yellow `F4CD86` gloves,
orange shirt, belt, covered legs and brown boots remain original. The broad
`5E423B` rear-head patch remains short hair. Every source frame was inspected
in Vanilla and all four targets, including the detached fingertips beside
moving gloves. Component inventory and art sheets are
`tmp/ryis-seasonal-actions-components.json` and
`tmp/ryis-seasonal-actions-art-{cycle}-{direction}.png`.

| Strip | Skin pixels per source frame, per target |
| --- | --- |
| Idle East / North / South | 47 / 26 / 52 |
| Walk East | 47, 49, 47, 47 |
| Walk North | 26, 25, 26, 25 |
| Walk South | 52, 51, 52, 51 |

The [five-choice summary](../../generated/ryis-seasonal-actions-preview/summary.png)
shows idle South, East and North. The [complete Vanilla/Blue review](../../generated/ryis-seasonal-actions-preview/blue-review/index.html)
covers all 20 cases across three pages, at most eight cases per page. Crop
`[29,24,22,34]` includes every visible pixel. Independent checks verified
2,992,000 full-review pixels, 403,920 summary pixels, all 15 sample bindings,
raw metadata and West reversal. Chromium decoded every image and checked all
pages and links. Evidence: `tmp/ryis-seasonal-actions-preview-check.log` and
`tmp/ryis-seasonal-actions-preview-browser-check.json`.

The opt-in `tests/ryis_seasonal_actions.rs` passed, checking every new pixel
in all four targets, per-frame counts, sidecars and 23 literal skin/material
landmarks (`tmp/ryis-seasonal-actions-corpus-test.log`). The final
`generated/ryis-seasonal-actions-trial` preserves all 1,780 prior PNG/metadata
files byte-for-byte; all 736 variants match the author outputs
(`tmp/ryis-seasonal-actions-preservation.log`). Profile SHA-256:
`8ea1626421b0b1f82044d014bd452361f60be67fc7f023c371de85f3cfb7ef22`.

See [the combined integration report](world-seasonal-actions.md) for shared
packaging, native probes, full checks and installation. Static previews do
not establish timing, natural outfit changes or live gameplay. User artwork
acceptance was received on 2026-09-16.
