# Nora portrait authoring

This profile covers all **32 main portrait strips / 64 unique frames** in the
supplied archive: eight expressions each for spring, summer, autumn and winter.
Each strip has two 296×180 frames. The expressions are embarrassed, happy, mad,
neutral, sad, think, ugh and wink. There are no beach, bathing or wedding strips
in this corpus. Overworld and UI assets are outside this profile.

The current lip comparison is `generated/nora-lips-preview/index.html`, with
`summary.png` and `mouth-comparison.png` beside it. The updated combined summary
is `generated/landen-nora-lips-preview/summary.png`.
The initial compact reviews were `generated/nora-summary.html` for Debug Blue and
`generated/nora-ryis-summary.html` for Ryis, with matching PNG files. Six
original/recolored pairs cover all four outfits, raised and supporting hands,
collar gaps, and closed and speaking mouths. All 64 unique frames were inspected
in the eight sheets at `tmp/nora-{outfit}-{page}.png`. All 64 actual
source/Blue/Ryis face trios were inspected in `tmp/nora-final-faces-{0..7}.png`.
The current component editor is `generated/nora-lips-review/index.html`.
Game-derived images and temporary Rust helpers remain ignored by Git.

## Source colors and boundaries

`palettes/profiles/nora-portraits.json` records **4,200 component seeds**, original
PNG hashes and dimensions. Eight skin shades map through four roles; two lip
shades have separate colors for each preset:

| Source | Authored role | Debug Blue |
| --- | --- | --- |
| `#FCDEBE` | Main light skin | `#9DB9D4` |
| `#EFA67A` | Light skin shadow | `#7F9FBD` |
| `#B65932` | Middle face, body and lip shading | `#6687AD` |
| `#762E21` | Deep skin contours | `#445F83` |
| `#D37249` | Fine neck shadow | `#6687AD` |
| `#F4BF94` | Fine neck and supporting-hand shading | `#7F9FBD` |
| `#F5BB90` | Fine lower-jaw shading | `#7F9FBD` |
| `#B35A35` | Brown eye and brow skin fringes | `#6687AD` |
| `#FCB0B6` | Lip highlight | `#9D93CB` |
| `#E6687A` | Lip middle shade | `#8F5788` |

The catalog supplied the first four shades. Although `#B35A35` is also a hair
color, independent source inspection found it in the eye and brow skin fringes.
Those small components recolor alongside nearby `#762E21`, `#B65932` and
`#EFA67A`, following the accepted Eiland/Olric treatment. Black outlines, green
and gray iris detail, and the actual dark `#501910` lash pixels stay original.

Five connected-color groups separate `#762E21`, `#B35A35`, and each lip shade
from the other six colors. This preserves their actual hair and jewelry uses
and keeps lip selection separate from skin. The long hanging
blonde lock, the hair beside the face silhouette, and the inward zigzag hair
outline retain their original contours. The isolated `#E5AE6E` at `[134,70]` is
inside the blonde curl and remains outside the source palette. These boundaries
were checked in enlarged source crops; there is no broad exclusion around the
eyes or brows.

Closed lip edges and lower-lip skin shadows follow the selected palette,
including the shifted shadow in speaking frames. The deepest speaking-mouth
contours remain original. Mouth-interior shades such as `#A1121D`, `#B45732` and
`#CE4865`, the pink tongue, blush and tears stay original. The lip colors are
adapted as described below. The actual Ryis output
was inspected to check for pale skin residues around the mouth and cheeks.

Exposed neck, fingers, hands and summer forearms recolor. The winter scarf has
a small skin window below its knot; its light, middle and deep shades are all
included. The autumn necklace sits over a blue scarf and retains its dark
outline even where that outline shares the skin palette. Clothing, buttons,
bracelet, earrings and hair retain their colors. Only explicit source-bound
seeds are retained in the profile; temporary authoring rules are not runtime
behavior.

Each target changes **89,464 pixels** and preserves **13,584 other pixels that
share source colors** across the source strips.

## Presets and atlases

`palettes/sets/nora-portraits-trial.json` orders Debug Blue, Hayden, Ryis and
Seridia. Each NPC target copies its catalog's four skin colors through the skin
roles above and adds the two authored lip colors, using the same profile.
Vanilla remains implicit in the installed
control; integration assigns Nora to **N**.

The original metadata maps eight strips each to `PortraitsSpring`,
`PortraitsSummer`, `PortraitsAutumn` and `PortraitsWinter`.

## Verification

```sh
nix-shell --pure --run 'cargo test --locked --test nora_portraits -- --ignored --nocapture'
target/release/mistria-palette apply \
  --input extracted/nora-portraits-study \
  --palette palettes/stylized/nora-portraits.json \
  --output generated/nora-portraits-study
target/release/mistria-palette validate \
  --original extracted/nora-portraits-study \
  --modified generated/nora-portraits-study \
  --palette palettes/stylized/nora-portraits.json
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/nora.json \
  --output generated/nora-review-authored
target/release/mistria-palette build-presets \
  --original extracted/nora-portraits-study \
  --presets palettes/sets/nora-portraits-trial.json \
  --output generated/nora-portraits-trial
```

