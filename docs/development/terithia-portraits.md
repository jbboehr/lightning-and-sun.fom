# Terithia portrait authoring

The profile covers all 32 main Terithia portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Every strip is
592×180 with two 296×180 frames; all 64 frames are unique. No main beach, bathing
or wedding strips occur in this corpus. Overworld and UI assets are outside it.

The source paths are under
`assets/animations/NPCs/Terithia/Portraits/{Season}/`. Each season's eight strips
use its corresponding PortraitsSpring, PortraitsSummer, PortraitsAutumn or
PortraitsWinter atlas. The profile binds 28,032 component seeds to the original
PNG hashes and dimensions. Source images, generated images and temporary Rust
authoring helpers remain ignored.

## Skin shades and boundaries

The catalog supplies four skin shades. Seven additional colors cover fine
shadows, the pale scars and one seasonal neck blend. Eleven separate color
groups allow shared colors on accessories to retain their original appearance.

| Source | Selected use | Debug Blue |
| --- | --- | --- |
| `#D29A7D` | Main skin | `#9DB9D4` |
| `#BA7E66` | Middle shading | `#7F9FBD` |
| `#AD7465` | Skin shadow and edge blends | `#6687AD` |
| `#7E4C3B` | Deep skin, jaw and outer-mouth shading | `#445F83` |
| `#724A3E` | Fine eye and nose shadows | `#445F83` |
| `#583131` | Ear, underjaw and hand creases | `#445F83` |
| `#482323` | Brown eye/brow fringes and finger creases | `#445F83` |
| `#DBA191` | Pale scar shading | `#A6C0E8` |
| `#E0A797` | Scar transitions | `#ABC6EE` |
| `#E8B1A2` | Scar highlights | `#B3D0F9` |
| `#7A4D45` | Autumn neck-edge blend | `#445F83` |

The true black eye/brow cores, gray hair, sclera, blush and pink tongue remain
original. Skin-colored outer-mouth shading follows the palette. All six distinct
mouth crops were inspected with all four targets; there was no separate pale
lip color needing a cosmetic adjustment.

The dark neck cord shares `#7E4C3B` and `#583131` with skin. Its narrow components
and the pendant outlines stay original. Nearby underjaw skin at `[151,93]`,
`[154,93]` and `[154,94]` recolors, as does the Spring exposed chest boundary
at `[141,104]`, `[144,105]` and `[149,106]`. These are distinct from the cord,
not a broad exclusion across the neck.

Summer's brown blouse laces stay original, including `#583131` at `[164,126]`
and `[164,127]`. The exposed gaps between and below the laces recolor, including
`#AD7465` at `[166,128]` and `[168,129]`, and `#D29A7D` at `[167,130]`.
The Autumn `#7A4D45` neck pixel `[140,96]` follows skin, while the same color
in the towel at `[124,154]` stays original.

Actual hand creases recolor, including `#482323` at `[140,172]` and `[185,158]`.
The bag's dark fill, edging and strap, the waist belt, and clothing stay original.
In particular, the `#452424` bag edge at `[125,175]` is outside the hand's black
boundary and is not a skin shade.

## Scars and presets

The pale face and arm scars retain their original shapes and relative lightness.
Their colors use the original RGB offsets from the main skin shade `#D29A7D`,
added to each target's main skin color: `(9,7,20)`, `(14,13,26)` and `(22,23,37)`.
No channel clipping is needed for these four presets.

| Source scar | Debug Blue | Hayden | Ryis | Seridia |
| --- | --- | --- | --- | --- |
| `#DBA191` | `#A6C0E8` | `#F1B985` | `#B9736B` | `#CAB6B9` |
| `#E0A797` | `#ABC6EE` | `#F6BF8B` | `#BE7971` | `#CFBCBF` |
| `#E8B1A2` | `#B3D0F9` | `#FEC996` | `#C6837C` | `#D7C6CA` |

The preset set supplies Debug Blue, Hayden, Ryis and Seridia; Vanilla is implicit
in the installed controls. Terithia uses F3 in the combined trial. The optional
local world action uses Shift+F3.

## Verification

Every target selects and changes 197,904 pixels while preserving 31,088 other
occurrences of the eleven source colors. No mapping is an identity mapping.
All four targets have the same selection mask, SHA-256
`239059967a8bfe5f5a8627f4c261dd6256e6bbaaa41066f5c58e15ce4618c9eb`.

```sh
nix-shell --pure --run 'cargo test --locked --test terithia_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/terithia-portraits-study \
  --presets palettes/sets/terithia-portraits-trial.json \
  --output generated/terithia-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/terithia.json \
  --output generated/terithia-review-gallery
```

Use fresh output directories. `FOM_TERITHIA_RECIPE` allows the focused corpus
test to exercise temporary omission and spill controls. The catalog-only control
failed on the omitted `#724A3E` skin pixel at neutral `[153,68]`. Unrestricted
mapping failed on the cord's `#7E4C3B` at `[150,99]`. The final test passes
148 literal skin/scar checks and 64 protected-detail checks, plus every-pixel
source-or-target, alpha, dimensions and unchanged-metadata assertions.

All 128 generated target strips passed palette validation and source hash
checks. The author and independent reviewer each inspected all 64 actual full
Source/Blue/Ryis frame trios, all 64 enlarged face trios and all four seasonal
hand/body views. The independent exact audit also passed 35 separately chosen
source/protection landmarks. No further concrete correction remained after
that complete pass.

The authored gallery reused all 64 frames with no conflicts or pending review.
Reconstructing its selections and applying them reproduced all 32 Blue strips
and metadata exactly. The reviewed output is
`generated/terithia-portraits-reviewed/variants`; all 128 PNG hashes match the
independently approved candidate. Detailed evidence is ignored under
`tmp/terithia-*` and `tmp/merri-terithia-art-review-*`.

Final profile SHA-256:
`026f985e4ded515fd3345c96e634a8636d775811755d4eb78646ff7f9f1b38db`.
Final preset-set SHA-256:
`9614bcf2fdb1ae2fbaa279fc9fac0e8c2c5ab6f49d9937c1385245b1dd767c59`.
