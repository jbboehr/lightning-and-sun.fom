# Ryis's first overworld batch

This second small pilot follows the accepted Hayden idle/walk batch. It covers
six Spring strips and 15 source frames, with five additional West views produced
by the native East mirror. F10 selects Vanilla, Debug Blue, Adeline, Hayden or
Seridia for Ryis's portraits and these world sprites together. His other actions
and outfits remain original.

The extended `ryis-world-trial.json` profile and set retain all 109 portrait
regions and add six world regions: 115 sources and 460 recolored variants.
The portrait-only definitions remain available. The combined trial now contains
36 characters, 2,369 sources and 9,476 variants. Shared Rust and GML runtime code
is unchanged.

## Sources and mask decisions

Sources are under `assets/animations/NPCs/Ryis/Sprites/Spring/`, named
`spr_npc_ryis_spring_{idle,walk}_{north,south,east}.png`. All have 80×80 frames,
Default atlas and Middle/54 origin. Idle omits frame count and duration, using
the engine's single-frame defaults. Walking has four frames at duration `0.15`.
The original `assets/fiddle/npcs/ryis.toml` declares linear idle/walk cycles with
North, South and East directions. Other declared actions are outside this pilot.

The read-only source archive is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Local extraction: `extracted/ryis-world-study`. The metadata inventory and native
NPC declaration are `tmp/ryis-world-metadata.json` and
`tmp/ryis-world-native-npc.toml`.

The world skin ramp is `B06C57`, `854D3C`, `63342A`, `491F1B`. Three shades
already occur in the portrait mapping; `854D3C` is added and maps to the same
target shade as the portrait's `814A3A`. All eight earlier source colors and
their target values remain unchanged. The added shade leaves the reviewed
portrait outputs byte-identical.

All four world shades occur on reviewed skin in these six pinned strips. The
83 component seeds cover 700 pixels per target: face, ears, neck, forearms and
exposed fingers around the gloves. Yellow gloves, red clothing, eyes and hair
stay original. In particular, the broad `5E423B` area at the back of his head is
retained as short hair. All 15 frames were inspected in Vanilla and all four
targets before producing the final review.

| Strip | Changed pixels per target |
| --- | ---: |
| Idle East | 51 |
| Idle North | 32 |
| Idle South | 58 |
| Walk East | 205 |
| Walk North | 124 |
| Walk South | 230 |

Profile SHA-256:
`d0e55ad3776e0975d148a8ff776483584441915db3d83b3f70eddc22973a4e7f`.
Local component inventory: `tmp/ryis-world-components.json`; inspected sheets:
`tmp/ryis-world-refined-art-{south,east,north}.png`.

## Offline review

- [Summary PNG](../../generated/ryis-world-preview/summary.png): idle South, East
  and North across all five choices, about 46 KB.
- [Complete Vanilla/Blue review](../../generated/ryis-world-preview/blue-review/index.html):
  all 15 source frames plus five West mirrors, split over three pages with at
  most eight cases each. The complete review directory is about 376 KB.

Crop `[29,24,22,34]` includes every visible source pixel, including walking boots.
The summary uses 6× nearest-neighbor enlargement; the full review uses 10×.
Exact checks cover 2,992,000 full-review pixels, 403,920 summary pixels, all
palette/sample bindings, native metadata and West reversal without clipping.
Chromium decoded every image and verified the three pages, 20 cases and links.
Evidence: `tmp/ryis-world-preview-pixel-check.log` and
`tmp/ryis-world-preview-browser-check.json`. These static images do not show
game timing. The user accepted the offline artwork review.

## Verification

Formatting, Clippy, all 91 active tests, the release build, the local Ryis corpus
test and three Fabricator GML tests passed. Full-check evidence:
`tmp/ryis-world-final-checks.log`. The five build inputs are pinned in
`tmp/ryis-world-frozen-inputs.sha256` and were rechecked after installation.

The portable package fixture first rejected the unregistered Ryis world source,
then passed after adding the six entries, checking F10, native geometry and
packaged PNG bytes. The local corpus test checks all four targets against every
world pixel, preserves alpha and sidecars, and checks ten explicit skin, glove
and hair landmarks. Removing the East idle finger seed in an ignored copy makes
that test fail at `[35,46]`, with the old brown finger where blue is expected.
Evidence: `tmp/ryis-world-package-red.log`, `tmp/ryis-world-focused-final.log` and
`tmp/ryis-world-missing-finger-red.log`.

The combined bundle preserves all 1,090 earlier Ryis original/variant PNG and
metadata files and 22,715 files for the other 35 characters. All 460 Ryis variants
match the inspected standalone output. Earlier table rows and controls are
unchanged, and all 9,476 combined variants pass exact validation. Evidence:
`tmp/ryis-world-comparison.log` and `tmp/ryis-world-all-variants.log`.

The focused native probe uses the original animator and NPC `animate` method
with the actual 115-row Ryis table. It passes 20 direction/frame cases,
100 matrix palette observations, ten Spring idle/walk collection switches with
50 additional observations and ten last-frame loop boundaries. It checks West
facing, fractional/paused phase, native state, portrait synchronization and
Adeline's independent selection. Engine services are simulated and durations
use relative units; collections are selected directly. Natural schedules,
automatic outfit changes, full NPC state machines, speaking/background pause
policy and live gameplay were not exercised. Evidence:
`tmp/ryis-world-runtime.log` and `tmp/ryis-world-runtime-inputs.json`.

A fresh MOMI install in `tmp/ryis-world-playtest` passed installed pixel,
metadata and table validation. Installed archive SHA-256:
`e7f4e2eb04b71387458531bf69b3454a0d633229785b091ae5a3c89cb0ce6317`.
The source and retained `previous.zip` match the pristine hash. No preview helper
is installed and the desktop launcher is unchanged. An uninstall roundtrip and
live gameplay were not rerun. Evidence: `tmp/ryis-world-install-report.json`,
`tmp/ryis-world-install.log` and `tmp/ryis-world-source-after.sha256`.

## Next pilot

Review Ryis, then make Celine's small Spring idle/walk batch locally. After all
three initial masks are accepted, expand larger batches with subagents.
Adeline's beach/wedding sprites remain queued. Game files, generated artwork
and isolated installs remain ignored.
