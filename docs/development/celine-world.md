# Celine's first overworld batch

This third small pilot follows the accepted Hayden and Ryis idle/walk batches.
It covers six normal Spring strips, containing 15 source frames. Native West
mirroring adds five direction/frame cases. Insert selects Vanilla, Debug Blue,
Hayden, Ryis or Seridia for Celine's portraits and these sprites together.
Other actions and outfits, including gardening, remain original in the overworld.

The extended `celine-world-trial.json` profile and set retain all 183 portrait
regions and add six world regions: 189 sources and 756 recolored variants. The
portrait-only definitions remain available. The combined trial contains
36 characters, 2,375 sources and 9,500 variants. Shared Rust and GML runtime code
is unchanged.

## Sources and masks

Sources are under `assets/animations/NPCs/Celine/Sprites/Spring/`, named
`spr_npc_celine_spring_{idle,walk}_{north,south,east}.png`. All have 80×80 frames,
Default atlas and Middle/54 origin. Idle uses the engine's single-frame defaults;
walking has four frames at duration `0.15`. The original
`assets/fiddle/npcs/celine.toml` declares linear cycles and North/South/East
directions, with separate garden outfits. This pilot does not include those
garden sprites.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local extraction: `extracted/celine-world-study`. Native metadata and cycle
declarations are recorded in `tmp/celine-world-metadata.json` and
`tmp/celine-world-native-npc.toml`.

The three added skin shades, `FCD9B3`, `F0B988` and `D37A57`, map to the first
three portrait target shades. They form a new connected-color group. The
existing eleven source colors, six groups, target values and all portrait region
objects remain unchanged. The new file uses compact seed formatting.

The shared `672115` outline stays in its existing separate group. Reviewed seeds
include its skin edges while excluding hair, belt and boots. Skin shading also
covers the tiny leg pixels above the boots in East walking frames two and four.
The orange `B65932` hair/belt/boot shade stays original throughout this pilot.
The 174 new seeds select 529 pixels per target: 390 in the three new shades and
139 outline pixels. Another 170 matching outline pixels remain original. All
15 frames were inspected in Vanilla and all four targets.

| Strip | Changed pixels per target |
| --- | ---: |
| Idle East | 45 |
| Idle North | 12 |
| Idle South | 52 |
| Walk East | 184 |
| Walk North | 38 |
| Walk South | 198 |

Profile SHA-256:
`2fe70b9d7849bfb25b2c67184c99a3e850285948ff5c30b6a9fde249eed6b6e1`.
Local authoring evidence: `tmp/celine-world-components.json`,
`tmp/celine-world-dark-components.json`, `tmp/celine-world-mask-decisions.json`
and the three `tmp/celine-world-refined-art-*.png` sheets.

## Offline review

- [Summary PNG](../../generated/celine-world-preview/summary.png): idle South,
  East and North in all five choices, about 49 KB.
- [Complete Vanilla/Blue review](../../generated/celine-world-preview/blue-review/index.html):
  all 15 source frames and five native West mirrors on three pages, at most
  eight cases per page. The complete review directory is about 392 KB.

Crop `[29,24,22,34]` contains every visible source pixel, including walking boots.
The summary uses 6× nearest-neighbor enlargement; the full review uses 10×.
Exact checks cover 2,992,000 full-review pixels, 403,920 summary pixels, all
palette/sample bindings, source metadata and West reversal without clipping.
Chromium decoded every image and verified all three pages, 20 cases and links.
Evidence: `tmp/celine-world-preview-pixel-check.log` and
`tmp/celine-world-preview-browser-check.json`. Static images do not show game
timing. The user accepted the offline artwork review.

## Verification

Formatting, Clippy, all 93 active tests, the release build, the local Celine
corpus test and three Fabricator GML tests passed. Evidence:
`tmp/celine-world-final-checks.log`. The final profile formatting was checked
against all 756 Celine variants in `tmp/celine-world-formatted-validation.log`;
five final build inputs are pinned in `tmp/celine-world-frozen-inputs.sha256`.

The package fixture first rejected the unregistered Celine world source, then
passed with the six registry entries, checking Insert, native geometry and
packaged PNG bytes. The local corpus test checks all four targets, every pixel's
permitted shade, complete coverage of the three new skin colors, alpha, metadata,
common selection and twelve literal skin/material landmarks. Evidence:
`tmp/celine-world-package-red.log` and `tmp/celine-world-focused-final.log`.
Merging the outline and new skin groups in an ignored negative-control copy
adds an unwanted boot pixel at East walk frame two `[39,51]`; the corpus test
rejects 185 changed pixels where 184 were reviewed. Evidence:
`tmp/celine-world-mixed-outline-red.log`.

The combined bundle preserves all 1,830 earlier Celine original/variant PNG and
metadata files and 22,035 files for the other 35 characters. All 756 Celine
variants match the inspected standalone output. Earlier table rows and controls
are unchanged, and all 9,500 combined variants pass exact validation. Evidence:
`tmp/celine-world-comparison.log` and `tmp/celine-world-all-variants.log`.

The focused native probe uses the original animator and NPC `animate` method
with the actual 189-row Celine table. It passes 20 direction/frame cases,
100 matrix palette observations, ten Spring idle/walk collection switches with
50 additional observations and ten last-frame loop boundaries. It checks West
facing, fractional/paused phase, native state, portrait synchronization and
Adeline's independent selection. Engine services are simulated and durations use
relative units; collections are selected directly. Natural schedules, automatic
outfit changes, full NPC state machines, speaking/background pause policy and
live gameplay were not exercised. Evidence: `tmp/celine-world-runtime.log` and
`tmp/celine-world-runtime-inputs.json`.

A fresh MOMI install in `tmp/celine-world-playtest` passed installed pixel,
metadata and table validation. Installed archive SHA-256:
`d38922802f73534e01c06998aa1a703a5d7f9fcd29a4fe9760973f862b1337e4`.
The source and retained `previous.zip` match the pristine hash. No preview helper
is installed and the desktop launcher is unchanged. An uninstall roundtrip and
live gameplay were not rerun. Evidence: `tmp/celine-world-install-report.json`,
`tmp/celine-world-install.log` and `tmp/celine-world-source-after.sha256`.

## Next parallel batch

After Celine's review, expand Hayden, Ryis and Celine with one author per
character, using their accepted idle/walk masks as references. The first bounded
expansion is normal Spring blink, sit, eat and drink: eleven additional strips
per character, confirmed in the source archive. Give each author ownership of
their character's profile, presets and local review; integrate shared registry
and combined-set changes centrally. Keep the compact summary and complete
Vanilla/Blue frame review for each character.

Celine's gardening outfits and Adeline's beach/wedding sprites remain queued.
Game files, generated artwork and isolated installs remain ignored.
