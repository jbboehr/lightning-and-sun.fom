# Landen portrait authoring

This pass covers all 32 main Landen portrait PNG strips in the supplied archive:
eight expressions each for Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Each strip has two
296×180 frames, giving 64 unique frames. There are no Beach, bath or Wedding
portraits under this character's main portrait prefix in this archive. World
sprites and UI art remain outside this profile.

The profile binds every strip to its original SHA-256 and 592×180 dimensions.
It stores 29,556 explicit component seeds in seven separate connected-color
groups. Temporary authoring helpers only select components; no classifier or
new runtime machinery ships with these recipes.

## Palette and boundaries

The four catalog colors omit two lighter palm shades and the deepest exposed
arm contour. Seven source colors cover the inspected face, ears, neck, chest,
arms and hands:

| Source | Role | Debug Blue |
| --- | --- | --- |
| `#9F6250` | Main skin light | `#9DB9D4` |
| `#854D3C` | Medium skin shading | `#7F9FBD` |
| `#63342A` | Skin shadow and lip edge | `#6687AD` |
| `#491F1B` | Deep skin shading | `#445F83` |
| `#CF9887` | Lighter palm | `#9DB9D4` |
| `#C1816E` | Palm crease shading | `#7F9FBD` |
| `#361714` | Deep arm contour | `#445F83` |

Brown fringes and creases around the black eyebrows and eyes follow the skin
palette, as do lighter lip edges and lower-lip skin shading. The actual deepest
closed-mouth line and speaking-mouth contours remain original. Gray hair,
sideburns and moustache use separate source colors and remain original, along
with black outlines, white eye details, blush, tears and tongue colors.

The Summer shirt's two buttons share `#CF9887` with the palm. Their four pixels
per frame (eight per strip), at x169–170, y118–119 and y126–127, are excluded. Exposed neck gaps
between the seasonal collars remain included; shirts, scarf, cuffs and other
clothing stay original.

The set provides Debug Blue, Hayden, Ryis and Seridia, with Vanilla supplied by
the runtime. All targets use the same selected regions and shade roles. Ryis
already contains Landen's `#63342A` and `#491F1B`, so those selected pixels are
valid no-op transformations in that preset. Actual changed-pixel masks therefore
differ: Blue, Hayden and Seridia each change 164,968 pixels; Ryis changes 147,464.

## Reproduce the local outputs

After exporting the 32 profile assets into `extracted/landen-portraits-study`,
build with a fresh destination:

```sh
target/release/mistria-palette build-presets \
  --original extracted/landen-portraits-study \
  --presets palettes/sets/landen-portraits-trial.json \
  --output generated/landen-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/landen.json \
  --output generated/landen-review-authored
nix-shell --pure --run 'cargo test --locked --test landen_portraits -- --ignored'
```

Six-pose local summaries are `generated/landen-summary.html` and
`generated/landen-summary-ryis.html`. Full source/Blue/Ryis sheets and enlarged
actual-output faces are ignored files under `tmp/landen-*`. The user reviews
completed recolors rather than individual component selections.

## Verification evidence

The focused corpus test checks all 32 strips, metadata, dimensions, alpha,
unmapped colors and exact target colors, plus 28 literal source-art landmarks.
Those landmarks independently cover lighter palms, eye/brow fringes, lip skin,
deep arm shading, gray facial hair, black and white eye details, actual mouth
contours, tongue and shared-color shirt buttons. `FOM_LANDEN_RECIPE` can point to
a temporary candidate recipe for mutation checks.

Three observed failures establish useful negative controls:

- A catalog-only mapping misses the lighter palm at `[224,97]`.
- An unrestricted seven-color mapping recolors the true mouth line at `[161,75]`.
- Adding one button component to the authored profile recolors the Summer shirt
  button at `[169,118]`.

The final focused test passes. All four targets pass exact recipe validation
across all 128 generated strips. The authored gallery reuses all 64 unique frames;
reconstructing its selected seeds reproduces every applied pixel. All 64 full
frames and all 64 enlarged actual source/Blue/Ryis faces were inspected, including
seasonal neckline differences and both closed and speaking mouths.

Local evidence includes `tmp/landen-catalog-red.log`,
`tmp/landen-unmasked-red.log`, `tmp/landen-button-spill-red.log`,
`tmp/landen-test-green.log`, `tmp/landen-frame-verification.json` and
`tmp/landen-{preset}-validation.json`.

The independent audit also passes all 128 actual target strips and 47 literal
landmarks per target, checking source hashes, exact corpus, every pixel,
transparent RGB and metadata. It records 164,968 selected pixels and 672
protected pixels that share mapped colors. Ryis has 17,504 selected no-op pixels.
All four targets share the selected-mask digest
`782aabfea24253dff4a6368438e401fa1e3e1e349783b5fa3e6e6b6b13d0c3ea`.
Evidence is `tmp/landen-nora-art-review-landen-verification.json`.

This authoring pass does not install or launch the game. Combined package,
installation, full repository checks and live-game verification belong to the
integration pass. Only declarative palette/profile data, this development note
and the focused test are intended for Git; game images and generated files stay
ignored.
