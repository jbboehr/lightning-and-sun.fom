# Hayden's Autumn blink, sit, eat and drink

Eleven Autumn strips add blink East/South and sit/eat/drink North/South/East.
The 31 source frames produce 43 review cases including 12 native West
mirrors. The profile now covers 233 sources and 932 variants; all 222
accepted region objects, groups and mappings remain unchanged. User
artwork acceptance was recorded on 2026-09-19.

The read-only source is `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Autumn/`, with the
`spr_npc_hayden_autumn_` prefix. The full corpus is
`extracted/hayden-autumn-actions-study`; raw sidecars are recorded in
`tmp/hayden-autumn-actions-metadata.json`. All frames retain 80×80 geometry,
Default atlas and Middle/54 origin. Native `blink` keeps three frames at
`[0.075,0.125,0.075]`; `sit` keeps single-frame defaults; `drink` and North
`eat` keep three frames at 1.0 seconds. East/South `eat` keeps five frames
at `[0.125,0.15,0.175,0.125,0.6]`.

The 469 reviewed component seeds select 1,127 skin pixels per target.
No new shade or mapping was needed. Purple sleeve edges, hair, beard,
trousers and boots remain original. The eating mouth preserves both
`#410808` and `#9E2626`, including zero-based East frame 2 `[40,32]` and
`[40,33]`, while nearby hands change. Separate food or cup artwork is not
baked into these source strips.

All 31 frames were inspected in all four targets on ignored
`tmp/hayden-autumn-actions-art-*.png` sheets. Component decisions and frozen
hashes are recorded in `tmp/hayden-autumn-actions-refinement.json` and
`tmp/hayden-autumn-actions-frozen-inputs.sha256`.

- [Five-choice summary](../../generated/hayden-autumn-actions-preview/summary.png):
  blink South, sit North, eat East and drink East samples.
- [Complete Vanilla/Blue review](../../generated/hayden-autumn-actions-preview/blue-review/index.html):
  all 43 cases and 86 views across six pages of at most eight cases.

Verification covers all 2,220 prior PNG/metadata files, 88 reviewed new
PNG/metadata bindings, full crop `[28,23,24,34]`, 7,017,600 review pixels,
261,120 summary pixels, 20 sample bindings and native West reversal.
The focused corpus test and targeted Clippy passed. The test checks 38
literal material landmarks, every pixel, per-frame coverage, alpha, metadata
and prior outputs. Chromium decoded every image and verified all cases and
links. Evidence is in
`tmp/hayden-autumn-actions-{compare,art-binding,preview-check,test}.log`
and `tmp/hayden-autumn-actions-preview-browser-check.json`.

See [combined integration](world-autumn-actions.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this character
slice changes no runtime code.
