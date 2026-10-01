# Winter and Autumn movement and actions

Following the approved batch in `b87aa7d`, this slice adds 17 strips: 56 source
frames and 22 native West mirrors, giving 78 review cases. The user approved the
offline artwork on 2026-09-30.

| Character | New strips | Frames + mirrors | Total sources | Folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 6 | 15 + 5 | 207 | Winter 6/28 |
| Valen | 5 | 26 + 12 | 188 | Autumn 22/31 |
| Eiland | 6 | 15 + 5 | 172 | Autumn 6/41 |

The 36-character collection now has 3,752 sources and 15,008 variants. I, O and J
retain their five choices, Vanilla defaults and portrait/world synchronization.
Only registry/profile data, tests and documentation change; runtime Rust and GML
code remain unchanged.

## Source and material decisions

The mounted archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 550 previously covered PNG pins match fresh exports. No pin refresh or hash
mismatch override was used. Earlier regions, source roles, color groups and
target ramps remain unchanged. All 17 new sidecars match independent archive
reads, retaining 80×80 frames, the Default atlas and native Middle/54 origins.

Balor's Winter idle/walk recolor his visible face using the existing four world
skin shades. His gloves, scarf, coat, trousers, boots, eyes and hair remain
original. North idle and all four North walking frames expose no skin; both
strips have empty seed lists and remain PNG-byte-identical in all palettes.

Valen's Autumn general actions, sleep and kiss use her four world skin shades.
Moving hands and faces change while goggles, coat, cuffs, trousers, boots, eyes
and hair stay original. North action exposes one hand pixel in its first frame
and none in the remaining six. Literal landmarks and per-frame counts guard that
brief exposure. Action has seven frames per direction and the native last-frame
hold; sleep has one East frame and kiss has four, also mirrored West.

Eiland's Autumn idle/walk use his five world skin shades. His outfit shares
`#BA6A4C` with face and finger shadows. Thirty individually reviewed clothing
pixels are excluded by omitting their disconnected component seeds: one in idle
East, two in idle South, seven in walk East, eight in walk North and twelve in
walk South. Tests independently enumerate those coordinates, verify their source
color and require unchanged output. The remaining skin pixels recolor while
cape, gold trim, pink clothing, boots, eyes and hair stay original. Portrait-role
colors `#C9785A` and `#6A3126` also occur in this outfit and are left unselected.

Both idle/walk groups have one idle frame and four walking frames per native
North/South/East direction. Walking retains 0.15-second frames and East-to-West
mirroring.

## Offline review

- [Five-choice sample summary](../../generated/balor-winter-valen-standard-eiland-autumn-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-winter-valen-standard-eiland-autumn-preview/index.html).

The combined index is the entry point for all three galleries. They contain
every source frame and native West mirror, including the unchanged North views.
All game-derived images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 210 active tests and the release
build passed. The normal suite leaves 263 local opt-ins ignored. Forty-five local
tests passed separately: three new material tests, thirty-nine retained corpus
tests and three GML runtime tests. Six omission/spill controls generated successfully
before failing their intended pixel assertions. Eiland's spill control specifically
reintroduces a shared-color clothing seed. The synthetic package test rejected an
unregistered Balor Winter idle strip before the additions and passed afterward.

All 15,008 combined variants validate. All 2,268 standalone character variants
pass strict recipe validation and match the combined output. All 5,500 prior
original/variant PNG and metadata files for these characters and all 32,015 files
for the other 33 characters remain byte-identical. Previous runtime rows, choices
and hotkeys remain exact. The thirty-nine retained tests change only corpus paths
and source totals.

The current archive's native animator and NPC object pass 390 frame/palette
observations, 120 linear completions and 40 last-frame hold checks using simulated
engine services and direct cycle selection. Checks cover mirroring, palette binding,
wrapper idempotence, frame phase, cycle counters, completion state and portrait
synchronization. Natural scheduling, interaction/outfit dispatch, attached effects
and full-engine rendering remain untested.

All 35 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks cover every case, native mirrors, metadata and complete opaque crops,
plus all fifteen source/palette bindings in the combined summary. The four preview
directories total 1,543,340 bytes (about 1.5 MiB).

MOMI installation passed in the fresh isolated
`tmp/bve-winter-autumn-standard-world-b87aa7d-playtest` copy, including required
compilation and installed-pixel/animation-metadata verification. Installed archive:
`411a99f2a1dc42cb25646be6811d3bb3172cc37e97b2b63628d036d05538006c`.
The mounted archive and lab's original backup retain the source hash above; all
eleven registry/recipe inputs remained unchanged through generation and installation.
No preview helper was installed or desktop launcher changed. An uninstall roundtrip,
other local opt-ins and live gameplay were not rerun.

Evidence uses `tmp/bve-winter-autumn-standard-world-`. Author source records use
`tmp/balor-winter-world-author-`, `tmp/valen-autumn-standard-author-` and
`tmp/eiland-autumn-world-author-`. Fresh corpora use the corresponding
`extracted/<character>-<batch>-study` directories. Eiland's `author-clothing.json`
records the thirty excluded clothing pixels; the tracked test retains those
coordinates independently of recipe seeds.
The `evidence.json` report summarizes test, comparison, gallery and installation
logs; `scope.log` verifies that only the expected 51 paths changed and the Git index
is empty. Game-derived artifacts remain ignored.

## Next coverage

Continue Balor's Winter and Eiland's Autumn blink/sit/eat/drink. Valen's nine
remaining Autumn strips are seated reading, standing writing and healing.
