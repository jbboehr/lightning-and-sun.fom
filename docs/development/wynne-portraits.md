# Wynne portraits

The profile covers Wynne's eight Spring cameo strips: embarrassed, happy, mad,
neutral, sad, think, ugh and wink. They live directly under
`assets/animations/NPCs/Wynne/Portraits/`, use `PortraitsMisc`, and contain two
296×180 frames per 592×180 strip. All 16 frames are distinct. The independently
authored profile stores 6,059 component seeds bound to original PNG hashes and
dimensions; sharing a catalog ramp with Darren did not substitute for inspecting
Wynne's artwork.

## Palette and boundaries

| Source | Role or target rule | Debug Blue |
| --- | --- | --- |
| `#A3614E` | Light | `#9DB9D4` |
| `#83453A` | Middle | `#7F9FBD` |
| `#63342A` | Shadow | `#6687AD` |
| `#491F1B` | Deep | `#445F83` |
| `#955343` | Middle + `(18,14,9)` | `#91ADC6` |
| `#AD6C59` | Light + `(10,11,11)` | `#A7C4DF` |
| `#B67565` | Light + `(19,20,23)` | `#B0CDEB` |
| `#C88A76` | Light + `(37,41,40)` | `#C2E2FC` |
| `#612924` | Shadow + `(-2,-11,-6)` | `#647CA7` |
| `#804136` | Middle + `(-3,-4,-4)` | `#7C9BB9` |

The additional middle shade covers fine facial shading; three pale shades
preserve the raised palm's lighter appearance, including the isolated transition
at `[201,106]`. The final two shades occur only around the think expression's
eye. Offsets clamp to 0–255; only Hayden's lightest palm red channel clips, to
255. The reviewed result retains the visible palm shading.

Dark hair, purple eyeliner and lipstick, gray/white eyes, mouth interior, tears,
gold earrings and gold nails remain original. In particular, the lipstick's
`#925051`, `#743E51`, `#613954` and `#7C374C` are cosmetic colors outside the
skin map. Both hands, shoulders, neck and face recolor. All occurrences of the
ten configured skin colors were selected after inspecting the complete corpus.

The presets are Debug Blue, Hayden, Ryis and Seridia; Vanilla is implicit. The
combined trial uses Q for Wynne. Ryis shares the original `#63342A`, `#491F1B`
and `#612924`, so some selected pixels correctly retain their original RGBA.

## Verification

The mask selects 27,422 pixels with no other matching-color occurrences outside
the selection. Blue, Hayden and Seridia change all 27,422; Ryis changes 23,150
and keeps 4,272 selected pixels through identity mappings. The selection mask
SHA-256 is
`e4e96bce0ef61061e876d943b30fb957e838054da20cb385506f07eca2a808d1`, hashing each
profile-ordered path, a NUL delimiter and a selected/not-selected byte per pixel.
The audit uses Blue to recover the selection and checks every target against it,
including identity mappings.

```sh
nix-shell --pure --run 'cargo test --locked --test wynne_portraits -- --ignored --nocapture'
```

The local test passes 111 independently chosen literal skin/protected-detail
assertions, plus every-pixel source-or-target, alpha, dimension and unchanged
metadata checks. Omitting the lightest palm shade reproduced missed recoloring
at `[215,97]`; the final recipe passes. `FOM_WYNNE_RECIPE` selects this temporary
control without editing production data.

The author inspected all 16 full Source/Blue/Ryis frame trios, all 16 enlarged
faces, the raised palm and six distinct mouth crops across all five choices.
All 32 target strips passed exact validation. Original PNG and metadata bytes
matched the ZIP, and rebuilding the gallery reused all 16 frames without
conflicts. Reconstructing its selected components reproduced every final Blue
strip and metadata file exactly. Independent art review found no further
visual issue.

Final variants are in `generated/wynne-portraits-reviewed/variants`. Evidence is
ignored under `tmp/wiscar-wynne-author-*`; the combined audit is
`wiscar-wynne-author-final-verification.json`. Read-only source extraction used
`tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
No game assets were changed or committed. The user accepted the offline previews;
native rendering remains untested. Batch notes record combined installation checks.

Profile SHA-256: `80078aa1365f38ccaeb7cd44df6b404592a8bc2fcb6bcbd1f1ab1a81259b6692`.
Preset-set SHA-256: `1c41c55ee21bf67b9fd707eb357d27c5de19c1f84ffa1ecfd7261a9c95f1d4da`.