Use fresh output directories when regenerating. The opt-in corpus test checks
all 32 strips, dimensions, alpha, metadata, unrelated colors and the main
face/neck tone in every expression. Its 71 literal source landmarks cover all
eight skin shades, eye and brow fringes, hair, iris/lash details, lighter lip
edges, mouth interiors, the autumn necklace and the winter collar gap. It
accepts `FOM_NORA_RECIPE` for mutation checks. Both the catalog-only mapping
and unrestricted eight-color mapping failed on the necklace at `[140,103]`.
A separate omission mutation left `#D37249` unchanged and failed on the fine
neck shade at `[149,94]`. The final masked recipe passes the same test.

All four targets passed exact recipe validation for all 32 strips. The authored
gallery reused all 64 frames with no pending or conflicting selections.
Reconstructing a profile from the gallery selections reproduced every output
pixel, and all applied frames matched the inspected author previews. Both
six-example summaries were inspected.

Local evidence is in `tmp/nora-catalog-red.log`,
`tmp/nora-unrestricted-red.log`, `tmp/nora-omission-red.log`,
`tmp/nora-green.log`, `tmp/nora-*-validation.json`,
`tmp/nora-verify-frames-report.json`, `tmp/nora-authored-report.json` and
`tmp/nora-final-inputs.sha256`. The source archive identity is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This authoring pass changes declarative recipes and a focused corpus test.
Combined package generation, installation, full repository verification and
live-game checks belong to the integration pass. No source game files or
generated images are staged by this workflow.

Initial independent verification, before the lip correction below, passed every
pixel in all four targets against 54
additional source-inspected landmarks, original PNG hashes/dimensions, alpha and
transparent RGB, unchanged metadata, exact target colors and identical selected
masks. The selected-mask digest is
`aa68669438aa37c8d0ab9b954d961563acfed593f904c06b62738ef584cc6b20`;
evidence is `tmp/landen-nora-art-review-nora-verification.json`.

## Lip color correction

The user spotted pale lips against the recolored skin, most visibly with Ryis.
The initial recipe adapted the surrounding skin shading while retaining the
original pink lip highlights. The resulting contrast made the lips appear as
a bright stripe. This correction gives those two lip shades colors suited to
each preset while preserving the tongue and mouth interior.

| Preset | Highlight (`#FCB0B6`) | Middle (`#E6687A`) |
| --- | --- | --- |
| Debug Blue | `#9D93CB` | `#8F5788` |
| Hayden | `#E88D6C` | `#D45349` |
| Ryis | `#B05653` | `#A03338` |
| Seridia | `#C18B9E` | `#B0526A` |

The source inventory found 240 highlight pixels and 64 middle-shade pixels,
all within the lip area (frame-local x150–154, y82–86). There are eight distinct
mouth crops across the 64 frames. Adding 176 component seeds selects these
304 pixels without altering the original skin selections. All eight mouth
crops were inspected in every preset, including closed and speaking frames.

The new regression builds the actual four-preset set and checks every lip pixel
across all four seasons and eight expressions, plus unchanged tongue/interior
colors. It first failed on an unchanged highlight at `[150,82]`, then passed
with the correction. The original corpus test retains its skin and protected
landmarks and now recognizes the two newly authored lip colors.

An exact comparison against the initial combined bundle confirmed that each
preset changes only the 304 lip pixels; all other pixels, dimensions, alpha,
and metadata remain identical. The authored gallery reuses all 64 frames, and
its exported selections reproduce the applied Blue output. The four standalone
outputs also match Nora's corresponding combined-package outputs.

Formatting, Clippy, all 87 active tests, both Nora corpus tests, the Landen
corpus test, and the release build passed. All 5,456 combined variants passed
exact recipe validation. The other twelve characters' complete output trees
matched the initial Landen/Nora bundle byte-for-byte.

A fresh MOMI installation in `tmp/nora-lips-playtest` verified the installed
pixels, metadata, and scripts. The real game checked all 32 Nora strips across
all five choices (160 source/preset pairs), actual N-key cycles, four-season
wraps, independent selections, portrait phase, and a hidden console. A Landen
smoke check adds five pairs; the final log audit records 33 distinct strips and
165 pairs with no script errors or disabled callbacks. The Ryis screenshot
was inspected, and the harness exited successfully after its checks.

The usual `tmp/play-characters` launcher was rebuilt and checked as executable
with valid shell syntax. F7 now opens Nora; N cycles her palettes. The user
accepted the revised appearance as good enough for the prototype on 2026-09-08
and approved committing the batch. A new desktop Wayland launch was not
confirmed in that exchange. Uninstall and the full world-action game
matrix were not rerun for this palette-only correction. The supplied archive
retained the original SHA-256 recorded above.

Evidence: `tmp/nora-lips-red.log`, `tmp/nora-lips-focused.log`,
`tmp/nora-lips-audit.json`, `tmp/nora-lips-profile-audit.json`,
`tmp/nora-lips-final-checks.log`, `tmp/nora-lips-all-variants-validation.log`,
`tmp/nora-lips-other-characters.log`, `tmp/nora-lips-install-report.json`,
`tmp/nora-lips-live-run.log`, `tmp/nora-lips-live-audit.json`, and
`tmp/nora-lips-inputs.sha256`.
