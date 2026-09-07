# Adeline beach portraits

This extends the [four-season trial](all-season-portraits.md) with the 19 strips
in Adeline's `Beach` folder. There are 119 strips and 238 original frames in the
combined trial. Vanilla, Debug Blue, Hayden, Ryis, and Seridia remain the five
choices. Wedding's seven strips, overworld sprites, and other characters remain
outside this set. Selection remains session-only, with Vanilla at launch.

## Source review

The source archive is still
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Both the archive inventory and `assets/fiddle/npcs/adeline.toml` identify the
19 beach expressions. `bath_neutral` is the towel portrait within the beach
outfit; the seasonal tired/sick expressions are absent from that outfit.

All 19 strips are 592×180 with two 296×180 frames and duration 0.2. Their metadata
assigns them to **PortraitsSummer**, despite the `Beach` source folder. The
packager and installer retain that shared atlas family. No production GML change
was necessary; the generated table adds the new original/preset groups.

The combined definitions are:

- `palettes/profiles/adeline-seasonal-beach.json`
- `palettes/sets/adeline-seasonal-beach-trial.json`
- `palettes/stylized/adeline-seasonal-beach.json` (blue only)

The first 100 regions equal the preceding seasonal profile. The beach extension
adds 706 seeds, bound to each source PNG's SHA-256 and dimensions. The source
colors remain E3A17B, D48363, C47054, and 9F5544; target ramps are unchanged.

Reusing seasonal overlap alone missed the exposed shoulders and crossed arm.
The beach components were reviewed separately, including the small skin opening
between the swimsuit ties, abdomen, thighs, fingers, and towel silhouette. In
these 19 reviewed strips the four source shades occur only on skin. All their
components are selected, while the source binding remains mandatory. This does
not justify unrestricted matching on other outfits or a changed source image.
Swimwear, towel and gold trim, bracelet, earrings, hair, eyes, lips, blush, and
dark outlines use other colors and remain unchanged.

Seven poses move the bracelet and swimsuit up one source pixel: blush,
embarrassed, gloomy_special, hope_special, sigh, sly, and wink. The local corpus
test accounts for this offset when checking clothing landmarks.

Both frames of every beach expression were inspected across all four target
ramps on `generated/adeline-beach-previews/page-1.png` through `page-7.png`.
Each variant changes 152,457 beach skin pixels, with zero matching pixels
excluded. Together with the seasonal masks this is 408,801 changes per variant,
or 1,635,204 across four variants. All earlier seasonal PNGs and metadata in the
new bundle match the previous trial byte for byte.

Originals are in `extracted/adeline-seasonal-beach-study`; generated variants and
the MOMI package are in `generated/adeline-seasonal-beach-trial`. All artwork,
game copies, authoring helpers, previews, and saves remain ignored. The repository
contains transformations, seeds, hashes, tests, code, and documentation.

## Automated checks

The 119-asset export, 119-strip package, and mixed beach/seasonal install tests
failed on the preceding implementation, then passed after extending the bounds
and beach mapping. Synthetic installation also checks beach alone, wrong pixels,
wrong folders, and original or variant frames placed only on the wrong atlas.
Failed builds retain the previous archive and leave no recovery receipt.
Packaging rejects seasonal-only expressions under beach and `bath_neutral` under
summer, even though those outfits share an atlas.

The ignored local corpus test checks all 38 beach frames: every reviewed skin
shade, every other pixel, alpha, metadata, body landmarks, swimwear, bracelet,
and towel trim. It failed against the initial candidate at `[140,82]`, exposing
the missing shoulder/arm component. A deliberate recipe mutation removing the
neutral portrait's leg seeds failed at `[171,146]`; evidence is in
`tmp/beach-mask-red.log` and `tmp/beach-missing-legs.log`.

Exact recipe validation passed for all 476 generated strips. The Fabricator
characterization checks beach, towel, and seasonal portrait changes while
retaining the selection and fractional animation phase, including Vanilla.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test beach -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

Formatting, Clippy, all 63 active tests, the release build, and the local corpus
and GML checks passed. Logs are `tmp/beach-checks.log` and
`tmp/beach-local-checks.log`; per-preset validation reports are
`tmp/beach-<id>-validation.json`.

## Real installation and local playtest

A MOMI roundtrip in `tmp/cli-installer-lab` verified all 119 original strips and
476 variants in their correct atlases, including beach and summer together.
Uninstall restored the prior probe-mod archive exactly:
`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`.
The pristine backup still matches the source archive. Reports are
`tmp/beach-roundtrip-{install,uninstall}.json`.

A separate installed copy at `tmp/beach-playtest` is ready for visual review.
Run `./tmp/play-beach` from the normal NixOS Wayland desktop terminal. It uses
the desktop GPU, its own game archive and save/config directories, and a
read-only mount of the original game. Earlier launchers remain available.

- F7 opens on beach neutral, then advances; F10 goes backward.
- F6 cycles the five choices; the dialogue label shows the selected name.
- F4 switches directly between beach and summer.
- F5 cycles spring, summer, autumn, winter, and beach.
- F8 replays talking; F9 checks fractional phase through a full palette cycle.

These controls belong to the ignored playtest helper. The distributed palette
mod adds only F6. The helper uses the game's wardrobe setter and portrait atlas
load/unload functions; beach/summer switches share the loaded summer atlas.

The final engine run in `tmp/beach-engine-v2/state/game.log` observed all **190**
beach expression/preset/frame combinations, with no missing entries against the
profile-derived expected set. It passed **54** fractional-phase cycles and
**35** outfit changes: direct beach/summer switches and a complete seasonal
cycle with each of the five choices. Expected, observed, and missing lists are
`tmp/beach-{expected,observed,missing}-frames.txt`. Screenshots of neutral and
towel portraits with each choice are in the same state directory.

An initial helper run left the startup spring atlas loaded, producing duplicate
texture messages on the first return to spring. The helper now unloads that
atlas before opening the initial beach portrait. The final run repeated the
full beach matrix and transitions after rebuilding the isolated installation;
it exited gracefully with no texture or portrait assertions. The only engine
error was the expected unavailable Steam initialization. Automated runs used
software Mesa under Xvfb; the desktop launcher uses the host GPU.

These are controlled wardrobe and atlas transitions. Natural NPC beach
schedules, dates, and ordinary beach conversations were not exercised. The
seasonal expression matrix and calendar rollover were not repeated in this
slice; its seasonal output was compared byte for byte instead. The two legacy
MOMI opt-in fixtures were not rerun; the current real installation roundtrip
above covers this bundle. User visual approval remains pending.
