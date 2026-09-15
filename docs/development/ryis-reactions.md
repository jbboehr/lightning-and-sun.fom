# Ryis's Spring shocked reactions and seated reading

This slice adds six strips and 13 South-facing source frames. The profile now
contains 137 sources and 548 variants; all 131 previous region objects, source
colors and target mappings are unchanged. The five F10 choices stay the same.

Sources come from the read-only `tmp/momi-lab/assets.bak.zip`, extracted to
`extracted/ryis-reactions-study`. Under
`assets/animations/NPCs/Ryis/Sprites/Spring/`, the normal filenames are
`spr_npc_ryis_spring_shocked_{start,loop,end}_south.png`; seated reading uses
`spr_npc_ryis_specialanimation_spring_read_sit_{start,loop,end}_south.png`.
All frames retain 80×80 geometry, Default atlas and Middle/54 origin.
Exact sidecars are recorded in `tmp/ryis-reactions-metadata.json`.

| Sequence phase | Frames | Native duration | Changed pixels per target |
| --- | ---: | --- | ---: |
| Shocked start | 1 | Engine default | 66 |
| Shocked loop | 1 | Engine default | 64 |
| Shocked end | 1 | Engine default | 66 |
| Reading start | 3 | `0.1` | 152 |
| Reading loop | 4 | `[3.0,0.1,3.0,0.1]` | 200 |
| Reading end | 3 | `0.1` | 152 |

The unchanged world ramp (`B06C57`, `854D3C`, `63342A`, `491F1B`) selects
700 skin pixels per target through 58 component seeds. All 13 frames were
inspected in Vanilla and all four targets. Book cover colors `2E4D69`,
`426883`, `4E7E9F`, `699CC1` and paper colors `C9AF9C`, `F6E4D7` remain
original, as do hair, clothing, gloves and the shocked mouth's `410808` and
`C83E37`. Detached fingers beside the gloves and book are included.
Inventory and inspected sheets are `tmp/ryis-reactions-components.json` and
`tmp/ryis-reactions-art-{cycle}_{phase}-south.png`.

The [five-choice summary](../../generated/ryis-reactions-preview/summary.png)
is about 52 KB. The [complete Vanilla/Blue review](../../generated/ryis-reactions-preview/blue-review/index.html)
contains all 13 frames across four pages, with at most four cases per page;
the full directory is about 328 KB. These sources have no native West views.
Crop `[27,22,26,35]` includes every visible pixel. Exact checks cover 2,366,000
full-review pixels at 10× and 491,400 summary pixels at 6×, all 15 sample
bindings, metadata and clipping. Chromium decoded every image and checked
all pages and links. Evidence: `tmp/ryis-reactions-preview-pixel-check.log`
and `tmp/ryis-reactions-preview-browser-check.json`.

The opt-in `tests/ryis_reactions.rs` test passes every new source pixel in all
four targets, per-frame counts, alpha, sidecars and 13 literal book, glove,
mouth and finger boundaries (`tmp/ryis-reactions-focused.log`). The final
`generated/ryis-reactions-trial` bundle preserves all 1,310 earlier Ryis
PNG/metadata files byte-for-byte; all 548 variants equal the inspected outputs
(`tmp/ryis-reactions-comparison.log`). Profile SHA-256:
`0c1a4e6ec8c1f1d57e47e599e4d0722d10f638c9eb9882682da67cd50864fcbd`.

See [the combined integration report](spring-world-reactions.md) for shared
packaging, native sequence probes, full checks and installation. Static
previews do not demonstrate timing, natural schedules or live gameplay.
The user accepted the offline artwork review.
