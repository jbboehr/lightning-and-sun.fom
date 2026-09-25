# Winter expansion and regular Autumn specials

The user accepted the [preceding Autumn special batch](world-autumn-special.md),
committed as `b8cf302`. This slice adds 27 strips, 93 source frames and 21 native
West mirrors. The user approved this slice for commit on 2026-09-25.

| Character | New coverage | Strips | Source frames | West mirrors | Total sources |
| --- | --- | ---: | ---: | ---: | ---: |
| Hayden | Winter idle and walk | 6 | 15 | 5 | 252 |
| Ryis | Remaining Winter tool, seated, reading and writing specials | 11 | 37 | 12 | 244 |
| Celine | Regular Autumn reading, sweeping and watering specials | 10 | 41 | 4 | 314 |

Ryis now covers all 33 Winter strips. Celine covers all 32 regular Autumn strips;
her fourteen Autumn garden strips remain outside this batch. Hayden covers six
of thirty Winter strips. All earlier coverage and palette mappings are retained.
F8, F10 and Insert keep their five choices and Vanilla defaults. No runtime
Rust or GML behavior changed. Character material decisions and focused checks
are in [Hayden](hayden-winter-expansion.md), [Ryis](ryis-winter-expansion.md) and
[Celine](celine-winter-expansion.md).

## Source and metadata

The mounted source archive still has SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`,
identical to the preceding batch's complete archive. New source pins were added;
no existing hash was refreshed. Generation uses strict hash checks, without
`--allow-source-hash-mismatch`.

Independent archive metadata, author sidecars and native NPC cycle configuration
agree. All strips retain 80×80 frames and Default atlas. Most retain Middle/54
origin; Ryis's eight non-reading strips retain numeric `[40.0,54.0]`. The
synthetic package regression checks complete properties, frame counts, output
PNGs and controls. It first failed on Hayden's unregistered Winter idle path,
then passed with the 27 paths registered. Evidence:
`tmp/world-winter-expansion-package-{red,checks}.log`,
`tmp/world-winter-expansion-inventory.log` and
`tmp/world-winter-expansion-native-inputs.json`.

## Offline review

- [Compact five-choice summary](../../generated/world-winter-expansion-preview/summary.png):
  Hayden walking, Ryis reading seated and Celine reading standing.
- [Complete Vanilla/Debug Blue review](../../generated/world-winter-expansion-preview/index.html):
  all 114 new frame/direction cases, paginated by character, plus each character's
  five-choice summary.

Every source frame was inspected in all four targets. The complete reviews also
include every native West mirror. The combined summary passes fifteen source
bindings, metadata and full-art crop checks, and 422,400 exact output pixel
comparisons. Each character's complete review passes source/target, crop, West
reversal and exact pixel checks. Chromium decoded images, checked local links
and found no horizontal overflow on each review page and the combined landing.
Evidence: `tmp/world-winter-expansion-summary-check.log`,
`tmp/world-winter-expansion-landing-browser.log` and the character preview logs.

Static previews do not establish natural NPC
schedules, automatic outfit changes, separately drawn props or live rendering.
Extracted art, previews, packages and isolated installs remain ignored.

## Integration and installation

Formatting, Clippy, all 114 active tests and the release build pass through the
pure Nix shell; 107 opt-in tests are ignored by the normal suite. Separately,
all three new character corpus tests, three retained `*_world` corpus tests
and three pinned Fabricator GML tests pass. Other historical opt-in corpus
tests, the legacy replacement-install test and live gameplay were not rerun.
Evidence: `tmp/world-winter-expansion-final-checks.log`,
`tmp/world-winter-expansion-retained-world.log`,
`tmp/world-winter-expansion-gml-checks.log` and the character test logs.

All 10,968 variants validate across 2,742 sources and 36 characters. The combined
bundle matches all 3,240 standalone variants for these three characters. Their
7,830 previous original/variant PNG and metadata files are byte-identical;
all 19,485 files for the other thirty-three characters are byte-identical,
including reports. Previous runtime rows and controls, all 783 earlier region
objects, color groups and palette mappings are unchanged. The 48 retained test
edits only update fixture paths, ignore descriptions and three source counts.
Evidence: `tmp/world-winter-expansion-{comparison.json,input-audit.log}`.

Probes execute the current archive's actual animator, sprite-pack constructors
and NPC `animate()` method with the shipped palette wrapper. All 114 cases pass
across five choices: 570 palette observations, ninety linear completions and
one hundred complex phase transitions for reading, writing and sweeping.
Palette changes preserve fractional phase, loop counters, West reversal,
portrait selection and wrapper identity. Engine services and sprite information
are simulated; cycles and loop limits are selected directly. This does not
exercise natural schedules, the full NPC state machine, automatic outfit
dispatch, audible sound or live gameplay. Evidence:
`tmp/world-winter-expansion-{hayden,ryis,celine}-runtime.{gml,log}`.

A fresh isolated install passes MOMI v0.16.4 strict lints, required compilation
and installed-pixel/metadata verification. The lab is
`tmp/world-winter-expansion-playtest`; its installed archive SHA-256 is
`6bd591d9fddfea821446c38eb4d6c87288ac0421dfc397d9a8c6c68d718dc44b`.
The mounted source and lab's preserved `previous.zip` retain the source hash
above. The live game and desktop launcher were not changed. Evidence:
`tmp/world-winter-expansion-install-report.json` and
`tmp/world-winter-expansion-source-after.sha256`.

Eleven frozen palette/registry input hashes remain unchanged after generation
and installation (`tmp/world-winter-expansion-frozen-{inputs.sha256,check.log}`).

## Next coverage

Continue the existing seasonal plan with Hayden's Winter blink/sit/eat/drink and
Celine's fourteen Autumn garden strips. Ryis's Winter folder is complete; his
next outfit can be selected separately. Folder evidence is
`tmp/world-winter-expansion-remaining.json`.
