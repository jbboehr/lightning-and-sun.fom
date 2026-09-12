# Merri portraits

The profile covers all 32 main Merri portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. The expressions
are embarrassed, happy, mad, neutral, sad, think, ugh and wink. Each 592×180 PNG
contains two 296×180 frames, with duration 0.2; all 64 frames are unique.

All four seasons use `PortraitsMisc`. Spring, Summer and Autumn have seasonal
subfolders, while Winter PNGs live directly under `Merri/Portraits/`. The
profile retains those exact source paths. The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

Local source files are in `extracted/merri-portraits-study`. Source images,
outputs, review sheets and temporary Rust authoring helpers remain ignored;
the repository contains only recipes, source-bound component seeds, this note
and the focused test.

## Skin and protected details

Four fine colors supplement Merri's four catalog shades:

| Source | Target role | Source detail |
| --- | --- | --- |
| `#DD9266` | Light | Main face, neck, arms and hands |
| `#CB7A56` | Medium | Skin shading |
| `#B36644` | Shadow | Fine contours and outer lip shading |
| `#7F382B` | Deep | Eye/brow skin fringes and deep skin contours |
| `#9E512F` | Deep | Ear, eye, jaw and finger creases |
| `#7E4C3B` | Deep | Closed lower-lip edge |
| `#F0A87E` | Pale blend | Tiny lower-eye highlights outside the gray iris |
| `#482323` | Deep | Cheek and under-jaw contour; shared with actual iris detail |

The pale eye blend follows the target skin while keeping its brightness above
the main light shade. Its source RGB difference from `#DD9266` is `(19,22,24)`;
adding that same difference to each target light gives:

| Preset | Pale eye blend |
| --- | --- |
| Debug Blue | `#B0CFEC` |
| Hayden | `#FBC889` |
| Ryis | `#C3826F` |
| Seridia | `#D4C5BD` |

No channel requires clipping. These two tiny highlights in the neutral frame
are skin/eye blends, rather than gray iris material. Leaving them peach would
repeat the pale-fringe issue corrected in Luc.

The black eyebrows remain black while their brown skin fringes recolor.
Actual gray iris colors and the dark `#482323` iris details stay original;
the same deep color at the cheek edge `[136,83]` and jaw `[141,87]` maps.
Closed eyes retain their skin shading without inheriting open-eye exclusions.

Closed and speaking lip edges follow the skin. The red mouth interior and pink
tongue stay original. Actual closed and speaking mouths were compared in all
four target palettes; no separate lip cosmetic change was needed. Expression
blush and the Ugh sweat drop also remain original.

The profile uses eight singleton color groups and 13,816 explicit seeds across
32 hash-bound regions. Temporary connected-color studies identified exposed
skin, including the full Summer forearms and Spring hanging fingers. The
stored profile contains the reviewed components; installation does not run
the temporary authoring rules. Blouses, collars, cardigan stitching, ties,
belts, bags, skirt patterns and Winter gloves retain their original colors.

## Verification

Debug Blue, Hayden, Ryis and Seridia use the same selections. Vanilla remains
implicit in the installed controls. Shared integration assigns Merri to F1.

```sh
nix-shell --pure --run 'cargo test --locked --test merri_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/merri-portraits-study \
  --presets palettes/sets/merri-portraits-trial.json \
  --output generated/merri-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/merri.json \
  --output generated/merri-review-authored
```

Use fresh output directories. The reviewed variants are in
`generated/merri-portraits-reviewed/variants`. All four palettes passed
validation against the 32 originals and their metadata, with 88,720 changed
pixels and 2,320 protected matching-color pixels each. None of the eight
source/target pairs is an identity mapping, so selected and changed counts
agree. The shared selection digest is
`1871aca25bae060b50dfc21a871ef8bb353205b598e5733c0df2cf504809c572`.

The ignored corpus test applies all four palettes and checks every strip for
dimensions, unchanged metadata, alpha, exact source-or-target colors, unchanged
unrelated colors, equal selections and all eight literal target colors. Its
29 source-art landmarks cover fine eye and jaw shades, lower lips, hands,
actual iris details, eyebrows, speaking interiors, a cuff and Winter collar.
The coordinates were chosen independently of the stored seeds.

`FOM_MERRI_RECIPE` overrides the Blue recipe for negative controls. The original
four-color recipe failed at the missed `#9E512F` eye fringe `[139,64]`; an
unrestricted eight-color recipe failed at the recolored iris `[141,70]`.
Both negative controls were observed failing. The source-bound candidate then
passed the focused all-four test, and the final test file is formatted.

The author inspected all 64 actual full Source/Blue/Ryis trios, all 64 enlarged
face/neck trios and all four seasonal hand/body comparisons. Further five-way
closeups cover neutral eyes, the closed and speaking mouth, raised fingers
and the hanging hand. Local evidence includes `tmp/merri-test-catalog-red.log`,
`tmp/merri-test-spill-red.log`, `tmp/merri-test-first-green.log`,
`tmp/merri-*-validation.json` and `tmp/merri-actual-{full,faces,hands}-*.png`.

The independent reviewer completed the same full-frame, face and seasonal hand
sweep without finding another concrete omission or spill. Its exact audit
passed all 128 target strips and 25 separately chosen landmarks. Evidence is
`tmp/merri-terithia-art-review-merri-verification.json`. The approved candidate
and final reviewed variant directories are byte-identical.

The authored gallery reuses all 64 frames without conflicts. Reconstructing a
profile from its selected components and applying it reproduces every actual
Blue pixel. The temporary authoring previews also match all 64 frame
occurrences. Evidence is `tmp/merri-roundtrip-verification.json`.

Final profile SHA-256 is
`70e7f5b250307f745556a9a033ab7c944d96b12b038c22f4f3b7f9d70858209d`.
The preset set SHA-256 is
`d4691c146be59662cf16a38dd4d3cbca06cbd8256a4e37a7ed8f09a4b66dfd50`.

Shared integration owns the combined preview, full repository checks, bundle,
installation and live-game trial. Those actions are outside this authoring pass.
