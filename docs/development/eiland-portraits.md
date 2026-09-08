# Eiland portrait authoring

This profile covers all **78 main portrait strips / 156 unique frames** in the
supplied archive: 14 each for spring, summer, autumn, and winter; 15 beach/bathing
strips; and seven wedding strips. Overworld, UI, and child sprites are outside
this profile.

The accepted correction's preview is `generated/eiland-spots-preview/index.html`.
The original six-pose comparisons are `generated/eiland-summary.html` (Debug Blue)
and `generated/eiland-summary-ryis.html` (Ryis), with matching PNG files. Both
show spring, summer, autumn embarrassed, winter wink, bathing, and wedding.
All 156 frames were inspected as original/Blue/Ryis trios in 22 sheets at
`tmp/eiland-{outfit}-{page}.png`. The optional editing gallery is
`generated/eiland-review-authored/index.html`. These game-derived images and
temporary authoring helpers remain ignored by Git.

## Palette and masks

`palettes/profiles/eiland-portraits.json` binds **32,458 component seeds** to the
original strip hashes and dimensions. Nine source shades map through four roles:

| Source | Role | Debug Blue | Source-art use |
| --- | --- | --- | --- |
| `#E9A980` | Light | `#9DB9D4` | Main face and body tone |
| `#D98E72` | Medium | `#7F9FBD` | Skin and lip shading |
| `#C9785A` | Shadow | `#6687AD` | Face, body, and lip contours |
| `#9C5241` | Dark | `#445F83` | Dark skin and lower-lip shadows |
| `#E18F71` | Medium | `#7F9FBD` | Fine skin shading below the eye |
| `#6A3126` | Dark | `#445F83` | Deep nose, jaw, neck, ear, and hand contours |
| `#C17A5B` | Shadow | `#6687AD` | Alternate raised-hand shading |
| `#955342` | Dark | `#445F83` | Alternate hand and finger contours |
| `#653227` | Dark | `#445F83` | Deep alternate finger contours |

The last five shades were absent from the catalog's four samples. Four component
groups keep `#C9785A`, `#9C5241`, and `#6A3126` separate from the other six shades.
This separates skin from the same colors in ornate gold collars, braids, shoulder
trim, cuffs, belts, earrings, and the beach bracelet. Winter and wedding gloves
stay original. The current mask changes **296,274 pixels** and preserves **125,295
matching-color pixels** outside the selected skin components.

Lip edges and the small shadow below the lower lip follow the selected skin
palette. Actual open-mouth interiors, tongue, black brow/eye outlines, hair, and
blush stay original. Brown shading around the brows and eyes follows the skin
palette. The dark `#6A3126` mouth components are kept separate from skin shading;
mouth-only colors such as `#9B2B1D` are outside the source-color set. Ryis previews
were inspected alongside Debug Blue to catch pale lip residues.

Independent enlarged review confirmed the alternate hand shades, gold ring,
bracelet, and neck/collar boundaries. The bathing portrait has no earrings, so
its exposed ear uses its own selections. A single deep brown pixel at spring
`[131,84]` belongs to the horizontal gold collar piping and stays original.

Temporary rules suggested components using source color neighbors and local
geometry. Every frame was inspected before export. Only explicit seeds, source
hashes, dimensions, and palette data are retained; no classifier runs during
recoloring.

`palettes/stylized/eiland-portraits.json` supplies Debug Blue, and
`palettes/review/eiland.json` reloads the authored masks. The standalone set at
`palettes/sets/eiland-portraits-trial.json` orders Debug Blue, Hayden, Ryis, then
Seridia. Shared NPC targets copy the catalog colors through the same four roles.
Vanilla remains the implicit default in the combined package.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test eiland_portraits -- --ignored'
target/release/mistria-palette apply \
  --input extracted/eiland-portraits-study \
  --palette palettes/stylized/eiland-portraits.json \
  --output generated/eiland-portraits-study
target/release/mistria-palette validate \
  --original extracted/eiland-portraits-study \
  --modified generated/eiland-portraits-study \
  --palette palettes/stylized/eiland-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/eiland.json \
  --output generated/eiland-review-authored
