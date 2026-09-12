# Darren portraits

This recipe covers Darren's eight Spring cameo portrait strips: embarrassed,
happy, mad, neutral, sad, think, ugh and wink. All use `PortraitsMisc`; each
592×180 strip has two unique 296×180 frames, giving 16 unique frames. The native
definition is `assets/fiddle/cameos/darren.toml`. The temporary palette key is D.

Originals came from ignored `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`,
and were exported to `extracted/darren-portraits-study`. Images and local review
artifacts remain ignored; the repository stores source-bound recipes and tests.

## Colors and boundaries

| Source | Standard target role | Selected pixels | Protected matching pixels |
| --- | --- | ---: | ---: |
| `#A3614E` | Light | 10,039 | 816 |
| `#83453A` | Medium | 2,889 | 1,296 |
| `#63342A` | Shadow | 996 | 688 |
| `#491F1B` | Deep | 782 | 288 |
| `#955343` | Medium | 651 | 0 |
| `#87473B` | Medium | 32 | 0 |
| `#392222` | Deep | 672 | 4,962 |

Seven singleton color groups and 3,408 seeds select face, ear, neck, and the
bare hands emerging from the cuffs. The first four colors are the catalog ramp;
`#955343` and `#87473B` cover small face/neck transitions. The last shade is
selected only along the hands' cuff-cast shadows, preserving its occurrences in
the eyebrows, beard and clothing. Coat buttons and shirt stitching share the
main skin ramp and require explicit exclusions.

Actual beard/fade colors (`#6A4A42`, `#4C2D2D`, `#5F3F36`, `#543C36`,
`#5E423B`, `#543830`), dark head hair, eye whites/irises, scarf, cream cuffs and
pink speaking-mouth interiors stay original. The warm `#BE7C66` cuff transition
is retained as fabric. Original beard, blush and mouth contrast remain cosmetic
choices, especially against Debug Blue.

There are no custom target formulas. Blue, Hayden and Seridia change 16,061
pixels each; Ryis changes 14,283 because its shadow and deep colors already equal
`#63342A` and `#491F1B`. Another 8,050 matching-color pixels are protected.

## Verification

All four presets built under `generated/darren-portraits-reviewed/variants`
and passed exact PNG/metadata validation. The focused test
`darren_covers_fine_skin_and_hands_without_recoloring_beard_cuffs_or_buttons`
applies all four targets to all eight strips. Its 18 literal skin and 16
protection landmarks were chosen from original art independently of mask seeds;
pixel checks also verify dimensions, alpha, unrelated colors and consistent
selection when a target is an identity mapping.

```sh
nix-shell --pure --run 'cargo test --locked --test darren_portraits -- --ignored'
```

`FOM_DARREN_RECIPE` supports local negative controls. Omitting `#87473B` failed
at neutral `[147,78]`; selecting a coat button failed at `[139,160]`. A later
independent review found three missing hand-shadow pixels at `[197..199,152]`.
The literal test failed at `[197,152]` before correction, then passed after
adding the component in both frames of all eight expressions. An exact audit
confirmed only those 48 pixels changed in each target.

The author inspected all 16 full Source/Blue/Ryis frame trios, all enlarged
faces and all 16 mouth rows across every target. Independent review inspected
the corrected hand in all four targets and passed 38 separate landmarks.
The final gallery reuses all 16 frames; reconstructing its selections reproduces
every final Blue PNG and metadata file byte for byte. Evidence is under
`tmp/darren-linnet-author-darren-*`, including the omission/spill/pocket red logs,
the pocket green log, four validation reports and `review-final/batch.json`.

Visible bounds are inclusive `[88,31,219,179]`; full review uses
`[80,25,165,155]`, with face detail `[130,38,50,48]`. The final profile SHA-256 is
`acf4034ba94f4b0045b23163f71e3621f07f46c331f1a7d237e6723203f964ce`.
See [batch integration](darren-linnet-wiscar-wynne-portraits.md) for the combined
package, user review pages and runtime-testing limits.
