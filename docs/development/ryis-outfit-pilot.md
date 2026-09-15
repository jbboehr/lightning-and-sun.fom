# Ryis's Summer idle/walk pilot

This slice adds six Summer strips and 15 source frames, plus five native West
mirrors. The profile now has 151 sources and 604 variants. All 145 previous
region objects, source colors and target mappings remain unchanged.

Sources come from the read-only `tmp/momi-lab/assets.bak.zip`, exported to
`extracted/ryis-outfit-pilot-study`. Under
`assets/animations/NPCs/Ryis/Sprites/Summer/`, the six names are
`spr_npc_ryis_summer_{idle,walk}_{north,south,east}.png`. All use 80×80 frames,
Default atlas and Middle/54 origin. Idle uses the single-frame engine defaults;
walking has four frames at duration `0.15`. Raw sidecars and counts are in
`tmp/ryis-outfit-pilot-metadata.json`.

The 116 seeds select 834 skin pixels per target using the unchanged ramp
`B06C57`, `854D3C`, `63342A`, `491F1B`. Summer exposes lower legs between the
shorts and pink footwear. These use the skin ramp; the footwear uses `FFA799`,
and dark gloves use `353A50`. Detached fingers around the gloves are included.
The broad `5E423B` patch at the rear of his head remains short hair. Hair, eyes,
shirt, undershirt, gloves, shorts and footwear stay original. All 15 frames were
inspected in Vanilla and all four targets, including the moving glove/leg
boundaries. Component inventory and inspected sheets are
`tmp/ryis-outfit-pilot-components.json` and
`tmp/ryis-outfit-pilot-art-{cycle}-{direction}.png`.

| Strip | Skin pixels per source frame, per target |
| --- | --- |
| Idle East | 60 |
| Idle North | 42 |
| Idle South | 69 |
| Walk East | 60, 61, 60, 58 |
| Walk North | 42, 38, 42, 38 |
| Walk South | 69, 63, 69, 63 |

The [five-choice summary](../../generated/ryis-outfit-pilot-preview/summary.png)
shows idle South, East and North. The [complete Vanilla/Blue review](../../generated/ryis-outfit-pilot-preview/blue-review/index.html)
covers all 20 cases across three pages, at most eight cases each. Crop
`[29,24,22,34]` includes every visible pixel. Exact checks cover 2,992,000
full-review pixels at 10×, 403,920 summary pixels at 6×, all 15 sample bindings,
native metadata and West reversal. Chromium decoded every image and checked
all pages and links. Evidence: `tmp/ryis-outfit-pilot-preview-pixel-check.log`
and `tmp/ryis-outfit-pilot-preview-browser-check.json`.

The opt-in `tests/ryis_outfit_pilot.rs` checks every new pixel in all four
targets, per-frame counts, alpha, sidecars and 17 literal skin/material
boundaries (`tmp/ryis-outfit-pilot-corpus-test.log`). The final
`generated/ryis-outfit-pilot-trial` preserves all 1,450 prior Ryis PNG/metadata
files byte-for-byte; all 604 variants equal the inspected author outputs
(`tmp/ryis-outfit-pilot-comparison.log`). Profile SHA-256:
`5c498288787e3942f34e6f0b6c3a7e8555a3aa3b81732fdfd39047b7e3137c6e`.

See [the combined integration report](world-outfit-pilots.md) for shared
packaging, native probes, full checks and installation. Static previews do not
establish timing, natural outfit changes or live gameplay. The user accepted
the offline artwork review.
