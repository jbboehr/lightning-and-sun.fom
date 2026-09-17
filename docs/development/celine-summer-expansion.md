# Celine's Summer idle and walking sprites

This slice adds six regular Summer idle/walk strips, North/South/East. Their
15 source frames contain nine distinct images; native West mirroring adds five
review cases. Other Summer actions remain outside this slice.

Sources are `spr_npc_celine_summer_{idle,walk}_{direction}.png` under
`assets/animations/NPCs/Celine/Sprites/Summer/`. All frames are 80×80, Default
atlas, Middle/54 origin. Idle uses single-frame defaults; walk has four frames
at duration `0.15`. Exact sidecars are in
`tmp/celine-summer-expansion-metadata.json`. The read-only source archive is
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The profile grows from 236 to 242 regions. Earlier regions, seven color groups
and source/target mappings remain unchanged. Existing world shades `FCD9B3`,
`F0B988` and `D37A57` retain light/middle/shadow roles; selected `672115`
components retain the deep role. No new colors were needed.

Bare arms, hands, calves and exposed feet recolor. The long hair, pink outfit,
belt, scarf, eyes and brown sandal straps stay original. Component selection
distinguishes the shared dark hand/face contours from hair tips and belt edges.
For example, North idle hand `[33,47]` maps while hair `[35,45]` and `[38,46]`
stay original. East walking frame two calf contours `[41,50]` and `[39,51]`
map while belt `[42,46]` and sandal strap `[41,53]` stay original. Coordinates
are frame-local; frame numbers start at one.

The 240 new seeds select 758 pixels per target: 614 core skin and 144 outline
pixels. Another 115 matching outline pixels stay protected. Idle East/North/South
select 61/26/70 pixels; walking selects 249/90/262. Every frame contains selected
skin, ranging from 19 to 71 pixels. Profile SHA-256:
`67375459630b8cb4a444cdafd2f2f63610dd54526331ecc181625ded6271c046`.

All 15 original and four-target frames were inspected. The focused opt-in
`tests/celine_summer_expansion.rs` passes four literal target ramps, 33 source-art
landmarks, every new pixel, alpha, metadata, complete core shade coverage and
common selection. All 968 variants validate; the standalone bundle matches the
inspected candidate. All 2,360 prior original/variant PNG and metadata files
remain byte-identical, and all 24 duplicate-target comparisons pass.

- [Five-choice summary](../../generated/celine-summer-expansion-preview/summary.png):
  South/North idle and East/South walking samples.
- [Complete Vanilla/Blue review](../../generated/celine-summer-expansion-preview/blue-review/index.html):
  all 20 direction/frame cases on three pages, at most eight cases per page.

Fresh crop `[29,24,22,34]` preserves every visible pixel. Exact checks cover
2,992,000 full-review pixels, 239,360 summary pixels, all source/palette/sample
bindings, original metadata and West reversal. Chromium verifies all three
pages, images and links. Evidence is under `tmp/celine-summer-expansion-`,
especially `final-audit.json`, `test.log`, `mask-decisions.json`,
`preview-check.log` and `preview-browser-check.json`.

The user accepted the offline artwork on 2026-09-16. Static review does not exercise live outfit
switching or game timing. Shared native, installation and full-check evidence
belongs in [the Summer expansion batch notes](world-summer-expansion.md).
Extracted artwork, generated previews and local evidence remain ignored.
