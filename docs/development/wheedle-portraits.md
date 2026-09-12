# Wheedle portrait authoring

The profile covers all 32 main Wheedle portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Each strip is
592×180 with two 296×180 frames. There are 64 frame occurrences and 62 distinct
frames: Autumn's wink strip duplicates Autumn's neutral strip in both frames.
Each season uses its corresponding `PortraitsSpring`, `PortraitsSummer`,
`PortraitsAutumn` or `PortraitsWinter` atlas. Overworld and UI assets are outside
this profile.

The paths are under
`assets/animations/NPCs/Wheedle/Portraits/{Season}/`. The profile binds 19,300
component seeds to the original PNG hashes and dimensions. Source images,
generated images and temporary Rust authoring helpers remain ignored.

## Skin shades and boundaries

Ten separate color groups cover the main skin ramp, deep contours, eye blends,
mouth edges and hands. The catalog's fourth sample, `#A56C54`, is close to the
shadow shade; the darker `#81503C` provides the deep role for this profile.

| Source | Selected use | Debug Blue |
| --- | --- | --- |
| `#F6C38D` | Main skin and exposed wrists | `#9DB9D4` |
| `#D39876` | Middle shading | `#7F9FBD` |
| `#AF7456` | Skin shadow | `#6687AD` |
| `#A56C54` | Slightly darker shadow and lip edges | `#5C7FAB` |
| `#81503C` | Deep ear, eye and outer-mouth shading | `#445F83` |
| `#553728` | Dark jaw, nose and bare-finger contours | `#2D3F57` |
| `#D89D7A` | Lighter middle shading around eyes, mouth and hands | `#84A4C1` |
| `#E5AC7E` | Pale eye transition | `#8EACC9` |
| `#B57657` | Fine eyelid edge | `#6C89AE` |
| `#7C5846` | Lower eyelid transition | `#3F678D` |

Actual brown hair and goatee remain original, including `#9A6D58`, `#553E33`,
`#795645` and `#604639`. The source uses slightly different goatee shading by
season; this distinction is retained. Brown skin transitions adjacent to the
hair, eyebrows and beard recolor. Eye whites, gray eye details, black cores,
pink tongue, red mouth interior, tears, ring and seasonal clothing retain their
source colors.

Spring, Summer and Autumn have bare hands. Winter has golden gloves and two
small exposed wrist patches. The first candidate incorrectly recolored the
shared `#553728` glove outlines. The correction excludes 57 components totaling
78 pixels in every Winter frame, including the glove fingertips, hems,
brown cuff details and three gold-necktie shadows. Examples are `[184,86]`,
`[183,87]`, `[175,97]`, `[175,116]`, the lower cuff at `[162,125]` and tie
shading at `[152,94]`, `[153,94]` and `[150,95]`.
Exposed wrist skin at `[184,116]`, `[168,119]`
and `[185,116]` still recolors. The same dark shade follows bare fingers in the
other seasons. These decisions are stored as source-bound component seeds;
there is no runtime coordinate heuristic.

## Blends and presets

The four presets are Debug Blue, Hayden, Ryis and Seridia. Vanilla is implicit
in the installed controls. Wheedle uses T in the combined trial.

The additional shades follow the target ramp with these channel-wise rules:

| Source | Target rule |
| --- | --- |
| `#A56C54` | Shadow + `(-10,-8,-2)` |
| `#553728` | Two thirds of deep, rounded to nearest integer |
| `#D89D7A` | Middle + `(5,5,4)` |
| `#E5AC7E` | Equal blend of light and middle, rounded to nearest integer |
| `#B57657` | Shadow + `(6,2,1)` |
| `#7C5846` | Deep + `(-5,8,10)` |

No channel clipping or identity mappings occur in these four presets. The main
light, middle, shadow and deep shades use their target ramp values directly.

## Verification

Every target selects and changes 78,108 pixels while preserving 1,248 other
occurrences of the configured colors, all Winter glove, cuff and tie details. The four
masks match: SHA-256
`f5f5be873146de190aad5b431f714d927ef1ab127cd746ceecc4813cd2ccdf6c`.
This audit hashes each profile-ordered asset path, a NUL delimiter, and one
changed-pixel byte per source pixel in row order.

```sh
nix-shell --pure --run 'cargo test --locked --test wheedle_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/wheedle-portraits-study \
  --presets palettes/sets/wheedle-portraits-trial.json \
  --output generated/wheedle-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/wheedle.json \
  --output generated/wheedle-review-gallery
```

Use fresh output directories. During this slice the game mount was empty, so
read-only extraction used the exact original backup
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The archive and mounted game directory were not modified.

The focused corpus test passes 472 literal skin checks and 128 protected-detail
checks, alongside every-pixel source-or-target, alpha, dimension, seasonal atlas
and unchanged-metadata assertions. Its glove regression failed at Winter
`[184,86]` before the mask correction, then passed. `FOM_WHEEDLE_RECIPE` permits
temporary controls: removing the `#E5AC7E` eye component from the mask failed the
skin assertion at neutral `[139,60]`. The final recipe passes again.

The author inspected all 62 distinct full Source/Blue/Ryis frame trios and 62
enlarged face trios, all four seasonal hand views in all five palette choices,
and 20 distinct mouth/goatee crops in all five choices. The final hand views
were inspected again after the glove correction. Exact validation passed all
128 generated target strips; all source PNG hashes matched the profile. The
rebuilt gallery reused all 64 frame occurrences with no conflicts or pending
review. Reconstructing its selections and applying them reproduced every Blue
strip and its metadata exactly.

The reviewed output is `generated/wheedle-portraits-reviewed/variants`.
Evidence remains ignored under `tmp/wheedle-*`, including
`wheedle-reviewed-verification.json`, `wheedle-glove-red.log`,
`wheedle-eye-omission-red.log` and `wheedle-final-test.log`.
Independent review and combined integration are recorded in the
[batch notes](vera-wheedle-portraits.md). The user accepted the offline previews.
In-game rendering remains untested; generated images cannot establish runtime behavior.

Final profile SHA-256:
`c0040e873c5025217520726066e9805ec2cefabf160bde6abde2df2c3cebf829`.
Final preset-set SHA-256:
`2a068396aa96d53072703c562d61da7e1539286652014357f422c1c9ca359aa0`.
