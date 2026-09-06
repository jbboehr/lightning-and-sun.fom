# Adeline spring portrait coverage

This slice adds an opt-in recipe for all 25 spring portrait strips in the supplied
archive: 50 frames in total. The default installer recipe remains the neutral
portrait that the user tested. Other outfits, overworld animations, secondary
cutscene portraits, and other characters are outside this slice.

## Recipe and art review

`palettes/stylized/adeline-spring.json` uses the same four source/target colors as
the neutral study. Each source PNG has its own SHA-256, dimensions, and seed list.
Several poses shift relative to neutral, so a single coordinate list is not
reused blindly. Both frames of every expression were inspected on comparison
pages. The neutral result remains identical to the prior masked study.

The corpus check found four missed pixels around the sly expression's eyebrow,
at [144,66], [143,67] and their second-frame counterparts. These disconnected
skin regions now have explicit seeds. The test failed on the missed fourth shade
before these additions and passed afterward. A separate run with unrestricted
four-color mapping failed because it recolored the robe at [123,148].

The result changes 80,951 pixels, including all 2,081 pixels of the fourth skin
shade, and preserves 20,598 other pixels that an unrestricted map would change.
Lip treatment remains provisional following the user's neutral-portrait feedback.
Blush, eye makeup, and dark outlines keep the existing recipe's treatment; this is
a coverage pass, not a final color/art pass.

| Expression | Changed pixels | Matching pixels outside mask |
| --- | ---: | ---: |
| angry_blush | 3,216 | 822 |
| blush | 3,112 | 826 |
| cartoon_embarrassed | 3,441 | 826 |
| concerned | 3,248 | 822 |
| embarrassed | 3,104 | 826 |
| embarrassed_tired | 3,215 | 826 |
| evasive_tired | 3,249 | 826 |
| gloomy_special | 3,113 | 826 |
| happy | 3,373 | 822 |
| happy_blush | 3,337 | 822 |
| hope_special | 3,116 | 826 |
| mad | 3,252 | 822 |
| neutral | 3,259 | 822 |
| neutral_tired | 3,257 | 826 |
| sad | 3,250 | 822 |
| shocked | 3,260 | 822 |
| sick_eyes_closed | 3,308 | 824 |
| sick_eyes_open | 3,218 | 826 |
| sick_smile | 3,309 | 824 |
| sick_think | 3,216 | 826 |
| sigh | 3,250 | 822 |
| sly | 3,172 | 826 |
| think | 3,256 | 822 |
| ugh | 3,230 | 822 |
| wink | 3,190 | 822 |

Source images are local under `extracted/adeline-spring-study`; final output is
under `generated/adeline-spring-study`. The five cropped comparison pages are
`generated/adeline-spring-review/page-1.png` through `page-5.png`, with original
and recolored views for each frame. The source tree was copied from the exact
Adeline spring PNG/metadata members of the supplied read-only ZIP. Diagnostic
authoring code and candidate previews remain in ignored `tmp/`.

## Package and runtime

The explicit exporter now accepts 1–25 distinct PNGs. The legacy replacement
packager keeps its two-change limit. `package-toggle` accepts 1–25 supported
Adeline spring expressions, checks every horizontal strip and animation metadata
record, rejects duplicate expression basenames, and removes original IDs from
each new animation's metadata.

A generated `gml/palette_assets.gml` contains the included vanilla/variant name
pairs. The production toggle script resolves those pairs at initialization. F6
changes only a matching included sprite, including after a later speaker or
expression change, and preserves its fractional animation phase. A missing
required asset disables the toggle. Vanilla remains the default for every launch.

For masked installation recipes, the region asset list selects the exports.
Recipes without regions retain the neutral-only selection. No images are bundled
in the binary or repository; the installer generates them from the local game.

```sh
target/release/mistria-palette install --game-dir '/path/to/Fields of Mistria' \
  --palette palettes/stylized/adeline-spring.json
```

Existing mod-source, installed-list, load-order, backup, and removal requirements
still apply. The CLI verifies each original and each variant, plus the exact
toggle and generated mapping scripts, before publication.

