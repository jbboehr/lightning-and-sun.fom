# Hayden's Beach swimming

Two strips add Beach `bath_swim` East/South. Their eight source frames produce
12 review cases with four native West mirrors. The profile now covers 290
sources and 1,160 variants. All 288 accepted regions, color groups and palette
mappings remain unchanged. The user approved this slice for commit on 2026-09-25.

The read-only source is `tmp/fields-of-mistria/assets.zip`, unchanged from the
accepted Beach actions batch, with SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Sources live under `assets/animations/NPCs/Hayden/Sprites/Beach/` with the
`spr_npc_hayden_beach_bath_swim_` prefix. The full corpus is
`extracted/hayden-beach-swim-study`; raw sidecars are recorded in
`tmp/hayden-beach-swim-author-metadata.json`. Both strips retain four 80×80
frames at 0.15 seconds each, Default atlas and Middle/54 origin. No source
hash was changed or relaxed.

The 52 reviewed component seeds select 138 skin pixels per target: 60 in
`#E7B172` and 78 in `#AB7E3F`. Only the face and small exposed neck patches
appear above the water. In one-based frames 1–2 those patches remain visible;
frames 3–4 lower the head and hide them. Exact skin counts per frame are
`[17,17,14,14]` East and `[22,22,16,16]` South. Existing darker skin mappings
and the accepted body-hair treatment remain unchanged; no body-hair region is
visible in these strips.

The 244 dark-water pixels (`#328BC9`) and 185 bright-water/spray pixels
(`#9DEBFC`) stay original, as do hair, beard, eyes, outlines and transparency.
Every actual frame was inspected in Vanilla and all four targets on ignored
`tmp/hayden-beach-swim-author-art-*.png` sheets and an exact material-color
grid. The final sheets and previews use the actual standalone bundle.

- [Five-choice summary](../../generated/hayden-beach-swim-preview/summary.png):
  raised and lowered East/South examples, with frame numbers.
- [Complete Vanilla/Blue review](../../generated/hayden-beach-swim-preview/blue-review/index.html):
  all 12 cases and 24 views across two pages of at most eight cases.

The complete preview directory is about 204 KiB. Crop `[30,42,20,21]`
includes every opaque pixel, including detached splashes. Saved-image checks
cover 1,562,400 exact pixels across all-target art sheets, review sheets and
20 summary bindings, including source/palette identity and West reversal.
Chromium decoded every image across all four HTML pages, verified every case
and local link, and found no horizontal overflow. Evidence is in
`tmp/hayden-beach-swim-author-preview-check.log`,
`tmp/hayden-beach-swim-author-preview-browser-check.json`, and the review's
`coverage.json`.

The focused local corpus test and targeted Clippy passed before the candidate
profile was published. The test checks 60 literal skin/material landmarks,
every source pixel against the independently reviewed material ramp, exact
per-frame skin counts, alpha, metadata and all accepted outputs. Production
data was then checked for exact object equality with that passing candidate.

The test accepts `FOM_HAYDEN_BEACH_SWIM_PRESETS` for ignored negative-control
copies. Removing the exposed-neck component failed at East `[36,54]`;
selecting bright water failed at `[37,55]`. Both intended failures occurred
in the material assertion after successful generation. Evidence is in
`tmp/hayden-beach-swim-author-candidate-test.log` and
`tmp/hayden-beach-swim-author-{omit-neck,spill-water}-red.log`.

All 1,160 final variants passed exact recipe validation; all 2,880 accepted
original/variant PNG and metadata files remain byte-identical. The comparison
report is `tmp/hayden-beach-swim-author-compare.json`. Frozen data and test
hashes are recorded in `tmp/hayden-beach-swim-author-frozen-inputs.sha256`.

See [combined integration](world-beach-swim.md) for shared checks, native
animation verification and installation. Static images do not establish live
animation timing, natural NPC scheduling or state transitions. Artwork stays
ignored; this character slice changes no runtime code.
