# Adeline wedding portraits

This extends the [seasonal and beach trial](beach-portraits.md) with all seven
wedding strips. The combined set covers all **126 Adeline portrait strips** found
in the reviewed archive: 25 per season, 19 beach, and seven wedding. Each strip
has two frames. Vanilla, Debug Blue, Hayden, Ryis, and Seridia remain the five
choices. Overworld sprites and other characters remain outside this set;
selection is session-only, with Vanilla at launch.

## Source and mask review

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The archive inventory and `assets/fiddle/npcs/adeline.toml` agree on the wedding
expressions: embarrassed, happy_blush, hope_special, neutral, sad, sly, and think.
Their strips are 592×180, with two 296×180 frames and duration 0.2. All seven
belong to **PortraitsMisc**. The packager and installed-frame verifier retain that
atlas; unsupported wedding expressions are rejected.

The new definitions are:

- `palettes/profiles/adeline-portraits.json`
- `palettes/sets/adeline-portraits-trial.json`
- `palettes/stylized/adeline-portraits.json` (blue only)

The first 119 regions equal the previous profile. The wedding extension adds
216 component seeds bound to each source PNG's SHA-256 and dimensions. Source
colors E3A17B, D48363, C47054, and 9F5544 and all target ramps are unchanged.
No production GML changes were needed; packaging adds the wedding groups to the
existing runtime table.

Wedding artwork reuses skin shades in earrings, shoulder embroidery, necklace,
glove cuffs, bodice trim, and skirt details. Copying the spring mask by overlap
missed exposed chest and upper arm, missed isolated skin beside the far eyebrow,
and selected some gold cuff pixels. The final masks select the reviewed face,
ear, neck, chest, and upper-arm components. Earrings and disconnected matching
gold details remain original, as do the white gloves, dress, cape, crown, hair,
eyes, lips, blush, and other colors. The wedding torso remains at the same pixel
coordinates across these seven expressions.

Both frames of every wedding strip were inspected in all four recolors on
`generated/adeline-wedding-previews/page-1.png` through `page-3.png`. Each variant
changes 13,027 wedding skin pixels and preserves 4,088 matching detail pixels.
The full set changes 421,828 pixels per variant, or 1,687,312 across all four.
Every preceding seasonal/beach PNG and metadata file matches the prior bundle
byte for byte.

Originals are in `extracted/adeline-portraits-study`; generated variants and the
MOMI package are in `generated/adeline-portraits-trial`. Artwork, game copies,
previews, authoring helpers, logs, launchers, and saves remain ignored by Git.
Only recipes, seeds, hashes, code, tests, and documentation belong in the repo.

## Automated checks

The 126-asset export, 126-strip package, and mixed wedding/spring install tests
failed on the preceding implementation, then passed after adding the wedding
allowlist, Misc atlas mapping, and bounds. Installation tests also cover wedding
alone, wrong pixels, wrong folders, and original or variant frames on the wrong
atlas. Packaging rejects `happy` and `bath_neutral` for wedding.

The local corpus test checks both wedding frames, alpha, metadata, head and neck
skin, chest/arm landmarks, matching gold details, and unchanged colors. It failed
against the overlap candidate at `[164,69]`. A deliberate mutation removing the
chest components failed at `[150,104]`. Logs are `tmp/wedding-mask-red.log` and
`tmp/wedding-missing-chest.log`.

All 504 generated variant strips passed exact recipe validation. Reports are
`tmp/wedding-<id>-validation.json`. The Fabricator characterization includes
wedding/seasonal changes with the selected palette and fractional animation phase
retained, including Vanilla.

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test wedding -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored --nocapture'
```

Formatting, Clippy, all 63 active tests, the release build, and the local wedding
corpus and GML checks passed. The final log is `tmp/wedding-checks.log`.

## Isolated installation and playtest

A real MOMI roundtrip in `tmp/cli-installer-lab` verified all 126 originals and
504 variants in their source atlases. Uninstall restored the prior probe-mod
archive exactly:
`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`.
The pristine backup still matches the source archive. Reports are
`tmp/wedding-roundtrip-{install,uninstall}.json`.

A separate installed copy at `tmp/portrait-playtest` is available for review.
Close any older trial, then run **`./tmp/play-portraits`** from the normal NixOS
Wayland desktop terminal. This launcher uses the desktop GPU, a separate archive
and save/config directories, and a read-only mount of the original game. Older
launchers still use their older trials.

- F7 opens on wedding neutral, then advances; F10 goes backward.
- F6 cycles the five choices; the dialogue label shows the selected name.
- F4 switches directly between wedding and spring.
- F5 cycles spring, summer, autumn, winter, beach, and wedding.
- F8 replays talking; F9 checks fractional phase through a full palette cycle.

These controls belong to the ignored playtest helper. The distributed palette
mod adds only F6. The helper uses the game's wardrobe setter and portrait atlas
load/unload functions; the game's scene system also borrows an outfit's portrait
atlas when a scene needs one outside the current season.

The engine run in `tmp/wedding-engine/state/game.log` observed all **70** wedding
expression/preset/frame combinations. Comparison against the profile-derived
expected set found no missing entries; lists are
`tmp/wedding-{expected,observed,missing}-frames.txt`. The run passed **47**
fractional-phase cycles and **40** outfit changes: wedding/spring switches and a
full six-outfit loop for each choice. It exited gracefully without portrait
assertions or texture errors. The only engine error was unavailable Steam
initialization. Neutral screenshots for all five choices are beside the log.
This automated run used software Mesa under Xvfb; the desktop launcher uses the
host GPU. The new launcher was built through Nix and its shell syntax checked.

Natural wedding ceremonies, scene-driven atlas borrowing, and normal wedding
conversations were not exercised. The previous seasonal/beach expression matrix
was not repeated; its outputs were compared byte for byte. The two legacy MOMI
opt-in fixtures were not rerun; the current installation roundtrip covers this
bundle. User visual approval remains pending.
