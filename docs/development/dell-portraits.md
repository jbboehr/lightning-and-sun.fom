# Dell portrait authoring

The profile covers all 32 main Dell portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn and Winter. Each strip
contains two 296×180 frames, and all 64 frames are unique. The expressions are
embarrassed, happy, mad, neutral, sad, think, ugh and wink. There are no beach,
bathing or wedding portraits in this corpus. Overworld and UI assets are outside
the profile.

Source PNGs and metadata are under
`assets/animations/NPCs/Dell/Portraits/{Season}/`. Each season's eight metadata
files use its corresponding `PortraitsSpring`, `PortraitsSummer`,
`PortraitsAutumn` or `PortraitsWinter` atlas. The compact local preview is
`generated/dell-summary.html`; source images, output images and temporary Rust
helpers remain ignored.

## Source colors and boundaries

The catalog supplies three skin samples. Source inspection adds nine shades
used on eyes, ears, nose, mouth edges, neck, arms and hands. The profile records
original PNG hashes, dimensions and 9,156 explicit component seeds. Six connected-color
groups separate the shared hair, eyebrow, blush and clothing colors from the
larger skin regions.

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#F5C88E` | Main skin | `#9DB9D4` |
| `#DD905B` | Middle skin shading | `#7F9FBD` |
| `#A8571C` | Ear, neck, arm and finger shadows | `#6687AD` |
| `#691B03` | Deep nose, jaw and neck contours | `#445F83` |
| `#91390E` | Fine deep neck shading | `#445F83` |
| `#9D4428` | Closed-eye skin shading | `#6687AD` |
| `#B15634` | Lower-lip and fine eye shading | `#6687AD` |
| `#ECAA71` | Fine nose, eye, mouth and body shading | `#7F9FBD` |
| `#FFDEB4` | Small facial highlights | `#9DB9D4` |
| `#EC9871` | Isolated sad under-eye crease | `#7F9FBD` |
| `#DB9150` | Fine nose, neck, arm and hand contours | `#7F9FBD` |
| `#770C1D` | Deep jaw, elbow and hand creases | `#445F83` |

The blonde hair shares `#A8571C` and `#DB9150` with skin. Actual hair strands
above the ear and the long locks beside the jaw remain original; the inner-ear
curls, face-side shading and exposed neck wedges recolor. The true `#691B03`
eyebrow arches move with expressions independently of the fixed hair. These
arches stay original while the adjacent skin-colored creases follow the target.

The Spring and Summer elbow fold includes a continuous three-shade crease:
`#DB9150` at `[112,145]`, `#A8571C` at `[113,146]` and `#770C1D` at `[114,147]`.
All three recolor. Summer's pale elbow bandage is a separate accessory; its
`#FFDEB4` fill and isolated `#F5C88E` pixel stay original. Gold fasteners,
shoulder buckles, hair clips, the Autumn pencil, clothes and Winter knit cuffs
also retain their source colors. Exposed fingertips follow the skin ramp even
where they share colors with those materials.

All four necklines include a short shaded area below the jaw. Its dark and
middle skin colors recolor up to the black clothing boundary. In Winter,
`#A8571C` at `[143,115]` is the skin-side border; `[140,115]` and `[142,116]`
belong to the knit and stay original. Spring's last skin row at `[153,125]`
and `[154,125]` also recolors, while the gold fastener immediately below it
remains unchanged.

The outer lower-lip and speaking-mouth skin edges follow the palette. Actual
mouth interiors (`#9B2B1D`) and tongue colors (`#E96984`, `#C53849`) remain
original. The broad embarrassed blush remains intentional; its shared
`#EC9871` color maps only in the separate sad under-eye crease. Actual enlarged
Blue and Ryis mouths were inspected before accepting this treatment.

Each target selects and changes 61,260 pixels, while 29,980 other pixels sharing
the twelve source colors stay original. Temporary authoring rules are not part
of the runtime; the installed recipes use the recorded source-bound components.

## Presets and verification

`palettes/sets/dell-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. The natural targets use their catalog's four shades through the roles
above. All four targets share one mask; Vanilla is implicit in the installed
controls. Integration assigns Dell to Y.

```sh
nix-shell --pure --run 'cargo test --locked --test dell_portraits -- --ignored --nocapture'
target/release/mistria-palette build-presets \
  --original extracted/dell-portraits-study \
  --presets palettes/sets/dell-portraits-trial.json \
  --output generated/dell-portraits-final
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/dell.json \
  --output generated/dell-review-accepted
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all strips for dimensions, alpha, unchanged metadata, untouched unrelated
colors and exact mapped colors. Its literal source landmarks are independent
of the component seeds and cover all twelve shades, outer mouth shading,
tongues, eyebrow hair, ear strands, neck boundaries, elbow creases, bandage,
fasteners and knit material. `FOM_DELL_RECIPE` accepts a temporary recipe for
omission and spill checks.

