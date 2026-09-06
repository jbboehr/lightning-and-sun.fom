# Portrait region masks

This slice refines the spring neutral Adeline portrait already supported by the
CLI and F6 toggle. It does not expand expression, outfit, or overworld coverage.

## Representation

The source recipe stores seed coordinates, source PNG SHA-256, and dimensions.
Four-neighbor traversal selects connected pixels whose original RGBA occurs among
the palette's source keys. Transparent pixels cannot connect regions. Selection
is computed before applying any replacement, so mapping chains cannot grow a mask.
Repeated seeds are allowed and have no additional effect. An entry with no seeds
selects nothing, while an absent `regions` field keeps unrestricted mapping.

The recipe requires an exact input PNG inventory. Its hashes bind coordinates to
the encoded source image, so even a re-encoded PNG needs a deliberate review and
checksum update. Invalid data fails before the output tree is written. Unknown
recipe fields are rejected; `description` remains optional.

Changing only target colors preserves the selected regions. Changing source-color
keys can connect or split regions and requires another visual review. This is a
small authoring format for reviewed assets, not automatic skin detection.

## Art review

The original three-color map also reached robe trim, clasps, earrings, and cuffs.
Inspection of color-connected components identified the face, ear, neck, upper
hand, chest, arms, and crossed hand as separate regions. The recipe supplies 16
seed points per frame, using strip coordinates for both frames. Some seeds cover
small disconnected edge pixels; overlapping seeds remain harmless.

The close-up also exposed a fourth skin shade, `#D48363`, left warm by the old
recipe. Its blue replacement is `#7F9FBD`. The other three target colors stay as
before. Dark outlines and eye highlights are outside this four-color ramp.

The reviewed result changes 3,259 pixels: 1,634 in the closed-mouth frame and 1,625
in the talking frame. Counts by original RGBA are:

| Source | Target | Changed pixels |
| --- | --- | ---: |
| `#E3A17BFF` | `#9DB9D4FF` | 2,439 |
| `#D48363FF` | `#7F9FBDFF` | 81 |
| `#C47054FF` | `#6687ADFF` | 507 |
| `#9F5544FF` | `#445F83FF` | 232 |

The mask protects 822 additional matching-color pixels. Both enlarged frames were
visually inspected. The output stays 592×180; alpha and metadata are unchanged.
Its PNG SHA-256 is
`a79f54ece5abe2b0dfa503e88ea5babfe2f01c21d2984c147da1fd6a4d72161e`.
The original PNG remains
`52e2871f2c96db217a568d15f3daaef666c2af9b90263ef0b39f0e38722c51bc`.

## Verification

The synthetic mask tests exercise disconnected and diagonal regions, transparent
barriers, partial alpha, overlapping seeds, invalid recipes, exact inventory, and
source guards. Recipe validation rejects omitted replacements, wrong colors,
outside-mask edits, and transparent-RGB edits. The preview test checks that its
magenta column represents actual changed pixels.

The local-asset test independently checks the lower robe area (all pixels at
`y >= 148`), reviewed jewelry landmarks, skin landmarks in both frames, and all 81
pixels of the fourth shade. Against the previous three-color recipe it fails on
the missed shade. Against an unrestricted four-color recipe it fails on a changed
robe pixel. The masked recipe passes both constraints.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test masks adeline_regions_preserve_clothing_and_cover_both_frames -- --ignored --exact --nocapture'
```

`FOM_PALETTE_TEST_RECIPE` lets the optional local test check a different recipe;
it is used for the unmasked regression demonstrations. Normal synthetic tests
contain no game images. Installer tests use their own source-bound synthetic
recipe, and a separate test confirms the embedded default rejects those unrelated
PNG bytes before publication.

Final verification passed formatting, Clippy with warnings denied, all 41 active
synthetic tests, the release build, and the separately invoked local-portrait test.
The three other opt-in tests (Fabricator lifecycle and the two older MOMI labs)
were not rerun for this slice; the current MOMI roundtrip is recorded below.

Reliability verdict: **PASS_WITH_RESIDUAL_RISK**. Independent correctness and test
reviews found no demonstrated defects or accepted static findings. The test review
added coverage for empty versus omitted selections, original-pixel traversal and
mapping chains, malformed hashes, and unmapped seeds. Live-game rendering of this
masked variant remains unverified.

Local preview commands, using fresh output paths:

```sh
target/release/mistria-palette apply --input extracted/adeline-rust \
  --palette palettes/stylized/adeline.json --output generated/adeline-masked-study
target/release/mistria-palette validate --original extracted/adeline-rust \
  --modified generated/adeline-masked-study --palette palettes/stylized/adeline.json
target/release/mistria-palette contact-sheet --original extracted/adeline-rust \
  --modified generated/adeline-masked-study \
  --output generated/contact-sheet-adeline-masked.png --changes
```

The contact sheet, generated PNGs, and investigation crops remain under ignored
`generated/` and `tmp/` directories. Neither game files nor derived images belong
in the commit. Broader portrait coverage and an in-game visual pass for this new
masked variant remain separate work.

## Installer integration

The Nix-built release binary installed the new embedded recipe into the disposable
`tmp/cli-installer-lab`, alongside the independently authored probe mod. Its report
matched the 3,259 changed pixels and masked PNG hash above. Both installed frames
passed the production atlas verifier, and both mod scripts were present.

```sh
target/release/mistria-palette install --game-dir tmp/cli-installer-lab \
  --installed-mods tmp/cli-installer-lab/config/mods/manifest.json
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab
cmp tmp/cli-before-othermod.zip tmp/cli-installer-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.bak.zip
```

All four commands exited zero. Removal restored the exact pre-palette archive
containing the probe; the pristine backup and supplied read-only archive remained
unchanged. Reports stay in `tmp/masked-install-report.json` and
`tmp/masked-remove-report.json`.
