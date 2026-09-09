# Hemlock portraits

The profile covers all 32 main Hemlock portrait strips in the supplied archive.
Spring, Summer, Autumn and Winter each have eight expressions: embarrassed,
happy, mad, neutral, sad, think, ugh and wink. Each 592×180 strip contains two
296×180 frames, with duration 0.2. The 64 frames are all unique. Metadata uses
the corresponding `PortraitsSpring`, `PortraitsSummer`, `PortraitsAutumn` or
`PortraitsWinter` atlas, with eight strips in each.

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local source files are in `extracted/hemlock-portraits-study`. Game images,
generated variants, galleries and temporary authoring helpers stay ignored.
Only recipes, explicit source-bound component seeds, this note and the focused
test belong in the repository.

## Skin and protected details

The four catalog colors are supplemented by three fine shades found in the
source frames. Every target uses the same shade roles and selected components.

| Source | Role | Use |
| --- | --- | --- |
| `#C9906A` | Light | Main skin and tiny isolated hand/chest pixels |
| `#B36844` | Medium | Face, neck, arm and finger shading |
| `#8A3F31` | Shadow | Skin contours, also shared with Spring neckwear and apron |
| `#712922` | Deep | Deep skin contours and outer lip shading |
| `#A05748` | Shadow | Fine eye and nose shading |
| `#BF7B5A` | Medium | Neck, chest, elbow and upper-arm shading |
| `#4C242C` | Deep | Deep nose and under-jaw contours, also shared with eyes and clothes |

The stored profile has seven singleton color groups and 22,616 explicit seeds
across the 32 hash-bound regions. Temporary connected-color studies helped
identify exposed skin. The final recipe stores the chosen components and does
not run the authoring heuristics during installation.

The gold hoop earring, gray scalp hair, stubble, chest hair and arm hair retain
their original colors. In particular, `#9A7D72` forms complete muted gray hair
strokes. The separate warm `#BF7B5A` neck and arm blends follow the skin palette.
Both the source topology and actual Ryis output were inspected to distinguish
these strokes from skin-colored infill around them.

User review flagged the beard and arm hair as looking wrong against Debug Blue,
while the natural presets were considered passable. A focused comparison of all
32 strips across all four presets confirmed that both `#9A7D72` and `#7F7676`
remain unchanged. Their warm tint stands out against blue skin. Adapting that
tint remains a cosmetic follow-up; the current recipes preserve the original
hair colors. Local evidence is `tmp/hemlock-hair-followup.json`.

Spring necktie and apron components sharing catalog colors stay original.
The neck-side `#8A3F31` at `[164,85]` maps, while the tie edge at `[149,85]`
and apron tie at `[170,156]` stay. Autumn's hanging fingers and the small
left-hand skin island at `[126..127,163]` also map despite being disconnected
from the main skin region.

The deep nose crease and brown skin around the eyes recolor. Actual eye details
at `[164,60]` and `[177,63]` in Spring neutral remain original. The closed-mouth
shadow and speaking lower lip follow the target. Black mouth corners, red
`#A1121D` interior and pink `#DF4868`/`#F96F8C` tongue remain unchanged. Actual
Blue and Ryis closed and speaking mouths were inspected. No separate cosmetic
palette adjustment was needed.

## Verification

The preset set orders Debug Blue, Hayden, Ryis and Seridia. Vanilla is implicit
in the installed controls, and shared integration assigns Hemlock to F.

```sh
nix-shell --pure --run 'cargo test --locked --test hemlock_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/hemlock-portraits-study \
  --presets palettes/sets/hemlock-portraits-trial.json \
  --output generated/hemlock-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/hemlock.json \
  --output generated/hemlock-review-authored
```

Use fresh output directories. The reviewed variants are in
`generated/hemlock-portraits-reviewed/variants`. All four targets passed exact
palette validation for 32 PNGs and their metadata. Each changes 118,560 pixels
and preserves 10,480 other pixels that share mapped source colors. None of the
seven source colors equals its target, so changed and selected counts agree.
The identical selection digest across all four targets is
`a17737a9f82135d95e211a6dacd3062b466565de992b272e5b346fbfc7d2c987`.

The ignored corpus test checks every strip for dimensions, unchanged metadata,
alpha, exact mapped colors and untouched unrelated colors. Its 37 literal
source landmarks cover all seven shades, eye and mouth details, isolated
fingers, hair, earring, necktie, apron and Winter collar. These coordinates were
chosen from source art independently of the mask seeds.

`FOM_HEMLOCK_RECIPE` accepts a temporary recipe for negative controls. The
catalog-only recipe failed on omitted `#A05748` at `[173,61]`. An unrestricted
seven-color recipe then failed on the Spring tie at `[149,85]`. Both failures
were observed before correcting the mapping and selection. The final test
passes and its file is formatted.

The author and independent reviewer each inspected all 64 actual full-frame
Source/Blue/Ryis trios, all 64 enlarged face/neck trios, and seasonal hands,
arms and mouth closeups. The independent exact audit passed all 128 target
strips with 27 separately chosen landmarks. The full visual sweep found no
remaining concrete omission or spill before the recipes were frozen.

The authored gallery reuses all 64 frames without conflicts. Reconstructing
the profile from its selections and applying it reproduces every actual Blue
pixel. Both the authoring preview and the reconstructed output match all 64
frame occurrences.

Local evidence includes `tmp/hemlock-catalog-red.log`,
`tmp/hemlock-unrestricted-red.log`, `tmp/hemlock-final-focused.log`,
`tmp/hemlock-*-validation.json`, `tmp/hemlock-roundtrip-verification.json`,
`tmp/hemlock-actual-{full,faces,hands}-*.png` and
`tmp/hemlock-louis-art-review-hemlock-verification.json`.

Final profile SHA-256 is
`e8666ad2e6fb95a4faff56b32d46eec4c0f7f2b5746818ba7c6ede99ddd3576c`.
The preset set SHA-256 is
`5de97c1aa95b78ef9154b4fd07b9a30a50fb562add5daa87087b1f5a3daab1a7`.
Shared integration owns the combined preview, full repository checks, bundle,
installation and live-game trial. Those actions were not run by the authoring
pass.
