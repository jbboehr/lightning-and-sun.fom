# Zorel portraits

The profile covers all 32 main Zorel portrait strips: embarrassed, happy, mad,
neutral, sad, think, ugh and wink in each of Spring, Summer, Autumn and Winter.
Every 592×180 strip contains two 296×180 frames with duration 0.2; all 64 frames
are unique. The eight strips in each season use that season's original atlas:
`PortraitsSpring`, `PortraitsSummer`, `PortraitsAutumn` or `PortraitsWinter`.

The original archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
This pass read `tmp/momi-lab/assets.bak.zip` because the game mount was empty.
Exported originals are under `extracted/zorel-portraits-study`; images, packages,
review sheets and temporary helpers remain ignored. Only recipe data, registry
entries, tests and documentation belong in Git.

## Colors and boundaries

The three catalog colors need seven additional shades for complete exposed
skin, pale palms and fine face, finger and mouth transitions.

| Source | Target role | Detail | Selected pixels |
| --- | --- | --- | ---: |
| `#B0755D` | Light | Main face, neck, arms and hands | 50,192 |
| `#985750` | Medium | Skin shading and creases | 15,004 |
| `#874048` | Shadow | Fine face and body contours | 4,348 |
| `#642E3D` | Deep | Jaw, nose, mouth and finger contours | 7,196 |
| `#432234` | Deep | Deep hairline, neck and skin outlines | 6,801 |
| `#A56553` | Medium | Lower-eye and chest skin transitions | 248 |
| `#7A354C` | Deep | Small skin/eyeliner transitions | 112 |
| `#DA977E` | Paler light | Lighter palms and fingertips | 4,336 |
| `#975250` | Medium | Mouth, arm and chest transitions | 2,716 |
| `#7A3118` | Deep, selected components only | Winter left-finger shading | 128 |

The paler light is the target light plus RGB `[42,34,33]`, with each channel
clamped to 255. This preserves the source palm contrast and gives `#C7DBF5`
for Blue, `#FFD492` for Hayden, `#DA8E78` for Ryis and `#EBD1C6` for Seridia.
All other colors use standard ramp roles.

The first nine colors have no matching-color exclusions in this corpus.
`#7A3118` is shared with Winter orange clothing. Five isolated components per
Winter frame select eight exposed finger pixels, totaling 128 across all 16
Winter frames; all 3,520 matching clothing pixels remain unchanged. The final
profile stores ten singleton groups and 21,609 seeds in 32 hash-bound regions.

Actual purple hair and Winter gloves (`#3F2A46`, `#241730`, `#321E39`), purple
eyeliner (`#6B1E6B`), green irises (`#1C4E44`), gray/pale eyes and pink mouth
interiors remain original. Cream wraps and waist fabric (`#F4E3C0`, `#C8A082`,
`#DBB796`, `#85605B`), seasonal clothes and the carried equipment also remain.
Selections cover the exposed fingers beyond Winter gloves, Spring arms and
Summer shoulder, arm and midriff. Original blush and eyeliner contrast against
the new skin remain subjective cosmetic details.

## Authoring verification

Final variants are under `generated/zorel-portraits-reviewed/variants`. All four
presets built and passed exact validation against source images and metadata.
Each changes 91,081 selected pixels and preserves 3,520 matching clothing pixels.
There are no identity mappings. The selection digest, hashing each ordered
path, a NUL separator and the per-pixel change bits, is
`5afe3da8294da78b2ef51226f3173f5a314dc8c24d950c7bd045fdf5ce20fbfa`.

```sh
nix-shell --pure --run 'cargo test --locked --test zorel_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/zorel-portraits-study \
  --presets palettes/sets/zorel-portraits-trial.json \
  --output generated/zorel-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/momi-lab/assets.bak.zip \
  --config palettes/review/zorel.json \
  --output tmp/zorel-author-review-authored
```

Use fresh output directories. The ignored corpus test applies all four target
ramps to all 32 strips and checks dimensions, metadata, alpha, exact replacement
colors, unchanged unrelated colors and identical selections across targets.
Its 80 literal skin and 47 protected landmarks per target were chosen from
source art independently of the stored seeds. These include fine face shades,
paler palms, arm shading, speaking-mouth colors, every corrected Winter finger
pixel in both frames, purple gloves and same-color orange clothing.

`FOM_ZOREL_RECIPE` overrides the Blue recipe for negative controls. The original
nine-color candidate failed at the missed Winter neutral finger `[71,112]`.
An unrestricted ten-color recipe then failed at recolored orange clothing
`[210,119]`. Both failures were observed, followed by passing all-four runs
with the final component selections.

The author inspected all 64 actual full Source/Blue/Ryis trios, all 64 enlarged
faces, four seasonal body/hand views and six all-four-target mouth closeups.
Independent review found the Winter finger omission after that initial pass.
The final correction changes exactly those 128 pixels per target compared with
the original candidate; all other pixels are identical. Final body/hand sheets
were regenerated and inspected after the fix.

