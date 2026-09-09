# Luc portraits

The profile covers all 32 main Luc portrait strips in the supplied archive.
Each season has eight expressions: embarrassed, happy, mad, neutral, sad,
think, ugh and wink. Each 592×180 strip contains two unique 296×180 frames,
with duration 0.2. All 64 frames were inspected. Metadata assigns eight strips
to each corresponding seasonal portrait atlas.

The original archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The local source export is `extracted/luc-portraits-study`. Source images,
generated variants, galleries and temporary Rust authoring helpers remain
ignored. The repository stores the recipes, explicit component seeds, this
note and the focused test.

## Palette and boundaries

Luc's catalog has three shades. Four extra colors cover fine lip, eye, ear,
neck and hand contours. Two more blend skin into the glasses rim. Natural skin
presets use their catalog's four shade roles; the rim uses pale blends of the
target's light shade and the original white rim.

| Source | Role | Use |
| --- | --- | --- |
| `#C48E68` | Light | Main skin, also present in the irises |
| `#B36844` | Medium | Skin shading, also shared with glasses and clothes |
| `#712922` | Deep | Skin contours, also shared with glasses and eyes |
| `#9B533C` | Shadow | Closed and speaking lower-lip shading |
| `#83462E` | Shadow | Fine eye, ear, neck and hand contours |
| `#965332` | Shadow | Fine hand creases |
| `#793D25` | Deep | Summer arm contour beside the sleeve |
| `#E8B896` | Rim blend | 60% target light shade, 40% `#F5F5F5` |
| `#F0C09D` | Rim highlight blend | 50% target light shade, 50% `#F5F5F5` |

Nine singleton color groups and 10,032 explicit seeds identify the selected
components in 32 hash-bound regions. Temporary connected-color studies helped
distinguish exposed skin from isolated clothing components. Only the recorded
seeds are used when applying the final recipe.

The gold glasses, white optical rim, iris colors, hair, backpack and clothing
remain original. The warm `#E8B896` and `#F0C09D` pixels blend skin into
the white glasses-rim curve and follow the target complexion. The dark glasses
arm at `[136,101]` and `[137,100]` stays original, while lower eye/nose skin at
`[160,98]`, `[163,101]` and `[166,98]` follows the target.

The same `#83462E` at `[143,96]` belongs to the iris in Neutral and to skin
beside the closed lid in Happy/Wink. Those expressions have separate masks.
True black eye contours remain untouched. The closed-mouth and speaking
lower-lip shadows recolor, while the actual red mouth interior and pink tongue
stay original. Actual Blue and Ryis mouths were inspected without adding
cosmetic palette changes.

Spring and Summer include the complete left hands and forearms. Autumn's
collar, cuffs and coat trim share several skin colors and are excluded. The
Summer `#793D25` contour at `[121,139]` lies on the skin side of the black cuff
border and maps. The nearby `#AB6946` patch at `[126,131]` belongs to the yellow
sleeve and stays original. Winter's gloves, coat and scarf remain unchanged.

## Presets and verification

The set orders Debug Blue, Hayden, Ryis and Seridia. Vanilla is implicit in
the installed controls. Shared integration assigns Luc to F11.

```sh
nix-shell --pure --run 'cargo test --locked --test luc_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/luc-portraits-study \
  --presets palettes/sets/luc-portraits-trial.json \
  --output generated/luc-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/luc.json \
  --output generated/luc-review-authored
```

Use fresh output directories. Current variants are in
`generated/luc-portraits-glasses-followup/variants`. Every target changes 77,656
pixels and preserves 14,008 other pixels sharing the nine source colors. None
of the target mappings is an identity. Before the glasses follow-up, the
76,888-pixel selection had digest
`b27cb1d20015e7958d16867e32dc59a5cbff7b82300cb4df5c6fc54a2ef40e75`.