Script verification uses this package's MOMI directory,
`assets/gml/scripts/lightning_and_sun_adeline_palette_toggle_study/`. Another mod
may use the same filenames in its own directory. Independent review identified
that the initial archive-wide basename check rejected this valid combination;
regression tests also demonstrated that it accepted a script from the wrong mod
when this package's copy was missing. Both tests failed before the directory
check and passed afterward.

## Atlas pages

The real 25-expression MOMI build overflowed the original 4096×4096 spring atlas
and created `PortraitsSpringAtlas_1.png` with corresponding metadata. The first
verification attempt rejected this build and left the prior archive intact
because it searched only the unnumbered page.

Verification now locates the unnumbered page and every numbered spring atlas page.
A frame must have exactly one placement across that complete set; its canvas,
trim offsets, and recovered RGBA must match the expected strip. Synthetic checks
cover an animation split across pages, duplicate placements on different pages,
and wrong pixels on the additional page. The split-page check failed before this
change and passed afterward.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" \
  nix-shell --run 'cargo test --locked --test gml_runtime -- --ignored --nocapture'
nix-shell --pure --run 'cargo test --locked --test spring spring_masks_cover_skin_and_preserve_reviewed_clothing_landmarks -- --ignored --exact --nocapture'
nix-shell --pure --run 'cargo test --locked --test masks adeline_regions_preserve_clothing_and_cover_both_frames -- --ignored --exact --nocapture'
```

Final verification passed with 51 active tests, formatting, Clippy across all
targets, and the release build. The three current opt-in tests above also passed.
The focused namespace regression command passed both tests:

```sh
nix-shell --pure --run 'cargo test --locked --test installer palette_scripts_'
target/release/mistria-palette validate --original extracted/adeline-spring-study \
  --modified generated/adeline-spring-study \
  --palette palettes/stylized/adeline-spring.json
```

The final local output validation passed with `palette_verified: true`.

The opt-in corpus test checks all 25 strips and both frames, every fourth-shade
pixel, reviewed interior skin and jewelry landmarks common to the poses, all robe
pixels at y >= 148, alpha, and sidecar bytes. `FOM_SPRING_RECIPE` can supply another
recipe for regression demonstrations. Synthetic tests contain no game images.

The updated Fabricator lifecycle test uses two independently defined expression
pairs. It verifies switching expression while blue is selected, toggling both
ways without changing expression or phase, unrelated sprites, menu lifecycle,
and missing assets on either side of a later pair. The new expression assertion
failed before the runtime change.

Independent test review also added checks for empty, duplicate, and oversized
export selections; all 25 generated expression pairs; unsupported near-miss names;
neutral fallback without regions; and overlapping animation IDs.

The real MOMI integration used the existing probe-mod lab:

```sh
target/release/mistria-palette install --game-dir tmp/cli-installer-lab \
  --installed-mods tmp/cli-installer-lab/config/mods/manifest.json \
  --palette palettes/stylized/adeline-spring.json
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab
cmp tmp/cli-before-othermod.zip tmp/cli-installer-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.bak.zip
```

These commands passed: the installed verifier recovered all 100 vanilla/variant
frames across both pages and checked both runtime scripts. Removal restored the
exact probe-only archive and retained the pristine backup. Reports remain under
`tmp/spring-install-report.json` and `tmp/spring-remove-report.json`; a local
snapshot is `tmp/spring-installed.zip`. The original game files remain untouched.

The interactive user test covered the earlier neutral portrait, as recorded in
[portrait-regions.md](portrait-regions.md#interactive-playtest). The expanded
25-expression runtime subsequently passed the isolated engine test and the user
approved the expanded visual playtest ("damn looks pretty good"). Lip treatment
remains provisional. The two older
MOMI opt-in lab tests were not rerun against their stale one-portrait installations.

Reliability verdict: **PASS_WITH_RESIDUAL_RISK**. The independent correctness
review's namespace finding was reproduced and fixed, including the missing-script
case discovered by its regression test. The independent test review found no
additional production defects and added the protections described above. No
accepted static findings remain. Final checks and the full MOMI roundtrip were
rerun after the fix. The user later approved the expanded visual pass; provisional
lip colors remain. The next [palette preset study](palette-presets.md) moves these
same masks into a shared profile and adds native creator-color trials.