Every visible source pixel lies within inclusive bounds `[67,33,231,179]`.
The author's full-art crop `[40,25,215,155]` preserves those bounds, and the
user review's face crop is `[125,55,44,48]`. The final gallery reuses all 64
frames without conflicts. Reconstructing its selected components and applying
the exported profile reproduces every final Blue PNG and metadata file byte
for byte.

Local evidence includes:

- `tmp/zorel-author-test-{fingers-red,spill-red,final-green,final-recheck}.log`
- `tmp/zorel-author-{blue,npc_hayden,npc_ryis,npc_seridia}-validation.json`
- `tmp/zorel-author-final-verification.json` and `tmp/zorel-author-finger-components.json`
- `tmp/zorel-author-actual-{full,faces,body}-*.png` and `tmp/zorel-author-actual-mouths.png`
- `tmp/zorel-author-final-gallery-report.json` and `tmp/zorel-author-gallery-roundtrip-match.log`
- `tmp/zorel-author-final-inputs.sha256`

Final profile SHA-256 is
`ea5056cc2398398aaf4915ea818f15b27a3423fdfb69fe8c6e59ef9b5bc3020e`;
the preset set SHA-256 is
`0add181f73b90ffd130e5db058532ad581e38c812d200bf413f434b339c1ac09`.

## Integration and offline review

The registry and collection add Zorel with R as the temporary palette control.
Vanilla is the session default, followed by Debug Blue, Hayden, Ryis and Seridia.
His 32 paths match the archive inventory and the game's native portrait table.
The combined package now contains 30 characters and 1,924 source animations:
1,907 portrait strips plus 17 Adeline world animations, with 7,696 recolored
variants. Shared Rust and GML runtime code is unchanged.

- [Compact five-palette summary](../../../generated/zorel-preview/summary.png):
  first Spring neutral frame only.
- [Complete Vanilla/Debug Blue review](../../../generated/zorel-preview/blue-review/index.html):
  four small pages, one per season, with eight expressions each.

The detailed review shows all 64 frames side by side in Vanilla and Debug Blue,
with full portraits at 2× and faces at 4×. Columns accommodate the outstretched
arms. The static local HTML and separate PNG sheets need no server or game.
They were generated from the final combined bundle; all 128 asset/frame/palette
views and 17,218,048 placed pixels matched the source images and manifest, with
no opaque pixels clipped. Chromium decoded all 32 sheets and resolved all
navigation links. The index, a full expression page and the corrected Winter
neutral sheet were visually inspected.

The independent art reviewer inspected all 64 full Source/Blue/Ryis frames,
all 64 faces, seasonal arms and torso, and 32 distinct mouth crops in all four
targets. The final exact audit checked 128 strips, 13,639,680 pixels and 61
independently selected landmarks per target. All 16 corrected Winter finger
occurrences were inspected in every target; no additional concrete art issue
was identified. See `tmp/zorel-art-review-final.md` for exact input/output hashes.
The user accepted the offline previews. This does not establish in-game rendering.

Fresh formatting, Clippy with warnings denied, all 87 active tests, the Zorel
corpus test and the release build passed through the pinned Fenix Nix shell.
All three existing opt-in GML tests also passed. The synthetic Zorel integration
case first failed with `Unsupported character: zorel`, then passed alone and
with the previous characters after registry insertion.

All 7,696 final variants passed exact recipe validation. The previous 29
character trees are byte-identical to the accepted Vera/Wheedle bundle; Zorel's
originals and all four variants match the reviewed standalone files. The target
roles and clamped palm shades were independently checked against the catalog.

A fresh isolated MOMI installation into `tmp/zorel-playtest` passed packed-atlas,
metadata and runtime-table verification. Its final archive SHA-256 is
`3934e0b1592773ead9b631e94cec29ec8483ca825d823a364a67d1135995df84`.
The receipt and retained previous archive match the original SHA-256 recorded
above. An earlier candidate install/uninstall roundtrip restored that original
archive exactly before the corrected installation. The final corrected install
was retained for testing. The source backup stayed unchanged.

The Nix-linked `tmp/play-characters` launcher was rebuilt for this isolated copy.
F7 opens on Zorel and advances expressions; Page Up goes backward. Shift+F4
changes character, Shift+F5 changes season, and R cycles Zorel's palette. The
helper preserves the prior world-scheduling fix and uses separate saves/state;
it is excluded from the player package. Native gameplay was not run because
the mounted game folder was empty.

Root verification evidence includes `tmp/zorel-final-checks.log`,
`tmp/zorel-integration-audit.json`, `tmp/zorel-review-tools-compile.log`,
`tmp/zorel-all-variants-validation.log`, `tmp/zorel-bundle-comparison.log`,
`tmp/zorel-preview-check.log`, `tmp/zorel-blue-browser-check.json`,
`tmp/zorel-trial-prep.log`, `tmp/zorel-install-report.json` and
`tmp/zorel-candidate-uninstall-report.json`. The final bundle is
`generated/characters-zorel-trial`; all game-derived artifacts remain ignored.
