# Winter movement and seasonal actions

Following the approved batch in `6b9ca12`, this slice adds 16 strips: 67 source
frames and 29 native West mirrors, giving 96 review cases. The user approved
the offline artwork on 2026-09-30. Numbered output names start here; the latest completed slice is
also available through `generated/review/` and `generated/build/`.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 5 | 26 + 12 | 223 | Winter 22/28 |
| Valen | 6 | 15 + 5 | 203 | Winter 6/31 |
| Eiland | 5 | 26 + 12 | 188 | Autumn 22/41 |

The 36-character collection has 3,799 sources and 15,196 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change.

## Source and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 598 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 16 sidecars match independent archive reads,
retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Winter general actions, sleep and kiss recolor visible faces with his
existing four world skin shades. Gloves, scarf, coat, trousers, boots, eyes and
hair remain original. All seven North action frames expose no skin: that strip
has no seeds and remains PNG-byte-identical in every palette.

Valen's Winter idle/walk use her existing four world skin shades for faces and
visible hands. Coat, cuffs, trousers, boots, goggles, eyes and hair stay original.
The coat covers all skin in North idle and walking: both strips have empty seed
lists and their five frames remain PNG-byte-identical in every palette. Walking
retains its four native frames, 0.15-second durations and East-to-West mirroring.

Eiland's Autumn general actions, sleep and kiss use his existing five world
skin shades. Thirty-four reviewed clothing pixels share `#BA6A4C` with skin:
17 in East action, 12 in South action, one in kiss and four in sleep. Their
disconnected components stay unselected. Tests independently enumerate these
coordinates and require their source colors and unchanged output. Moving hand
shadows remain selected. North action briefly exposes 6/0/1/0/1/3/4 skin pixels
across its seven frames. Cape, gold trim, pink clothing, boots, eyes and hair
stay original, including outfit colors `#C9785A` and `#6A3126`.

Both general-action groups retain seven frames per native direction and the
last-frame hold. Sleep has one East frame and kiss has four; both mirror West.

## Offline review

- [Five-choice sample summary](../../generated/slice-001-review/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/slice-001-review/index.html).

The combined index links the three `slice-001-<character>-review/` galleries.
Every new source frame and native West mirror appears, including covered views.
All four review directories total 1,800,400 bytes (about 1.7 MiB). Their images
come from the current generated bundle and remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 212 active tests and the release
build passed; the normal suite leaves 269 local opt-ins ignored. Fifty-one local
tests passed separately: three new material tests, forty-five retained corpus
tests and three GML runtime tests. Six omission/spill controls generated before
failing their intended pixel assertions. Eiland's spill control reintroduces a
shared-color clothing seed. The synthetic package test rejected an unregistered
Winter general-action strip before the additions and passed afterward.

All 15,196 combined variants validate. All 2,456 standalone variants pass strict
recipe validation and match the combined build. All 5,980 prior original/variant
PNG and metadata files for these three characters and all 32,015 files for the
other 33 remain byte-identical. Previous runtime rows, choices and hotkeys remain
exact. The forty-five retained tests change only corpus paths and source totals.

The current archive's native animator and NPC object pass 480 frame/palette
observations, 120 linear completions and 80 last-frame hold checks using simulated
engine services and direct cycle selection. Checks cover mirroring, palette
binding, wrapper idempotence, frame phase, cycle counters, completion state and
portrait synchronization. Natural scheduling, interaction/outfit dispatch,
attached effects and full-engine rendering remain untested.

All 39 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque
crops, plus all fifteen source/palette bindings in the combined summary.
The `review` shortcut also passes Chromium loading; all 36 linked HTML pages
and 71 image references resolve through the short path.

MOMI installation passed in the isolated `tmp/slice-001-playtest` copy, including
required compilation and installed-pixel/animation-metadata verification. The
mounted archive and the lab's original backup retain the source hash above;
eleven registry/recipe inputs remain unchanged through generation and
installation. No preview helper was installed or desktop launcher changed.
An uninstall roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/slice-001-`. Author records use
`tmp/balor-winter-standard-author-`, `tmp/valen-winter-world-author-` and
`tmp/eiland-autumn-standard-author-`; fresh corpora use the corresponding
`extracted/<character>-<batch>-study` directories. Eiland's `author-clothing.json`
records the 34 protected clothing pixels, retained in the tracked material test.
`evidence.json` summarizes tests, comparisons, galleries and installation, including
the installed archive hash. `scope.log` checks the expected 57 paths and empty index.

## Next coverage

Finish Balor's Winter reading and gem inspection. Continue Valen's Winter
blink/sit/eat/drink and Eiland's Autumn seated reading and standing writing.