The ignored corpus test checks all 32 strips for dimensions, alpha, unchanged
metadata, exact target colors and untouched unrelated colors. Its 111 literal
source landmarks cover all nine shades, glasses, irises, closed eyes, mouths,
forearms, sleeve boundaries and Autumn clothing. Coordinates were chosen from
source art independently of the component seeds.

`FOM_LUC_RECIPE` accepts temporary negative-control recipes. The catalog-only
control failed on the omitted lower-lip shade at `[158,109]`. Unrestricted
seven-color mapping failed on the glasses arm at `[136,101]`. Both were
observed before correcting the initial mapping and selection.

The complete first visual sweep found closed-eye omissions, Autumn clothing
spills and Spring/Summer forearm omissions caused by a temporary authoring
height cutoff. Additional source landmarks failed on the closed-eye fringe
before one consolidated correction. The final test passes, and its file is
formatted. The corrected mask uses complete connected skin regions for the
body, preserving separate cuff and collar components.

The author inspected all 64 actual full Source/Blue/Ryis trios, all 64 enlarged
face/neck trios and all seasonal hand/body closeups. The correction was checked
in the affected eye, collar, cuff and arm views. Its exact four-target delta
adds 44 closed-eye pixels per season. Autumn restores 3,584 clothing pixels.
Spring adds 4,432 skin pixels and restores 192 cuff pixels. Summer adds 7,840
skin pixels and restores 128 garment pixels. Winter's body is unchanged.

The independent reviewer completed the full first sweep and the bounded final
eye, mouth, collar, cuff and arm checks. All 128 final target strips pass its
exact audit with 37 independently chosen landmarks. Its delta check confirms
the season counts above, with every change strictly between source and target.
No concrete omission or spill remained before the final recipe freeze.

All four targets pass exact palette validation of the 32 PNGs and their
metadata. The authored gallery reuses all 64 frames with no conflicts.
Reconstructing the profile from those selections reproduces every actual Blue
pixel, matching both the authoring preview and all 64 frame occurrences.

Local evidence includes `tmp/luc-catalog-red.log`,
`tmp/luc-unrestricted-red.log`, `tmp/luc-consolidated-red.log`,
`tmp/luc-final-focused.log`, `tmp/luc-*-validation.json`,
`tmp/luc-correction-delta.json`, `tmp/luc-roundtrip-verification.json` and
`tmp/luc-maple-art-review-luc-verification.json`.

The initial authoring and independent-review evidence above predates the user's
glasses-rim correction. The initial classification kept two peach rim blends
unchanged, leaving visible traces of the original complexion. The focused test
first failed at `[142,99]` with the original peach instead of the pale Blue
blend. It now checks every occurrence of both rim shades across all 64 frames,
as well as unchanged white and gold details.

The correction changes exactly 12 rim pixels per frame: 768 pixels per target.
An exact four-target comparison confirms that every other pixel and all metadata
are unchanged. The pale blends retain the rim's brightness without a peach
fringe on Blue. Source/Blue/Ryis closeups were inspected, including closed eyes
and Ugh's expression marks. Evidence is `tmp/luc-glasses-red.log`,
`tmp/luc-glasses-final-focused.log`, `tmp/luc-glasses-delta.json` and
`tmp/luc-glasses-after.png`.

All four corrected targets passed palette validation. The refreshed gallery in
`generated/luc-review-glasses-followup` reuses all 64 frames; reconstructing its
selected profile validates against every corrected Blue strip. Evidence is
`tmp/luc-glasses-gallery-roundtrip-validation.json`.

Current profile SHA-256 is
`eddda44c420e7b1eab51622fd298b7a0ace2469e46e12f6061d73e471850e51a`.
The preset set SHA-256 is
`bd3127d683c7e1788c9d1a6bf8f8fa6c9a272550f8411dd6faab5259f6c5f30e`.
Shared integration owns the combined preview, full repository checks, bundle,
installation and live-game trial. Those actions were not run by the authoring
pass.