target/release/mistria-palette build-presets \
  --original extracted/eiland-portraits-study \
  --presets palettes/sets/eiland-portraits-trial.json \
  --output generated/eiland-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all 78 strips, dimensions, alpha and transparent RGB, byte-identical metadata,
unmapped colors, and 38 literal skin, mouth, brow, collar, and jewelry landmarks
selected independently of the mask seeds. Its `FOM_EILAND_RECIPE` override supports
mutation checks. A catalog-only map failed on the bathing neck's omitted deep
shade at `[134,76]`. The initial mask failed on the uncovered bathing ear at
`[131,70]`; the corrected mask passed. An unrestricted nine-color map failed on
the protected beach bracelet at `[186,129]`.

All four targets passed exact recipe validation for all 78 strips. The authored
gallery reused all 156 masks without pending or conflicting frames. Every
applied frame matched the inspected author preview, and reconstructing the
profile from gallery selections reproduced every output pixel.

The initial independent verification passed all 312 target strips and 50 literal source
landmarks per target. It also checked source hashes and dimensions, exact target
colors, metadata, alpha and transparent RGB, and unrelated pixels. All four
targets shared the initial changed-mask digest, before the follow-up below,
`63c813e52817fc8c18a5fd62be4851a5bd97f663de9ef20af6931a5782ad2c51`.
The reviewer inspected all 156 full frames and enlarged initial face crops in
source/Blue/Ryis, finding no remaining peach lip residues or clothing spill.

Local evidence includes `tmp/eiland-red-catalog.log`,
`tmp/eiland-red-ear-collar.log`, `tmp/eiland-red-unmasked.log`,
`tmp/eiland-test-green.log`, `tmp/eiland-frame-verification.json`,
`tmp/eiland-{preset}-validation.json`, and
`tmp/eiland-olric-art-review-eiland-verification.json`. The source archive identity
is `b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

The initial authoring pass did not install or launch the game. Its integration
evidence belongs to the [Eiland/Olric integration pass](eiland-olric-portraits.md).

## Eye, cheek, and collar follow-up

The user found missed skin between the Spring collar edges, above the viewer's
left eye, beside the viewer's right eye, and on the lower-right cheek. The initial
selections had treated small eye-shading components and a cheek pixel as protected
details, and missed disconnected neck shading inside the collars.

The corrected profile adds 2,298 explicit component seeds across all 78 strips.
This recolors 4,012 additional pixels per target, including corresponding eye and
cheek details in other expressions and exposed neck gaps in Autumn, Winter, and
Wedding. Black outlines, earrings, collar trim, and mouth interiors remain as
before. Source hashes, color groups, target palettes, and every previous seed are
unchanged. The former protected brown brow-fringe landmark now expects the skin
shadow; a literal black brow pixel remains protected.

The expanded corpus regression failed on the original mask, then passed with
the correction. Formatting, Clippy, 87 active tests, both Eiland/Olric corpus
tests, and the release build passed. All 5,200 variants in
`generated/characters-eiland-spots-trial` passed exact recipe validation. A
comparison against the previous build confirmed that only the inspected 4,012
pixels per Eiland target changed; the ten other character output trees were
byte-identical. All 156 source/Blue face and collar crops were inspected.

MOMI installed the rebuilt package in `tmp/eiland-spots-playtest` and verified
its pixels, animation metadata, and scripts. A fresh game session passed all
five choices for all 14 Spring expressions and one expression in each other
outfit, plus J's full cycle and the switch to Olric. Captured Spring portraits
on Blue and Ryis were inspected. The user confirmed Eiland looked much better.
This trial is retained separately; the shared `./tmp/play-characters` launcher
follows the [latest combined trial](characters.md#local-visual-check).
The prior full runtime matrix and uninstall roundtrip were not repeated.

Evidence includes `tmp/eiland-spots-red.log`, `tmp/eiland-spots-final-checks.log`,
`tmp/eiland-spots-audit.json`, `tmp/eiland-spots-validation-count.txt`,
`tmp/eiland-spots-install-report.json`, and `tmp/eiland-spots-live-run.log`.
The corrected profile SHA-256 is
`f325bda1a3b262aed8a46b9aa774aa0cdca5c893821cac572256364ad41b8ca8`.
