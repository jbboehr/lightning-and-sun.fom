# Maple portrait authoring

The initial pass is recorded below. The [lip follow-up](#lip-follow-up) records
the later user-requested correction and the current recipe hashes and counts.

The profile covers all 32 main Maple portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Each strip has
two 296×180 frames, and all 64 frames are unique. This corpus has no main beach,
bathing or wedding portraits. Overworld and UI assets are outside it.

Source PNGs and metadata are under
`assets/animations/NPCs/Maple/Portraits/{Season}/`. The eight strips in each
season use the corresponding PortraitsSpring, PortraitsSummer, PortraitsAutumn
or PortraitsWinter atlas. The profile records 12,968 component seeds, original
PNG hashes and dimensions. All source images, generated images and temporary
Rust authoring tools remain ignored.

## Initial skin colors and boundaries

The catalog's four samples omit five shades used on the face, ears, hands and
neck. Each preset uses the same explicit component selection and shade roles.

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#97623E` | Main skin | `#9DB9D4` |
| `#89492A` | Middle skin shading | `#7F9FBD` |
| `#713527` | Skin shadow | `#6687AD` |
| `#57271D` | Deep skin and outer-mouth shading | `#445F83` |
| `#83462E` | Fine face, ear, jaw, neck and arm shading | `#7F9FBD` |
| `#5D3228` | Skin-colored eyebrow fringes | `#445F83` |
| `#CE794E` | Fingertip highlights | `#9DB9D4` |
| `#89502A` | Happy/Wink speaking-frame nose shading | `#7F9FBD` |
| `#392222` | Local deep ear, nose, cheek and jaw contours | `#445F83` |

Eight connected-color groups separate the main shades and shared details.
`#83462E` and `#CE794E` share one group, allowing connected iris components to
stay original while the same shades on fingers and exposed skin recolor.
The isolated iris-edge components are also protected. Actual brown iris pixels,
black pupils, white sclera and gray highlights remain unchanged.

The true `#392222` brow and head-hair cores stay original. The lighter
`#5D3228` brow fringes follow the skin palette. Deep `#392222` nose and inner-ear
contours recolor, as does the anatomical diagonal between chin and neck. The
tiny cheek and jaw pixels at `[140,103]` and `[143,107]` lie inside the black
face boundary and follow that rule. By contrast, `[163,106]` is outside the
right-jaw separator in hanging hair and remains original.

The Summer outfit shares `#97623E` with exposed skin. Small collar/bodice
components outside the black neck boundary stay original, including
`[141,114]`, `[156..157,115]`, `[139,116]` and `[160,117]`. The seven `#57271D`
pixels at the left orange ruffle, around x125..127 and y117..123, also retain
their source color. The adjacent bare arm recolors. The narrow Winter skin gap
between its cream collar lobes remains selected. Bows, beads, sleeves, cuffs,
bracelet, dress details and hair retain their original colors.

Closed-mouth shading and the speaking lower outer arc follow skin. The actual
speaking interior border, pink tongue and pink lips stay original. In neutral's
speaking frame, `#57271D` at `[152,101]`, `[151,102]` and `[154,102]` bounds the
interior, while `[152,104]` and `[153,106]` are outer-mouth shading. The lighter
outer edge at `[155,103]` also recolors. This distinction was checked in Blue
and Ryis across all expressions; no cosmetic palette adjustment was needed.
Blush and Ugh's existing gray expression marks likewise stay original.

## Initial presets and verification

`palettes/sets/maple-portraits-trial.json` provides Debug Blue, Hayden, Ryis and
Seridia, using each natural preset's catalog shades through the roles above.
Vanilla is implicit in the installed controls. Integration assigns Maple to F12.
Every target selects and changes 83,672 pixels while preserving 45,768 other
occurrences of the nine source colors. None of these target mappings is an
identity mapping.

```sh
nix-shell --pure --run 'cargo test --locked --test maple_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/maple-portraits-study \
  --presets palettes/sets/maple-portraits-trial.json \
  --output generated/maple-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/maple.json \
  --output generated/maple-review-gallery
```

Use fresh output directories. `FOM_MAPLE_RECIPE` lets the opt-in source-corpus
test run against a temporary omission or spill control. The catalog-only control
failed on Happy's omitted `#89502A` nose shade at frame-one `[157,100]`.
Unrestricted mapping failed on the actual `#392222` brow at `[145,86]`.
The cheek/jaw correction first failed on literal source coordinate `[140,103]`.
All failures were observed before their corresponding corrections.

The final test passes 182 literal source-art landmarks: 108 skin/outer-mouth
checks and 74 protected checks. It covers every source shade, all seasonal
atlases, every strip's dimensions and unchanged metadata, alpha, exact mapped
colors and untouched unrelated pixels. Expected landmarks come from source
art, independently of the profile's seeds.

All four targets passed validation of 32 PNGs and their metadata. The author
inspected all 64 actual full-frame Source/Blue/Ryis trios, all 64 enlarged
face/ear/mouth trios and four distinct seasonal arm/hand trios. Independent
review completed the same full corpus before the consolidated correction.
The only remaining finding was the two cheek/jaw pixels; the correction adds
exactly 128 pixels per target across all 32 strips, with every other pixel
unchanged. Final affected Blue/Ryis images were inspected afterward.

Independent exact review passes all 128 final target strips with 29 separately
chosen landmarks per target, matching source hashes, metadata, alpha and the
shared selected mask. Its mask SHA-256 is
`834087d4587651f8ab4532ad894e1b74bd36a39865e5750a3d5c7183fea00358`.
Evidence is `tmp/luc-maple-art-review-maple-verification.json` and
`tmp/luc-maple-art-review-maple-delta.json`.

The final output is `generated/maple-portraits-reviewed/variants/`. The authored
gallery `generated/maple-review-final/` reuses all 64 frames without pending or
conflicting selections. Reconstructing a profile from those gallery selections
reproduces all 64 applied Blue frames exactly.

Local evidence includes `tmp/maple-catalog-red.log`,
`tmp/maple-unrestricted-red.log`, `tmp/maple-jaw-omission-red.log`,
`tmp/maple-final-test.log`, `tmp/maple-*-validation.json`,
`tmp/maple-final-delta.json`, `tmp/maple-final-roundtrip-verification.json`,
`tmp/maple-final-faces-*.png`, `tmp/maple-final-hands-0.png` and
`tmp/maple-final-inputs.sha256`.

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The final profile SHA-256 is
`6e0e3c670ad868f066a19f0e0840a0cc5c5233ec8a2f44438c54a3b6423f9bfa`.
Integration owns the combined preview, full repository checks, installation and
live-game testing. This authoring pass neither edits the source game nor stages
any game-derived images.

## Lip follow-up

The user found the lips off in the completed preview. Enlarged original and
Blue/Ryis comparisons showed the fixed `#BF5C70` pink highlight standing out as
a saturated stripe against the recolored skin. The surrounding skin selection
and the tongue/interior placement were intact. This correction gives that one
lip shade a muted tone suited to each preset:

| Preset | Lip color for `#BF5C70` |
| --- | --- |
| Debug Blue | `#947FA7` |
| Hayden | `#BC7860` |
| Ryis | `#97535B` |
| Seridia | `#A17B88` |

The blue choice uses a muted mauve; the natural presets retain a restrained
rose or peach tint with less contrast against their skin shades. These are
hand-authored cosmetic colors, independent of the four catalog skin roles.
The source lip shape and all pink tongue/interior colors stay unchanged.

All 160 occurrences of `#BF5C70` are within the lips, at frame-local
x152..154 and y104..107. Adding one component per frame selects those pixels
without changing any earlier seed or color mapping. The current profile has
ten source colors, nine component groups and 13,032 seeds. Every preset now
selects and changes 83,832 pixels and protects 45,768 matching source pixels.

The new regression builds the actual four-preset set and checks every lip
pixel against literal per-preset colors, along with all 112 tongue/interior
pixels and explicit unchanged upper-mouth boundary landmarks. It first failed
on the unchanged highlight at Autumn embarrassed `[153,104]`, then passed.
The existing corpus test keeps its skin, hair, iris, clothing and mouth
checks; four previous fixed-lip expectations now check the new lip color.
Its 182 landmarks comprise 112 selected skin/lip points and 70 protected points.
Both focused tests pass after formatting.

All 18 distinct source mouth crops were inspected in Source, Blue, Ryis,
Hayden and Seridia, including closed and speaking poses. The compact actual
before/after face comparison is `tmp/maple-mouth-before-after.png`; the complete
mouth comparisons are `tmp/maple-mouth-before-*.png` and
`tmp/maple-mouth-after-*.png`. The muted lip tones reduce the conspicuous stripe
while preserving the shape and interior detail.

An exact comparison with `generated/maple-portraits-reviewed/variants` confirms
that each target changes only the 160 lip pixels. Every other pixel, metadata
file and original component seed remains unchanged. All four targets pass exact
palette validation. The updated authored gallery reuses all 64 frames, and a
profile reconstructed from its selections reproduces every applied Blue pixel
and metadata file.

The current output is `generated/maple-portraits-mouth-followup/variants/`;
its gallery is `generated/maple-mouth-review/`. The initial authoring review
and its earlier hashes above predate this correction. No further review agent,
full repository suite, installation or game launch was run by this author for
the bounded follow-up; integration owns those shared checks and the new trial.

Evidence includes `tmp/maple-mouth-lip-red.log`,
`tmp/maple-mouth-final-check.log`, `tmp/maple-mouth-profile-audit.json`,
`tmp/maple-mouth-delta.json`, `tmp/maple-mouth-*-validation.json`,
`tmp/maple-mouth-gallery-check.json`, and `tmp/maple-mouth-inputs.sha256`.
Current profile SHA-256:
`43eef21e929410213c78afd35c52ce7a17501f560551557039d34df08e9a5869`.
Current preset-set SHA-256:
`ed63400ee21fb7c75d78d0b1c06c54b8c84c9d170507ef1f55e371d11bf33521`.
