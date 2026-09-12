# Wiscar portraits

The profile covers Wiscar's eight Spring cameo strips: embarrassed, happy, mad,
neutral, sad, think, ugh and wink. They live directly under
`assets/animations/NPCs/Wiscar/Portraits/`, use `PortraitsMisc`, and contain two
296×180 frames per 592×180 strip. All 16 frames are distinct. The profile stores
3,412 component seeds bound to original PNG hashes and dimensions.

## Palette and boundaries

| Source | Role or target rule | Debug Blue |
| --- | --- | --- |
| `#E3A17B` | Light | `#9DB9D4` |
| `#D98364` | Middle | `#7F9FBD` |
| `#C47054` | Shadow | `#6687AD` |
| `#9F5544` | Deep | `#445F83` |
| `#6D3328` | Two thirds of deep | `#2D3F57` |
| `#834132` | Four fifths of deep | `#364C69` |
| `#441B14` | Two fifths of deep | `#1B2634` |
| `#F4C2A7` | Three quarters light, one quarter white | `#B6CBDF` |
| `#DDAB82` | Light + `(-6,10,7)` | `#97C3DB` |

Fractions round each channel to the nearest integer. The two pale shades cover
small eye transitions; the three dark shades cover eye, ear, cheek, neck and
outer-mouth contours. No clipping or identity mappings occur in the four
presets: Debug Blue, Hayden, Ryis and Seridia. Vanilla is implicit; the combined
trial uses W for Wiscar.

Skin colors are also heavily reused by the gold costume. The mask preserves
the circlet's brown prong outlines, including `#6D3328` at `[154,52]` and
`[155,54]`, plus gold trim, clasps and earrings. The light skin visible between
the prongs belongs to the connected forehead component and recolors. Pink hair,
braid and beard, gray/purple eyes, blush, tears and mouth interior remain
original. The raised hand and neck recolor, as does the exposed lower hand at
`[181..188,171..179]`; adjacent robe and belt material stays original. These are
source-bound component decisions, with no runtime coordinate heuristic.

## Verification

Each preset selects and changes 13,825 pixels and preserves 11,104 other
occurrences of the configured colors. The selection mask SHA-256 is
`cce998afad538015d518ef28707655bbb37b19d731b9390cda3518ef97a6286b`, hashing each
profile-ordered path, a NUL delimiter and a selected/not-selected byte per pixel.

```sh
nix-shell --pure --run 'cargo test --locked --test wiscar_portraits -- --ignored --nocapture'
```

The local test passes 188 independently chosen literal skin/protected-detail
assertions, plus every-pixel source-or-target, alpha, dimension and unchanged
metadata checks. An unrestricted temporary recipe reproduced the costume spill
at circlet `[154,52]`; the final mask passes. `FOM_WISCAR_RECIPE` selects such a
temporary control without editing production data.

The author inspected all 16 full Source/Blue/Ryis frame trios, all 16 enlarged
faces, both hand views and six distinct mouth crops across all five choices.
All 32 target strips passed exact validation. Original PNG and metadata bytes
matched the ZIP, and rebuilding the gallery reused all 16 frames without
conflicts. Reconstructing its selected components reproduced every final Blue
strip and metadata file exactly. Independent art review found no further
visual issue in the corrected outputs.

Final variants are in `generated/wiscar-portraits-reviewed/variants`. Evidence is
ignored under `tmp/wiscar-wynne-author-*`; the combined audit is
`wiscar-wynne-author-final-verification.json`. Read-only source extraction used
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
No game assets were changed or committed. The user accepted the offline previews;
native rendering remains untested. Batch notes record combined installation checks.

Profile SHA-256: `f41f0eed693188c736d8fdd132f8d2640f7ec7cdf8facf41197a492eb73314f6`.
Preset-set SHA-256: `08f7e75ab26deaebd9624c841f4343d4b70c538387317a466a269fae948285c2`.
