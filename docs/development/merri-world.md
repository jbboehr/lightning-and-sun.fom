# Merri: complete four-season world candidate

2026-11-05: authored and locally validated. **Displayed world art was accepted
with qualification on 2026-11-06; live play remains unverified.**
This approved finish-plan batch adds all 44 seasonal world strips to a new
`merri-world-trial` profile/set. The prior 32 portraits, eight source-color roles,
singleton groups and four preset mappings are preserved exactly. Shared registry,
collection selection and completion counts are owned by the integrator.

## Source and material decisions

The preserved archive is SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Every PNG is pinned in the profile. Export evidence additionally pins each source
metadata file in `local/authoring/merri-pilot/export.json`.

All four outfits have action East/North/South (seven frames each), blink East/South
(three each), idle East/North/South (one each) and walk East/North/South (four each).
There are 168 frame occurrences, 92 exact decoded unique frames and 76 duplicate
occurrences. No previously user-accepted world masks existed. Duplicate reuse here
is reuse of newly authored candidate masks, not reuse of accepted world art.

The actual world skin ramp is `#DD9266`, `#B36644`, `#9E512F`, `#7F382B`.
These already have portrait roles, so this batch needs no new colors or changed
role semantics. Source inspection corrected the preliminary color guess:
`#B36C44` and `#95212F` do not occur. `#E8B58A` belongs to the Summer blouse and
Winter footwear and stays unchanged. The Summer portrait independently confirms
the pale garment is clothing rather than an exposed torso.

Face, ear, closed-eyelid skin, neck and visible hands change. Black eye/lash/mouth
lines, gray hair, orange Spring sleeves/trousers, Autumn jacket/shirt/boots, Summer
blouse/pink scarf/suspenders/waist garment/boots, and Winter collar/coat/gloves/boots
retain their pixels. Spring uses `#7F382B` on 28 clothing pixels across neckline,
sleeve folds and trousers. Exact coordinate exclusions protect these components;
the tests independently assert them alongside fixed per-frame selected counts.
Winter walk East frame 2 has two exposed wrist pixels at local `(36,44)` and
`(36,45)` that change. The rest of the Winter hands are covered by gloves.

Winter action/idle/walk North contain no visible skin. Those three strips have
intentional empty masks, justified by the back-facing hair, high collar, gloves
and footwear, and remain byte-identical in decoded pixels across all variants.
The other 41 strips contain 5,941 selected pixels in total over all frame
occurrences; all four generated variants use the same material selection.

The source Spring action North frame 1 also contains one isolated opaque brown
pixel at `(53,72)`, color `#4C1C0A`. It is preserved. The first crop check caught
its exclusion; final full views extend through row 73 and include it. Face/hand
details are supplementary crops, never the only view of a case.

## Review artifacts and checks

- [Five-choice SAMPLE overview](../../generated/merri-world-finish-review/summary.png).
- [Every-frame review, split into four outfits](../../generated/merri-world-finish-review/index.html):
  168 source frames plus 60 native West mirrors, 228 cases over 76 small case pages.
  Each case has Vanilla left and Debug Blue right, full visible sprites and enlarged
  face/hand details. This is the user-facing candidate review.
- [Component authoring gallery](../../generated/merri-world-mask-review/index.html):
  44 strips, 168 reconstructed candidate frame masks, 92 unique frames, no conflicts.
  Its reference-mask status is tool evidence and does not establish user acceptance.
- Reproduction inputs and measurements: `local/authoring/merri-pilot/`.
  Sources: `extracted/merri-world-finish-study/`; the stable corpus link is
  `extracted/test-corpus/merri`. Candidate variants and MOMI package:
  `generated/merri-world-finish-build/`.

Fresh focused verification passed:

```sh
nix-shell --pure --run 'cargo clippy --locked --release --test merri_world -- -D warnings'
nix-shell --pure --run 'cargo test --locked --release --test merri_world -- --include-ignored'
nix-shell --pure --run 'cargo test --locked --release --test merri_portraits -- --ignored'
```

The two regular checks preserve the prior portrait definition and palette roles,
and package all 44 synthetic strips while preserving 168 native frames, timing,
offsets and atlas properties. The opt-in local check builds the real complete
76-strip corpus, verifies every world RGBA pixel in all four presets against
independent source-material expectations and fixed counts, protects the 28
shared-color clothing pixels, preserves all metadata bytes, and rebuilds prior
portraits to compare all 256 prior PNG/metadata files byte-for-byte.
The retained Merri portrait skin-fringe/iris/clothing test also passed unchanged.

Two negative probes prove the material assertions fail when a Spring hand component
is omitted or a same-color Spring sleeve component is selected. Their retained
logs identify the exact wrong pixel. Each preset also passed the CLI's exact
recipe validation over all 76 strips, with source hashes, dimensions, alpha and
metadata checked and no source-hash override.

The gallery helper reconstructs every displayed cell from the current generated
variant, checks 27,224,064 enlarged pixels, requires all 456 full Vanilla/Blue
bindings plus details, and independently checks complete visible bounds and
native mirrors against source metadata. The complete source/candidate corpus and
West mirrors were visually inspected by the authoring agent. This is agent
inspection, not user acceptance. CPU Chromium passed all 81 desktop pages plus
five mobile index pages: every local link and image exists/loads and no viewport
overflows. The 86 page loads took 42.99s, with evidence in
`local/authoring/merri-pilot/browser-check.json`.

## Pilot measurements and limits

The observed source-sheet/authoring checkpoint began at `2026-11-06T00:27:46Z`;
the first passing three-test local material run was recorded at `00:35:29Z`.
That 7m43s window covers 92 newly authored unique masks, material investigation,
all-case agent visual inspection, gallery preparation and focused checks. It
excludes earlier context reading and the integrator's initial source export.
Later Chromium/final checks are recorded separately in the retained measurements.
This is one simple character's observed agent time, not a delivery forecast or
a measurement of human visual-review throughput.

Measured CPU wall times: full 76-strip/four-preset build 0.281s; old portrait build
0.214s; four exact recipe validations 1.113s combined; final gallery generation and
pixel/bounds verification about 1.5s; first release test compilation 2.40s and test
execution 0.66s. Source-export time was not measured. No local GPU/model inference
was used. Human visual-review and live-gameplay time remain unmeasured.

At the original authoring checkpoint, no user had accepted this candidate and no
accepted baseline had been written. Full
collection checks, combined installation/removal, retained cross-character suites
and natural gameplay are integration/release work and were not claimed here.
All game images and generated packages remain ignored local artifacts. The local
source/build/review directories retained for this batch total about 32 MiB, plus
roughly 1.4 MiB of reusable authoring evidence and temporary inspection sheets.

## Displayed-art acceptance, 2026-11-06

The user replied "looks okay to me" to the consolidated remaining-art review,
including Merri's 44 world strips and 228 comparison cases. Record qualified
acceptance of the displayed world artwork; inspection depth is unspecified.
This closes Merri's art gate without establishing live-game, menu or input
acceptance. The accepted snapshot is frozen under `local/fixtures/finish-merri`.
See the [release-candidate assessment](linux-release-candidate-2026-11-06.md) for
shared review and snapshot/install binding evidence and overall limits. No artwork changed.
