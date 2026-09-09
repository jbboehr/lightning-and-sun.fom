# Elsie and Errol portrait batch

The later [Errol hair-fringe correction](errol-portraits.md#hair-fringe-correction)
updates the current summary and trial. The batch record below remains the
baseline for that bounded comparison.

This batch adds all 36 Elsie and 36 Errol portrait strips in the supplied archive:
nine expressions in each of Spring, Summer, Autumn, and Winter. Each strip has
two 296×180 frames. Elsie includes `closed_eyes`; Errol includes `neutral_closed`.
Both characters have Vanilla, Debug Blue, Hayden, Ryis, and Seridia choices,
with Vanilla selected at launch. Z cycles Elsie; X cycles Errol.
The shared Rust and GML runtime is unchanged.

The complete collection has 1,564 source strips and 6,256 recolored variants
across nineteen characters. That includes 1,547 portraits and Adeline's 17
previously reviewed world animations. Earlier recipes retain their accepted
state, including Dell's two eye/ear fringe corrections. Deferred cosmetic
contrast and earlier character art follow-ups remain separate.

The archive metadata uses the matching four seasonal atlases for both
characters. All 72 filenames match the outfits and expressions in the game's
native NPC definitions.

Character-specific mask details are in [Elsie portraits](elsie-portraits.md)
and [Errol portraits](errol-portraits.md).

## Local visual check

The compact comparison is `generated/elsie-errol-preview/summary.png`, with
`index.html` beside it. The combined bundle is
`generated/characters-elsie-errol-trial`. Images, packages, extracted assets,
and trial game copies stay ignored and local.

The `tmp/play-characters` launcher uses `tmp/elsie-errol-playtest` with separate
saves and state, mounting the supplied game read-only. F7 opens on Elsie;
F4 moves to Errol. Z and X cycle their palettes. F5 changes season; F7 and
Page Up move through expressions. F9 checks independent selections and
fractional portrait phase. These preview controls are local test helpers
excluded from the player package.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when the source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test elsie_portraits --test errol_portraits -- --ignored --nocapture'
```

The synthetic collection test first failed with `Unsupported character: elsie`.
After registry integration, all four character tests passed, covering standalone
builds, the nineteen-character collection, Z/X controls, Vanilla-first choices,
generated pixels, and differing preset counts among existing characters.

Formatting, Clippy, all 87 active tests, both new local corpus tests, and the
release build passed. The three local GML interpreter checks also passed.
All 6,256 final variants passed exact recipe validation. The previous seventeen
characters' entire output trees, including originals, variants, metadata, and
reports, matched `generated/characters-dell-eye-ear-trial` byte-for-byte.
The new characters' variant trees match their final reviewed authoring outputs.

The independent archive audit checked complete PNG/metadata inventories,
source hashes and exported bytes, original atlases, dimensions and frame counts,
unique registry paths, profile coverage, preset arity, standalone Blue
equivalence, and catalog shade roles. All native outfit/expression pairs match.

The independent art review passes both characters. All 144 unique full frames
and enlarged Source/Blue/Ryis face comparisons were inspected, along with
seasonal hands and neck boundaries. Exact pixel, selection, source-hash, alpha,
and metadata checks passed all 288 target strips. There are no identity-color
exceptions in these four targets. Both authored galleries reuse all 72 frames
per character, and their applied Blue roundtrips match every pixel.

| Character | Source shades | Component seeds | Changed pixels per target | Protected matching pixels |
| --- | ---: | ---: | ---: | ---: |
| Elsie | 8 | 19,066 | 135,136 | 4,782 |
| Errol | 7 | 4,808 | 138,042 | 160 |

The repository tests check 55 Elsie and 167 Errol literal source landmarks;
the independent review checks another 54 and 41 respectively per target.
Observed omission/spill failures preceded the mask corrections. Elsie's final
pass adds expression-specific eye fringes and restores two Summer bow folds.
Her cosmetics remain original. Errol's outer lips follow the target while his
mouth interiors and gray hair remain original. No concrete omission or spill
remains from the complete review; subjective cosmetic contrast is deferred.

Inputs are recorded in `tmp/elsie-errol-bundle-inputs.sha256`. Final recipe
hashes are `9caf795362c1a818fe02a2976ef606c52c1ef628ecb38616c8ecde6859184328`
for Elsie and `e8ee018286923edc1e2fca0380ac296bbfdcff949ca30c6887160e73e492c11b`
for Errol.

A fresh installation through the normal CLI into `tmp/elsie-errol-playtest`
verified every selected Vanilla and variant atlas frame, animation metadata,
and generated script before publishing the isolated archive. All nineteen
characters and their expected controls appear in the installation report.

The isolated game run checked all 72 new portraits across all five choices and
another 88 portraits covering every available outfit of the previous seventeen
characters: exactly 160 distinct sources and 800 source/preset pairs. Actual
Z/X keys, four-season wraps, independent selections, fractional portrait phase,
and a hidden debug console passed. The final complete character wrap retained
Elsie's and Errol's choices. The source/preset log audit matched exactly, with
no script errors or disabled callbacks. Blue/Ryis game screenshots and the
compact comparison PNG were inspected.

The harness returned zero after completing the entire matrix. It sent a quit
signal, then forcibly cleaned up the game after its one-second shutdown wait;
the game logged its quit request. Normal desktop exit was not tested.

The usual `tmp/play-characters` launcher was rebuilt through Nix and checked as
an executable shell script. It opens the verified `tmp/elsie-errol-playtest`
copy on Elsie. The source archive remains unchanged at SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Game checks used Xvfb and separate state. Desktop Wayland play and user visual
acceptance remain pending. Uninstall and the full Adeline world-action matrix
were not rerun for this portrait-data batch.

Local evidence includes `tmp/elsie-errol-final-checks.log`,
`tmp/elsie-errol-gml-checks.log`, `tmp/elsie-errol-integration-audit.json`,
`tmp/elsie-errol-native-portraits.json`,
`tmp/elsie-errol-all-variants-validation.log`,
`tmp/elsie-errol-previous-output-check.log`,
`tmp/elsie-errol-art-review-final.md`,
`tmp/elsie-errol-install-report.json`, `tmp/elsie-errol-live-run.log`, and
`tmp/elsie-errol-live-audit.json`. Recipe hashes match the inputs used for the
combined bundle and installation. The preceding Darcy/Dell checkpoint is
committed; this new slice remains uncommitted for user review. All game-derived
files remain ignored.
