# Ryis's normal Spring action, sleep and kiss strips

This slice adds five strips and 26 source frames to the accepted Spring
blink/sit/eat/drink batch. The profile now has 131 sources and 524 variants.
All 126 earlier region objects, source colors and target mappings remain
unchanged. F10 still offers Vanilla, Debug Blue, Adeline, Hayden and Seridia.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`; the complete local
corpus is `extracted/ryis-standard-study`. Exact native sidecars are recorded
in `tmp/ryis-standard-metadata.json`. Source filenames under
`assets/animations/NPCs/Ryis/Sprites/Spring/` are
`spr_npc_ryis_spring_{cycle}_{direction}.png`.

| Strip | Frames | Native duration | Changed pixels per target |
| --- | ---: | --- | ---: |
| Action North | 7 | `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]` | 196 |
| Action South | 7 | Same | 398 |
| Action East | 7 | Same | 353 |
| Sleep East | 1 | Omitted; engine default | 50 |
| Kiss East | 4 | `[0.15,0.15,0.8,0.15]` | 214 |

All frames retain 80×80 geometry, Default atlas and Middle/54 origin. The
107 component seeds select 1,211 skin pixels per target using the unchanged
world ramp `B06C57`, `854D3C`, `63342A`, `491F1B`. All 26 frames were inspected
in Vanilla and all four targets. The broad rear-head `5E423B` area remains
short hair. Yellow gloves, clothing and black facial contours stay original;
isolated fingers beside the extended action hand and raised sleep glove are
included. Inventory: `tmp/ryis-standard-components.json`; inspected sheets:
`tmp/ryis-standard-art-{cycle}-{direction}.png`.

Profile SHA-256:
`a072ac1deb3ff356528279f52736b0620e23271388d0ae2ed24d96ac8be578d9`.
The profile, set and stylized recipe hashes are frozen in
`tmp/ryis-standard-frozen-inputs.sha256`.

## Review and verification

- [Five-choice summary](../../generated/ryis-standard-preview/summary.png):
  Action East frame 2, Sleep East frame 1 and Kiss East frame 3, about 48 KB.
- [Complete Vanilla/Blue review](../../generated/ryis-standard-preview/blue-review/index.html):
  all 26 source frames and 12 native West mirrors across six pages, at most
  eight cases per page; about 776 KB in total.

Crop `[29,23,24,34]` includes every visible pixel. Exact checks cover 6,201,600
full-review pixels at 10× and 440,640 summary pixels at 6×, all 15 sample
bindings, native metadata, clipping and West reversal. Chromium decoded every
image and checked all 38 cases, six pages and links. Evidence:
`tmp/ryis-standard-preview-pixel-check.log` and
`tmp/ryis-standard-preview-browser-check.json`.

The opt-in `tests/ryis_standard.rs` corpus test passes all four targets against
every new source pixel, per-frame counts, alpha, sidecars and nine literal
skin/material landmarks. Evidence: `tmp/ryis-standard-focused.log`.
The final `generated/ryis-standard-trial` bundle preserves all 1,260 prior
Ryis PNG/metadata files byte-for-byte, and all 524 variants match the inspected
outputs: `tmp/ryis-standard-comparison.log`.

Combined packaging, native runtime probes, full repository checks and isolated
installation are recorded in [the Spring standard integration](spring-world-standard.md). These static
previews do not establish timing, natural schedules or live gameplay. The user
accepted this expanded artwork in the offline review.
