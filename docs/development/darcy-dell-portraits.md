# Darcy and Dell portrait batch

The later [Dell eye and ear correction](dell-portraits.md#eye-and-ear-correction)
updates the current trial and summary. The original batch record below remains
the baseline for that bounded pixel comparison.

This batch adds all 32 Darcy and 32 Dell portrait strips in the supplied archive:
eight expressions in each of Spring, Summer, Autumn, and Winter. Each strip has
two 296×180 frames. Both characters have Vanilla, Debug Blue, Hayden, Ryis, and
Seridia choices, with Vanilla selected at launch. B cycles Darcy; Y cycles Dell.
The shared Rust and GML runtime is unchanged.

The complete collection has 1,492 source strips and 5,968 recolored variants
across seventeen characters. That includes 1,475 portraits and Adeline's 17
previously reviewed world animations. The previous characters keep their
accepted recipes, including Holt's lower-mouth correction. Josephine's
palette-specific lipstick contrast remains deferred.

The archive metadata puts every Darcy portrait in `PortraitsMisc`, despite
their seasonal directories. Dell uses the matching four seasonal atlases.
The registry and local trial use those actual atlas names. All 64 filenames
also match the outfits and expressions in the game's native NPC definitions.

Character-specific mask details are in [Darcy portraits](darcy-portraits.md)
and [Dell portraits](dell-portraits.md).

Darcy uses ten skin shades, including lighter arms and fine eye, ear, and neck
shading. Her hair, gold ornaments, eye details, and original cosmetics are
preserved. Dell uses twelve shades, with explicit separation of skin from
hair, bandages, buttons, and Winter clothing that borrow the same colors.

## Local visual check

The compact comparison is `generated/darcy-dell-preview/summary.png`, with
`index.html` beside it. The combined bundle is
`generated/characters-darcy-dell-trial`. Images, packages, extracted assets,
and trial game copies stay ignored and local.

The `tmp/play-characters` launcher uses `tmp/darcy-dell-playtest` with its own
saves and state, mounting the supplied game read-only. F7 opens on Darcy;
F4 moves to Dell. B and Y cycle their palettes. F5 changes season; F7 and
Page Up move through expressions. F9 checks independent selections and
fractional portrait phase. The controls are local test helpers excluded from
the player package.

## Verification

Run the normal checks in [verification](verification.md), plus the local corpus
tests when the source exports are available:

```sh
nix-shell --pure --run 'cargo test --locked --test darcy_portraits --test dell_portraits -- --ignored --nocapture'
```

The synthetic collection test first failed with `Unsupported character: darcy`.
After registry integration, all four character tests passed. The fixture also
covers Darcy's seasonal paths with `PortraitsMisc`, standalone builds, the
complete seventeen-character collection, B/Y controls, Vanilla-first choices,
generated pixels, and differing preset counts among existing characters.

Formatting, Clippy, all 87 active tests, both new local corpus tests, and the
release build passed. The three local GML interpreter checks also passed.
All 5,968 final variants passed exact recipe validation. The previous fifteen
characters' entire output trees, including originals, variants, metadata, and
reports, matched `generated/characters-holt-mouth-fix-trial` byte-for-byte.

The independent archive audit checked the complete PNG/metadata inventories,
source hashes and exported bytes, exact original atlases, dimensions and frame
counts, unique registry paths, profile coverage, preset arity, standalone Blue
equivalence, and catalog shade roles. All native outfit/expression pairs match.

A fresh MOMI installation into `tmp/darcy-dell-playtest` verified every selected
Vanilla and variant atlas frame, animation metadata, and generated script before
publishing the isolated archive. The inputs are recorded in
`tmp/darcy-dell-bundle-inputs.sha256`.

The independent art review passes both characters. All 128 unique full frames
and enlarged Source/Blue/Ryis face comparisons were inspected. Exact color,
selection, source-hash, alpha, and metadata checks passed all 256 target strips,
with 44 independent Darcy and 80 Dell landmarks per target.

| Character | Source shades | Component seeds | Changed pixels per target | Protected matching pixels |
| --- | ---: | ---: | ---: | ---: |
| Darcy | 10 | 12,264 | 64,432 | 392 |
| Dell | 12 | 9,044 | 61,148 | 30,092 |

There are no identity-color exceptions in these four targets. The repository
tests use 41 Darcy and 139 Dell literal source landmarks. Observed
omission/spill failures preceded the corrections. Dell's final correction adds
exactly 672 neck pixels per target across Spring and Summer, with every other
pixel and all metadata unchanged from the preceding reviewed candidate. The
affected final neck views were rechecked. No concrete omission or spill remains;
palette-specific cosmetic contrast is deferred.

An initial candidate game run checked all 64 new portraits across all five
choices and another 80 portraits covering every available outfit of the previous
fifteen characters: 144 distinct sources and 720 source/preset pairs. Actual B/Y
keys, four-season wraps, independent selections, fractional portrait phase, and
a hidden debug console passed. The source/preset log audit matched exactly,
and the game logged a graceful exit. This run predates only Dell's final neck
correction; the previous fifteen characters' generated outputs remain identical.

After the neck correction, a second fresh MOMI installation verified the final
archive. Its focused game run passed every new portrait again: exactly 64
sources and 320 source/preset pairs, with B/Y cycles, season wraps, all seventeen
independent selections, fractional phase, and an unobscured preview. A full
character wrap retained Darcy's and Dell's choices. The final compact PNG and
Dell's Ryis game screenshot were inspected. The log contains no script errors
or disabled callbacks. The wrapper returned 143 during intentional shutdown
after completing the matrix; the game logged its quit request and graceful exit.

The usual `tmp/play-characters` launcher was rebuilt through Nix and checked as
an executable shell script. It opens the verified `tmp/darcy-dell-playtest` copy
on Darcy. The source archive remains unchanged at SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
These game checks used Xvfb and separate state. Desktop Wayland play and user
visual acceptance remain pending. Uninstall and the full Adeline world-action
matrix were not rerun for this portrait-data batch.

Local evidence includes `tmp/darcy-dell-final-checks.log`,
`tmp/darcy-dell-gml-checks.log`, `tmp/darcy-dell-integration-audit.json`,
`tmp/darcy-dell-native-portraits.json`,
`tmp/darcy-dell-all-variants-validation.log`,
`tmp/darcy-dell-previous-output-check.log`,
`tmp/darcy-dell-art-review-final.md`,
`tmp/darcy-dell-art-review-neck-delta.json`,
`tmp/darcy-dell-install-report.json`,
`tmp/darcy-dell-candidate-live-audit.json`,
`tmp/darcy-dell-final-live-run.log`, and
`tmp/darcy-dell-final-live-audit.json`. Recipe hashes match the inputs used for
the final combined bundle and installation. Nothing was staged or committed
during this slice; all game-derived files remain ignored.

The user accepted this prototype checkpoint after Dell's eye/ear correction.
Darcy's lip-highlight contrast remains deferred. Fresh formatting, Clippy,
active tests, both local portrait tests, and the release build were checked for
the commit in `tmp/darcy-dell-commit-checks.log`; installation and live gameplay
were not repeated for the commit.
