# Taliferro portrait authoring

The profile covers all 36 main Taliferro portrait strips in the supplied archive:
nine expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, sly, think, ugh and wink. Each strip is
592×180 with two 296×180 frames and duration 0.2; all 72 frames are unique.
Overworld, UI and other characters' assets are outside this profile.

The paths are under
`assets/animations/NPCs/Taliferro/Portraits/{Season}/`. All 36 strips use
`PortraitsMisc`, including those stored in seasonal folders. The profile binds
25,025 component seeds to the original PNG hashes and dimensions. Source images,
generated images and temporary Rust authoring helpers remain ignored.

## Skin shades and boundaries

Eight separate color groups cover the four catalog shades, pale eye blends and
three outer-mouth colors. The separate groups let real hair retain shared deep
brown without taking adjacent skin shading with it.

| Source | Selected use | Debug Blue |
| --- | --- | --- |
| `#FFDBB8` | Main skin | `#9DB9D4` |
| `#F2A174` | Middle shading | `#7F9FBD` |
| `#C4613B` | Skin shadow and outer-mouth edges | `#6687AD` |
| `#762E21` | Deep eye, nose, ear, jaw and finger shading | `#445F83` |
| `#FCC49A` | Pale eye and skin transition | `#91AFCB` |
| `#DE7E48` | Closed lips and under-mouth shading | `#7393B5` |
| `#EFA67A` | Speaking-mouth outer transitions | `#7CA4C3` |
| `#FCDEBE` | Pale skin surrounding speaking mouths | `#9ABCDA` |

Taliferro's actual blond hair, eyebrows and sideburns retain their source colors.
The shared `#762E21` hair outline stays original, including the top hair pixel
`[163,33]`, sideburn component beginning `[137,59]`, and neutral eyebrow components
beginning `[145,54]`, `[149,55]`, `[150,56]` and `[162,58]`. The separate inner-ear
crease at `[134,60]`, skin below the ear at `[137,68]`, lower eyelid details and
nose contour follow the skin palette. These boundaries were inspected across all
expressions; a broad horizontal exclusion would split real hair outlines.

Both hands, including their deep finger creases and Summer's exposed forearm,
recolor. The gold earring, cravat clasp, buttons and clothing trim stay original.
The pink tongue, red mouth interior, eye whites and gray iris details also stay
original. The original expression blush remains; its contrast is a cosmetic
choice deferred to later polish.

User review was limited to the summary images and accepted that appearance,
with a minor concern about eyebrow edges, especially on the Ryis palette.
A fresh enlarged neutral Source/Blue/Ryis
comparison shows the retained warm `#AB672D` and `#762E21` brow colors contrasting
with the recolored surrounding skin. Eyebrow-edge blending remains a cosmetic
polish item; this feedback did not change the masks or presets. The local
comparison is `tmp/taliferro-brows-before.png`.

## Blends and presets

The four noncatalog shades adapt to the target palette. `#FCC49A` uses 60% of
the target's light shade and 40% of its middle shade. `#DE7E48` uses equal parts
middle and shadow. Both blends round each channel to the nearest integer.
`#EFA67A` adds its original offset `(-3,5,6)` to the target middle shade, and
`#FCDEBE` adds `(-3,3,6)` to the target light shade. These four presets require
no channel clipping.

| Source blend | Debug Blue | Hayden | Ryis | Seridia |
| --- | --- | --- | --- | --- |
| `#FCC49A` | `#91AFCB` | `#DCA465` | `#9D5E4B` | `#B6A398` |
| `#DE7E48` | `#7393B5` | `#BE814C` | `#723F32` | `#9A8279` |
| `#EFA67A` | `#7CA4C3` | `#C79558` | `#7E4F40` | `#A3958A` |
| `#FCDEBE` | `#9ABCDA` | `#E5B577` | `#AD6F5D` | `#BEB2AB` |

The preset set supplies Debug Blue, Hayden, Ryis and Seridia; Vanilla is implicit
in the installed controls. Taliferro uses F5 in the combined trial. The optional
local portrait preview switches outfits with Shift+F5.

## Verification

Every target selects and changes 128,504 pixels while preserving 8,116 other
occurrences of the eight source colors. No mapping is an identity mapping.
The four masks match: SHA-256
`edea24599d5c1191c0fd581e53a270b6ccced310fc2691f2350e041701cdb588`.
This audit hashes each profile-ordered asset path, a NUL delimiter, and one
changed-pixel byte per source pixel in row order.

```sh
nix-shell --pure --run 'cargo test --locked --test taliferro_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/taliferro-portraits-study \
  --presets palettes/sets/taliferro-portraits-trial.json \
  --output generated/taliferro-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/taliferro.json \
  --output generated/taliferro-review-gallery
```

Use fresh output directories. During this slice the game mount was empty, so
read-only extraction used the exact original backup
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The archive and mounted game directory were not modified.

`FOM_TALIFERRO_RECIPE` lets the focused corpus test exercise temporary omission
and spill controls. The catalog-only recipe failed on the missing `#FCC49A` eye
blend at neutral `[165,52]`. Unrestricted mapping failed on the real hair outline
at `[163,33]`. The final test passes 52 literal skin/blend checks and 48 protected
detail checks, alongside every-pixel source-or-target, alpha, dimensions and
unchanged-metadata assertions.

All 144 target strips passed palette validation and source hash checks. The
author and independent reviewer each inspected all 72 actual full Source/Blue/Ryis
frame trios, all 72 enlarged face trios, and all four seasonal body/hand views.
The author also inspected 27 distinct mouth-region crops with all four targets;
the reviewer used a different crop covering 32 distinct mouth regions. The
independent exact audit passed all 15,344,640 target pixels and 37 separately
chosen source/protection landmarks. No concrete correction remained.

The authored gallery reused all 72 frames, with no conflicts or pending review.
Reconstructing its selections and applying them reproduced all 36 Blue strips
and their metadata exactly. The reviewed output is
`generated/taliferro-portraits-reviewed/variants`; all 144 PNG hashes match the
independently approved candidate. Detailed evidence remains ignored under
`tmp/taliferro-*` and `tmp/stillwell-taliferro-art-review-*`.

In-game rendering was not run for this slice because the game executable mount
was unavailable. Combined package and installation checks are recorded in the
[batch notes](stillwell-taliferro-portraits.md).

Final profile SHA-256:
`f07f39b8ec3a895ff65f04ca01571b89feda1faa531844cc138b3e70f5cf639f`.
Final preset-set SHA-256:
`c6a8f2ada01a42ebf021131f214430f3a693640fc00ddfac8abc214286dee730`.
