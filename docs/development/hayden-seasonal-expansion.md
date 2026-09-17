# Hayden's Autumn idle and walk

Six Autumn idle/walk strips cover North, South and East: 15 source frames,
plus five native West mirrors. The profile now covers 222 sources and 888
variants. All 216 accepted region objects, groups and mappings remain
unchanged. The user accepted this offline artwork on 2026-09-16.

The read-only source is `tmp/momi-lab/assets.bak.zip`, under
`assets/animations/NPCs/Hayden/Sprites/Autumn/`, with the
`spr_npc_hayden_autumn_` prefix. The full corpus is
`extracted/hayden-seasonal-expansion-study`; raw sidecars are recorded in
`tmp/hayden-seasonal-expansion-metadata.json`. Native `idle` retains its
single-frame defaults; `walk` retains four frames at 0.15 seconds each.
All retain 80×80 geometry, Default atlas and Middle/54 origin.

The 202 reviewed component seeds select 500 skin pixels per target.
Autumn's purple shirt uses distinct colors; all four `#523C26` pixels in
these strips are moving-hand shadows. Their zero-based frame/coordinates
are walk East frame 1 `[35,43]` and `[35,44]`, and walk South frames 1 and 3
at `[45,41]` and `[34,41]`. They change with the skin; purple sleeve edges,
hair, beard, mouth, trousers and boots stay original. No new shade or
mapping was needed. The stylized description now includes Autumn.

All 15 frames were inspected in all four targets on ignored
`tmp/hayden-seasonal-expansion-art-*.png` sheets. Evidence includes
`tmp/hayden-seasonal-expansion-refinement.json` and frozen input hashes.

- [Five-choice summary](../../generated/hayden-seasonal-expansion-preview/summary.png):
  idle South and walk North/East samples.
- [Complete Vanilla/Blue review](../../generated/hayden-seasonal-expansion-preview/blue-review/index.html):
  20 cases and 40 views across three pages of at most eight cases.

Verification covers all 2,160 prior PNG/metadata files, all 48 new reviewed
PNG/metadata bindings, full crop `[29,24,22,34]`, 2,992,000 review pixels,
179,520 summary pixels, 15 sample bindings and native West reversal.
The focused corpus test and targeted Clippy passed. The test checks 30
literal material landmarks, every pixel, per-frame coverage, alpha, metadata
and prior outputs. Chromium decoded every image and verified all cases and
links. Results are in
`tmp/hayden-seasonal-expansion-{compare,art-binding,preview-check,test}.log`
and `tmp/hayden-seasonal-expansion-preview-browser-check.json`.

See [combined integration](world-seasonal-expansion.md) for full checks,
native probes and isolated installation. Static review does not exercise
live timing or state transitions. Artwork remains ignored; this character
slice changes no runtime code.