Observed failures preceded their corresponding fixes: the catalog-only recipe
missed the closed-eye shade at Spring happy `[142,97]`; an unrestricted expanded
recipe changed hair at Spring neutral `[147,81]`; early masks missed the final
Spring neck row, the deep elbow-fold pixel, and the shared dark and middle neck
shades. The focused test retains 110 selected-skin and 29 protected landmarks,
plus 112 eye/ear fringe checks added by the follow-up below.

All four targets passed exact recipe validation across 32 PNGs and 32 unchanged
metadata files. The authored gallery at `generated/dell-review-accepted/index.html`
reuses all 64 frames with no pending or conflicting selections. A profile rebuilt
from those selections reproduces every applied pixel. All 64 full frames and
enlarged actual Source/Blue/Ryis face, mouth and neck comparisons were inspected;
the final neck-only delta was checked separately after the last correction.
Before the eye/ear follow-up, the independent art audit passed all 128 target
strips and 80 separately chosen source landmarks per target. Its selected-mask digest was
`1d15e69cf45186f352b6ba7c409e3fa78fd1347d639c0bfbf56e4ae88e2c2b59`.

Local evidence includes `tmp/dell-*-red.log`, `tmp/dell-final-test.log`,
`tmp/dell-*-validation.json`, `tmp/dell-frame-verification.json`,
`tmp/dell-final-faces-*.png` and `tmp/dell-final-inputs.sha256`. The supplied
archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Independent evidence is in `tmp/darcy-dell-art-review-dell-verification.json`
and `tmp/darcy-dell-art-review-neck-delta.json`. The final outputs are under
`generated/dell-portraits-final/variants/`; earlier study and reviewed directories
contain superseded candidates.

This pass adds declarative art recipes and a focused source-corpus test.
Combined generation, installation, repository checks and live-game testing
belong to the integration pass. No source game files or generated images are
staged by this workflow.

## Eye and ear correction

A user spot check found two isolated `#DB9150` skin pixels still outside the
mask: frame-local `[163,96]` beside the viewer-right eye and `[139,105]` below
the viewer-left ear. Their local source neighborhoods repeat across seasons.
The ear pixel occurs in all 64 frames; the eye pixel occurs in 48, excluding
the happy and wink expressions with a closed viewer-right eye.

The corpus test first failed on the omitted ear pixel. Adding these 112
singleton component seeds makes all 112 literal fringe checks pass. A separate
four-palette comparison against `generated/characters-darcy-dell-trial` proves
exactly 112 changed pixels per target, all at the expected locations, with no
other pixel or metadata changes. Nearby hair, lashes, and the previous 139
landmarks remain protected or mapped as before. The prior sixteen characters'
complete output trees, including Darcy, match the preceding bundle byte-for-byte.

Formatting, Clippy, all 87 active tests, both local portrait corpus tests, and
the release build pass. All 5,968 combined variants pass exact recipe validation.
The updated bundle is `generated/characters-dell-eye-ear-trial`, and the compact
PNG is `generated/dell-eye-ear-preview/summary.png`.

A fresh MOMI installation verified every selected Vanilla/variant atlas frame,
metadata entry, and generated script in `tmp/dell-eye-ear-playtest`. The focused
game run passed all 32 Dell portraits and five choices, plus one Darcy portrait:
33 distinct sources and 165 source/preset pairs. Actual Y cycles, four-season
wraps, independent selections, portrait phase, and an unobscured preview passed.
The wrapper exited successfully and the game logged a graceful exit. Dell's
installed Blue screenshot was inspected.

The refreshed gallery at `generated/dell-eye-ear-review` reused all 64 frames
with no pending selections or conflicts. Reconstructing its profile reproduced
all 32 Blue PNGs and their metadata byte-for-byte. The usual `tmp/play-characters`
launcher now points to the verified copy and still opens on Darcy. The supplied
archive's checksum remains unchanged. Desktop visual acceptance remains with
the user; uninstall and the world-action matrix were not rerun. No files were
staged or committed, and all game-derived artifacts remain ignored.

Local evidence is
`tmp/dell-eye-ear-red.log`, `tmp/dell-eye-ear-green.log`,
`tmp/dell-eye-ear-final-checks.log`, `tmp/dell-eye-ear-delta.json`,
`tmp/dell-eye-ear-inventory.json`,
`tmp/dell-eye-ear-previous-output-check.log`,
`tmp/dell-eye-ear-all-variants-validation.log`,
`tmp/dell-eye-ear-inputs.sha256`, `tmp/dell-eye-ear-install-report.json`,
`tmp/dell-eye-ear-live-run.log`, `tmp/dell-eye-ear-live-audit.json`, and
`tmp/dell-eye-ear-review-report.json`.
