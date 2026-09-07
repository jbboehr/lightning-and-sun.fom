# Hayden portrait authoring

The first Hayden pass authors all **133 main portrait strips / 266 frames** from
the mounted archive. The batch contains 256 unique decoded frames. It includes
spring, summer, autumn, winter, beach, bathing, and wedding artwork. The beach
`shy_special` strip stored in the source's Spring directory is included too.
Hayden's overworld, UI, and child assets are outside this pass.

The user reviews `generated/hayden-mouth-hair-v3.png` or opens
`generated/hayden-mouth-hair-v3.html`: enlarged original/previous/corrected
comparisons of the mouth, chest hair, and arm hair.
`generated/hayden-summary-v3.png` and its HTML page show six full
original/Debug Blue comparisons. They do not
need to select regions or approve individual frames. The editor remains useful
for authoring and corrections.

## Masks and colors

- `palettes/profiles/hayden-portraits.json`: source-bound component seeds.
- `palettes/stylized/hayden-portraits.json`: Debug Blue recipe.
- `palettes/review/hayden.json`: optional gallery with the authored masks loaded.

The four catalog samples missed the lighter patches on Hayden's skin. Source
inspection added `#FAD8A8`, `#F9C4A1`, and `#CBAD93`. The first two map to the same
blue highlight as the main skin color; the third maps to the next darker blue.
This deliberately reduces the relative difference between the patch colors and
the main tone, as permitted by the user. This adds coverage for 54,860 lighter
patch pixels across the 266 frames. Vanilla source artwork is untouched.

A closer review found two more omitted shades, `#CA894D` and `#C58A4A`, around
the eyebrows, hairline, nose, and necklace. Both now map to `#7F9FBD`. This
corrects another 9,816 pixels (5,254 and 4,562 respectively). All occurrences
connect to the existing skin selections; expanding the color set does not join
any previously excluded components. The pale `#C3B5B5` streak at the temple and
sideburn is hair, also used in the braids, and stays original along with the
brown hair strands and pendant.

Authoring began with outfit and pose examples, reused matching connected regions,
then inspected all 256 unique frames in original/recolor sheets. The remaining
neck and hand variants and disconnected skin pixels were selected internally.
Only explicit seeds, source hashes, and palette data are retained in the profile;
the temporary selection rules are not part of the mod runtime. The exported
profile now has 29,983 seeds and changes 1,265,566 pixels in Debug Blue.

The mouth/body-hair correction partitions the source colors into three
`color_groups`: the previous nine skin shades, `#CE913B`, and `#9F7D6F`.
Additional seeds select 496 gold mouth-edge pixels and 33,114 brown body-hair
pixels across the corpus. Gold mouth shading maps to `#7F9FBD`, and body hair
maps to `#6687AD`, consistent with the darker skin shading already recolored.
This includes the chest, arms (including the raised-arm pose), and abdomen.
The head hair, beard, sideburn streak, and wedding ponytail stay original.
Every added component touches the previously selected skin mask. The source
colors remain separate during flood fill, so adding the new seeds cannot join
the skin mask to shirt trim or the beard.

The masks keep shared-color shirt seams, summer plaid shadows and buttons, belt
details, swimwear drawstrings, towel edging, and wedding accessories unchanged.
Pink blush and mouth-interior colors stay original. The previously missed gold
pixels on the viewer-left mouth edge are now included without recoloring gold
clothing. Grouping is preserved through the review gallery and preset builder.

## Local verification

The extracted corpus is `extracted/hayden-portraits-study`; the applied outputs
are `generated/hayden-portraits-v3`. Both are ignored by Git, as are all sheets,
gallery PNGs, and temporary authoring programs. No game material is staged.
The previous outputs and comparison sheets remain available for comparison.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test hayden_portraits -- --ignored --nocapture'
target/release/mistria-palette validate \
  --original extracted/hayden-portraits-study \
  --modified generated/hayden-portraits-v3 \
  --palette palettes/stylized/hayden-portraits.json
```

The corpus test checks lighter patches, added skin shadows, mouth details, body
hair, a face landmark in every frame, alpha, dimensions, metadata, unrelated colors, 20
shared-color clothing/accessory landmarks, and eight hair, eyebrow, and pendant
landmarks. It supports `FOM_HAYDEN_RECIPE` for mutation checks. Replacing
the three lighter mappings with identity mappings reproduced the reported miss;
the test failed on an autumn neck patch. Removing the profile also caused the
test to fail, on the autumn belt detail. Both mutations were kept outside the
repository's tracked data.

The v2 edge regression failed on an autumn hairline pixel before the palette
correction, then passed afterward. That verification passed formatting, Clippy,
74 normal tests, the Hayden corpus test, the release build, and exact recipe
validation of all 133 outputs. Comparing every output pixel against the previous
preview confirmed that only the 9,816 added-color pixels changed. The rebuilt
optional gallery reuses all 256 unique frame masks and leaves no pending groups.
Logs and reports are `tmp/hayden-skin-edges-red.log`,
`tmp/hayden-skin-edges-green.log`, `tmp/hayden-v2-final-checks.log`,
`tmp/hayden-v2-validation-report.json`, `tmp/hayden-v2-comparison-report.json`,
and `tmp/hayden-v2-gallery-report.json`.

For the mouth/body-hair pass, `tmp/hayden-mouth-hair-red.log` records the missed
mouth pixel before the data correction. `tmp/hayden-groups-red.log` records the
unsupported grouped selection before the runtime change. The corrected corpus
and grouping checks pass. `tmp/hayden-v3-comparison-report.json` confirms that
only the 33,610 intended detail pixels differ from v2; every other output pixel
is identical. `tmp/hayden-v3-validation-report.json` validates all 133 strips,
and `tmp/hayden-v3-gallery-report.json` records all 266 frame masks reused with
no pending or conflicting groups. The v3 close-up and six-example sheets were
visually inspected.
Reconstructing the profile from the gallery's selected components and color
groups reproduces all 133 outputs; see `tmp/hayden-v3-gallery-roundtrip-report.json`.

Removing the color groups in a temporary recipe made the corpus test fail on
a sideburn pixel, demonstrating why the detail colors must stay separate.
The failure is recorded in `tmp/hayden-v3-no-groups-test.log`.

V3 reliability verdict: **PASS_WITH_RESIDUAL_RISK**. Independent code review
found no actionable defects. Independent test hardening added coverage for
multi-color groups, exact RGBA boundaries, swapping target colors, legacy empty
groups, profile ownership, and grouped frame boundaries; no defects were found.
Fresh final verification passed formatting, Clippy, all 81 normal tests, the
Hayden corpus test, three Chromium tests, the Adeline corpus test, and the release
build. The combined log is `tmp/hayden-v3-final-checks.log`. The remaining risk
is live installation/rendering, which this offline art pass did not exercise.

The source archive still has SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.

This pass produced authored masks and a visual preview. The later
[independent character palettes](characters.md) slice added Hayden installation
and in-game selection, with MOMI and live-game verification recorded there.

Known art follow-up: the user noticed a few pixels that still look wrong in the
embarrassed expression. Their exact positions and source colors have not yet
been isolated. The user accepted this checkpoint and deferred that cleanup;
passing the recipe checks does not establish that every art detail is correct.
