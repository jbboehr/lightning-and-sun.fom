# Autumn specials and seasonal general actions

The user accepted the [preceding seasonal action batch](world-autumn-standard.md),
committed as `c6c60c8`. This slice adds 18 strips, 93 source frames and 49 native
West mirrors. The user approved this slice for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Autumn hammer, water, harvest, till, wipebrow and seated reading | 8 | 41 | 25 | 246 |
| Ryis | Winter general actions, sleep and kiss | 5 | 26 | 12 | 233 |
| Celine | Regular Autumn general actions, sleep and kiss | 5 | 26 | 12 | 304 |

Hayden now covers all thirty Autumn strips. Ryis covers twenty-two of thirty-three
Winter strips; Celine covers twenty-two of forty-six Autumn strips. Her fourteen
Autumn garden strips remain outside this batch. Their complete Spring and Summer
coverage and Ryis's complete Autumn coverage are retained.

F8, F10 and Insert keep their five choices and Vanilla defaults. In-game palette
selection is unchanged. Character material decisions and focused evidence are in
[Hayden](hayden-autumn-special.md), [Ryis](ryis-autumn-special.md) and
[Celine](celine-autumn-special.md).

## Updated game and installer

The newly mounted archive has SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
All 2,697 previously supported PNGs and all 2,697 raw metadata files are
byte-identical to the retained Autumn-standard baseline. No existing source
pins, regions, color groups or palette mappings were refreshed. Evidence:
`tmp/autumn-special-update-audit.{json,log}`.

MOMI v0.15.10 rejected the updated engine at three seam anchors: `monster_draw`,
`furniture_floor_sprite` and `obj_item_outline_sprite`. Installation stopped
before publication. The Nix shell now pins
[MOMI v0.16.4](https://github.com/Garethp/Mods-of-Mistria-Installer/releases/tag/v0.16.4),
whose release notes support game 1.0.5 and describe the removed monster draw hook.
This palette mod does not use that hook. The downloaded Linux CLI's SHA-256,
`0c36b78dd1ee964c604f3b54d9669a962bbbed1078ae07e0be8d7abc9f93895c`, matches the
release API digest. No installer checks were bypassed.

A fresh isolated install with the new pin passes strict lints, required game
compilation, and installed-pixel/metadata verification. The lab is
`tmp/world-autumn-special-playtest`; its installed archive SHA-256 is
`159c81d13976949462bda371e96af68304e2390a7cae5884a67f723bdc983e82`.
The mounted source and lab's preserved `previous.zip` retain the source hash above.
The live game and desktop launcher were not changed. Evidence:
`tmp/world-autumn-special-install-old-momi.log`,
`tmp/world-autumn-special-install-report.json`,
`tmp/world-autumn-special-source-after.sha256`.

## Offline review

- [Compact five-choice summary](../../generated/world-autumn-special-preview/summary.png):
  Hayden hammering East and Ryis/Celine's South general actions.
- [Complete Vanilla/Debug Blue review](../../generated/world-autumn-special-preview/index.html):
  all 142 new frame/direction cases, with paginated character reviews and separate
  five-choice summaries.

Every new source frame was inspected in all four targets. The compact summary
passes fifteen source/palette bindings, metadata, full-art crops and 473,280
exact-pixel comparisons. Character checks cover all frames, native West mirrors,
source/palette bindings and crops. Chromium verifies images, local links and
horizontal overflow on each review page and the combined landing page. Evidence:
`tmp/world-autumn-special-summary-check.log`,
`tmp/world-autumn-special-landing-browser.log` and the character preview records.

Static artwork does not establish gameplay schedules, automatic outfit changes,
separately drawn props or live rendering. Extracted art, previews, packages and
isolated installs stay ignored.

## Verification

The synthetic package regression first failed on the unregistered Hayden hammer
path, then passed after the eighteen exact paths were registered. It checks PNGs,
controls and complete native animation properties. Independent archive metadata,
author exports and current native NPC cycle configuration agree. All strips keep
80×80 frames, Default atlas and Middle/54 origin, with original frame durations.
Hayden's seated reading remains complex; Ryis/Celine's general actions retain
random final-frame holds of `[240,360]`. Evidence:
`tmp/world-autumn-special-package-{red,checks}.log`,
`tmp/world-autumn-special-inventory.log` and `native-inputs.json`.

Formatting, Clippy, all 107 active tests and the release build pass through the
updated pure Nix shell; 104 opt-in tests are ignored by the normal suite.
Separately, all three new character corpus tests, three retained `*_world` corpus
tests and three pinned Fabricator GML tests pass. Other historical opt-in corpus
and legacy replacement-install tests were not rerun. Both flake systems evaluate,
and `nix develop` exposes the expected executable digest. Evidence:
`tmp/world-autumn-special-final-checks.log`, `retained-world.log`, `gml-checks.log`,
`nix-check.log` and the character test logs (each with the same batch prefix).

All 10,860 variants validate across 2,715 sources and 36 characters. The combined
package matches all 3,132 standalone variants for these three characters. All
7,650 previous original/variant PNG and metadata files for these characters are
byte-identical. The other thirty-three characters retain all 19,320 PNG/metadata
files and 132 palette reports unchanged. Export reports have the new archive
hash; their previous file entries are identical. Earlier runtime rows, controls,
all 765 earlier region objects and palette sets are unchanged. The 45 retained
test edits only update fixture paths, ignore descriptions and three source counts.
Evidence: `tmp/world-autumn-special-{comparison.json,input-audit.log}`.

Probes execute the updated game's actual animator, pack constructors and NPC
`animate()` method with the shipped palette wrapper. All 142 direction/frame
cases pass across five choices: 710 palette observations, 125 linear completions,
80 hold-boundary checks and twenty seated-reading phase transitions. Palette
changes preserve fractional phase, loop counters, West reversal, portrait
selection and wrapper identity. Engine services and sprite information are
simulated; cycles and loop limits are selected directly. This does not exercise
natural schedules, the full NPC state machine, automatic outfit dispatch, audible
sound or live gameplay. Evidence:
`tmp/world-autumn-special-{hayden,ryis,celine}-runtime.{gml,log}`.

Eleven frozen palette/registry input hashes remain unchanged after builds and
installation (`tmp/world-autumn-special-frozen-{inputs.sha256,check.log}`).

## Next coverage

Continue the existing seasonal plan: finish Ryis's eleven Winter special strips
and Celine's ten regular Autumn specials. With Hayden's Autumn folder complete,
begin his Winter idle/walk pilot alongside those batches. Celine's fourteen
Autumn garden strips remain a separate pass. Folder evidence is
`tmp/world-autumn-special-remaining.json`.
