# Linnet portraits

This recipe covers Linnet's eight Spring cameo portrait strips: embarrassed,
happy, mad, neutral, sad, think, ugh and wink. All use `PortraitsMisc`; each
592×180 strip has two unique 296×180 frames, giving 16 unique frames. The native
definition is `assets/fiddle/cameos/linnet.toml`. The temporary palette key is C.

Originals came from ignored `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`,
and were exported to `extracted/linnet-portraits-study`. Images and local review
artifacts remain ignored; the repository stores source-bound recipes and tests.

## Colors and boundaries

| Source | Standard target role | Selected pixels | Protected matching pixels |
| --- | --- | ---: | ---: |
| `#C7875C` | Light | 7,252 | 0 |
| `#A36049` | Medium | 2,632 | 32 |
| `#6D3328` | Shadow | 364 | 16 |
| `#4A1B16` | Deep | 189 | 0 |
| `#B16A52` | Medium | 853 | 0 |
| `#8F5042` | Shadow | 405 | 32 |
| `#472721` | Deep | 20 | 336 |
| `#6A3126` | Deep | 300 | 32 |
| `#B9755A` | Medium | 234 | 0 |
| `#884636` | Shadow | 203 | 48 |
| `#6C3327` | Deep | 112 | 0 |
| `#C87463` | Medium | 93 | 0 |
| `#BB6C5D` | Shadow | 72 | 0 |
| `#5B241E` | Deep | 58 | 0 |
| `#9A5547` | Shadow | 64 | 0 |
| `#9F5544` | Shadow | 16 | 0 |

Sixteen singleton color groups and 2,787 seeds cover the exposed face, ear and
neck. The first four shades are the catalog ramp; the others cover fine eyebrow,
eye, nose, cheek, lip and neck transitions. Each target changes 12,867 pixels,
with 496 matching-color pixels protected. All targets use standard ramp roles;
there are no custom formulas or identity mappings.

The discrete brown borders of the gold circlet are protected. One connected
`#A36049` region meets its right edge and continues into forehead shading; that
whole skin-shadow component remains selected. Gold trim `#C9785A` was removed
from the candidate source list after inspection showed every occurrence belongs
to jewelry or armor. White/lavender hair, purple irises, gray eye shades, armor,
gloves and the gold trim remain original.

The diagonal pink cheek/eye stripe is treated as an intentional scar:
`#EF977D`, `#FFA793` and its `#D98868` transition remain original. Pink/red
speaking-mouth interiors are also retained. Both author and independent reviewer
found those boundaries coherent in actual Blue/Ryis and all-target mouth crops;
their contrast against different skin tones remains a cosmetic choice.

## Verification

All four presets built under `generated/linnet-portraits-reviewed/variants`
and passed exact PNG/metadata validation. The focused test
`linnet_covers_fine_face_and_neck_shades_without_recoloring_scar_circlet_or_armor`
applies all four targets to all eight strips. Its 19 literal skin and 22
protection landmarks were chosen from original art independently of mask seeds;
pixel checks also verify dimensions, alpha, unrelated colors and consistent
selection across all targets.

```sh
nix-shell --pure --run 'cargo test --locked --test linnet_portraits -- --ignored'
```

`FOM_LINNET_RECIPE` supports local negative controls. Omitting the fine
`#B16A52` skin shade failed at neutral `[147,48]`; selecting the circlet's dark
border failed at `[148,44]`. Both controls were observed failing, followed by a
passing final all-four test.

The author inspected all 16 full Source/Blue/Ryis frame trios, all enlarged
faces and all 16 mouth rows across every target. Independent review passed all
final full/face views, six distinct all-target mouth crops and 43 separate
landmarks. The final gallery reuses all 16 frames; reconstructing its selections
reproduces every final Blue PNG and metadata file byte for byte. Evidence is
under `tmp/darren-linnet-author-linnet-*`, including omission/spill red logs,
four validation reports and `review-final/batch.json`; the final passing test
log is `tmp/darren-linnet-author-tests-final-green.log`.

Visible bounds are inclusive `[83,26,241,179]`; full review uses
`[80,25,165,155]`, with face detail `[133,38,44,57]`. The final profile SHA-256 is
`75c381f30c1c0d92180b91cb8f6642c8838305eedbc342acdcdcede2475cbf2a`.
See [batch integration](darren-linnet-wiscar-wynne-portraits.md) for the combined
package, user review pages and runtime-testing limits.
