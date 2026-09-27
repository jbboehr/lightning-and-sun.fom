# Juniper's Autumn general actions, sleep and kiss

This slice adds five strips: action North/South/East, sleep East and kiss East.
They contain 26 source frames; twelve native West mirror views make 38 offline
review cases. Juniper now has 228 sources and 912 variants. All 223 earlier
region objects, hashes, groups and palette mappings remain unchanged, including
`F1E791` and previously accepted bracer-edge exceptions. Portrait-only recipes
remain untouched.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The full corpus is `extracted/juniper-autumn-standard-study`. All 446 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-autumn-actions-trial` bundle. Pins remain strict.

Sources are under `assets/animations/NPCs/Juniper/Sprites/Autumn/`, with prefix
`spr_npc_juniper_autumn_`. All retain 80×80 frames, Default atlas and Middle/54
origin. Action strips each have seven frames with durations
`[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`; sleep uses single-frame defaults; kiss has
four frames with durations `[0.15,0.15,0.8,0.15]`. Exported raw sidecars are in
`tmp/juniper-autumn-standard-author-metadata.json`. The root's independent
inventory confirms the counts and native mirror support. Sleep/kiss West views
exercise the native Single pack and NPC mirror; they do not establish natural
gameplay dispatch.

Every actual frame was inspected in Vanilla and all four target palettes. Face,
ears, neckline, midriff and moving hands change. Hair, circlet, gems, cuffs,
sleeves, skirt, boots, cosmetics and outlines stay original. The sleeping hand
beside the face and the changing closed-mouth silhouette are included.

No new source shade or coupled-material exception is needed. Matching-color
cuff edges are separate components in this batch. In one-based frames:

- North action frames 2–5 preserve the shaded cuff top at `[34,44]`; the hand
  directly below changes. Frame 6 preserves `[33,45]` beside cuff gold, while the
  same `BC8B43` shade at `[33,47]` belongs to the fingers and changes.
- South action frame 1 preserves the far cuff's `E3BF7F`/`BC8B43` pair at
  `[45,46]`/`[46,46]`; finger shading immediately below changes. Equivalent
  moving boundaries are checked across the strip.
- East action frames 2/4 preserve `[46,44]`, below the cuff gemstone; frames 3/5
  preserve the lowered cuff corner `[45,44]`. The extended and lowered fingers
  remain covered.
- Sleep preserves the cuff corner `[44,41]`, while the raised hand and finger
  crease above it change. Kiss keeps the moving cuff borders and eye cosmetics.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Action North | 5, 5, 5, 5, 5, 6, 12 |
| Action South | 40, 39, 38, 39, 38, 40, 44 |
| Action East | 31, 32, 29, 32, 29, 31, 37 |
| Sleep East | 32 |
| Kiss East | 34, 34, 37, 38 |

The masks add 337 seeds and recolor 717 pixels per target; 112 matching-color
jewelry pixels remain protected. Evidence is in
`tmp/juniper-autumn-standard-author-refined-components.json`,
`tmp/juniper-autumn-standard-author-exclusions.json` and the eight
`tmp/juniper-autumn-standard-author-refined-art-*.png` sheets. Profile SHA-256:
`a04e26a0a76bcd4ebc742233485795d1ec9d767101b98616fcafe7e9b4e7d57e`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-autumn-standard-preview/summary.png)
shows action South frame 3, sleep East frame 1 and kiss East frame 3. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-autumn-standard-preview/blue-review/index.html)
shows all 38 cases over twelve detail pages, including enlarged face views.

The full-body crop `[25,20,32,39]` includes every visible pixel. Exact checks
verified 7,618,080 preview pixels, 76 full-frame bindings, fifteen summary
bindings, raw metadata, frame counts and West mirrors. Chromium checked all
fourteen pages including both indexes: every local link and image loaded,
with no horizontal overflow. Evidence:
`tmp/juniper-autumn-standard-author-preview.log` and
`tmp/juniper-autumn-standard-author-preview-browser-check.json`.

The independent candidate test and targeted Clippy passed before promotion;
the final focused test in `tests/juniper_autumn_standard.rs` passed afterward.
It checks every new pixel in four targets, per-frame counts, unchanged metadata,
58 literal skin/material landmarks and all 1,784 prior variant PNG/metadata
files. Practical controls drop a finger-shadow component, drop the midriff
highlight component and merge groups; these expose missed skin and spill into
a protected cuff. Evidence: `tmp/juniper-autumn-standard-author-candidate-test.log`,
`tmp/juniper-autumn-standard-author-clippy.log` and
`tmp/juniper-autumn-standard-author-focused-test.log`.

All 912 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,230 prior original/variant PNG and metadata files are
byte-identical; all 1,824 final variant files match the inspected candidates.
Evidence: `tmp/juniper-autumn-standard-author-preservation.log`,
`tmp/juniper-autumn-standard-author-stylized-validation.json` and
`tmp/juniper-autumn-standard-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Static previews do not establish live timing, schedules
or outfit transitions. Game-derived images remain ignored.
