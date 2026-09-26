# Ryis's Beach swimming

This slice adds two Beach swimming strips: eight source frames and four native
West mirrors. The profile now has 258 sources and 1,032 variants. All 256 prior
region objects, other profile fields, source colors and target mappings remain
unchanged.

Sources were exported from the read-only `tmp/fields-of-mistria/assets.zip`
into `extracted/ryis-beach-swim-study`. Every prior Ryis PNG hash and all 512
original PNG/metadata files match the retained Beach-actions combined build.
New sources under `assets/animations/NPCs/Ryis/Sprites/Beach/` are
`spr_npc_ryis_beach_bath_swim_east.png` and
`spr_npc_ryis_beach_bath_swim_south.png`. Both retain four 80×80 frames at
`0.15`, Default atlas and Middle/54 origin. Raw exported sidecars are in
`tmp/ryis-beach-swim-author-metadata.json` and match the independent root inventory.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Swim East | 38, 38, 36, 36 |
| Swim South | 41, 41, 39, 39 |

The twenty component seeds select 308 skin pixels per target with the unchanged
`B06C57`, `854D3C`, `63342A`, `491F1B` ramp. Every actual frame was inspected
in Vanilla and all four targets, including all detached splash pixels. Forehead,
face, ears and the edge beneath the chin change. Hair and sideburn shades,
eye details, outlines, dark water (`328BC9`) and light foam/splashes (`9DEBFC`)
remain original. No clothing or limbs are exposed in these swimming strips.
Evidence: `tmp/ryis-beach-swim-author-components.json` and
`tmp/ryis-beach-swim-author-art-bath_swim_{direction}-0.png`.

The [five-choice summary](../../generated/ryis-beach-swim-preview/summary.png)
shows East frame one and South frames one and three. The [complete Vanilla/Blue review](../../generated/ryis-beach-swim-preview/blue-review/index.html)
covers all twelve cases across three pages, four cases per page. Swimming heads
sit lower in their 80×80 source frame than the standing sprites; crop
`[25,38,32,27]` includes every visible pixel, including splashes down to row 62
and all West mirrors. Each case also has a 10× face and waterline detail.
Exact checks verified 2,064,096 preview pixels, all 24 full-frame and fifteen
summary bindings, metadata and West reversal. Chromium checked all five pages,
including both indexes: every image and local link loaded, with no horizontal
overflow. Evidence: `tmp/ryis-beach-swim-author-preview-check.log` and
`tmp/ryis-beach-swim-author-preview-browser-check.json`.

The candidate corpus test passed before publishing the profile. The final opt-in
`tests/ryis_beach_swim.rs` passed every new pixel in all four targets, per-frame
counts, sidecars and twenty literal skin/material landmarks. Practical controls
cover an omitted isolated forehead component and palette spill into foam at
the chin. Log: `tmp/ryis-beach-swim-author-corpus-test.log`. The final
`generated/ryis-beach-swim-trial` preserves all 2,560 prior original and variant
PNG/metadata files byte-for-byte; all 1,032 variants pass exact-palette validation
(`tmp/ryis-beach-swim-author-preservation.log`). Profile SHA-256:
`cf393487b4059d333fd0e294c35a267b13229d1db2ea7f873c9e097fe22c14f0`.

See [the combined integration report](world-beach-swim.md) for shared packaging,
native probes, full checks and installation. Static previews do not establish
timing, natural outfit changes or live gameplay. The user approved this slice for commit on 2026-09-25.
