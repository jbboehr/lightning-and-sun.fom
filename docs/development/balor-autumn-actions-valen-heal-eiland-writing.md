# Balor's Autumn actions, Valen's Summer healing and Eiland's Summer writing

Following the approved Autumn pilot and Summer actions, this batch adds 23
strips: 63 source frames and 18 native West mirrors, giving 81 review cases.
The user approved the offline artwork on 2026-09-30. Live gameplay was not rerun.

| Character | New strips | Frames + mirrors | Total sources | Current folder coverage |
| --- | ---: | ---: | ---: | --- |
| Balor | 11 | 31 + 12 | 190 | Autumn 17/28 |
| Valen | 3 | 6 + 6 | 166 | Summer 34/34, complete |
| Eiland | 9 | 26 + 0 | 156 | Summer 31/41 |

The complete 36-character collection now has 3,697 sources and 14,788 variants.
I, O and J retain their five choices, Vanilla defaults and portrait/world palette
synchronization. No runtime Rust or GML code changes are needed.

## Sources and material decisions

The mounted archive SHA-256 remains
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 489 earlier source PNG pins match fresh exports; no mismatch override or pin
refresh was used. All prior regions, source roles, color groups and target ramps
are retained. The new raw metadata matches independent archive reads: 80×80
frames, Default atlas and the native Middle/54 or numeric 40/54 origin.

Balor's Autumn blinking, sitting, eating and drinking use the existing four
world skin shades. Faces, necks and moving hands recolor while scarf, clothing,
cup, food, eyes and hair stay original. North, South, East and native West mirrors
are included where the original cycles support them.

Valen's three Summer healing strips finish that folder. The existing four world
skin shades cover face, neck, exposed arms, fingers and sandal skin. Gold goggles,
the amber/blue healing prop, pink shirt, trousers and sandal straps stay original.
The complex East cycle uses 1/4/1 start/loop/end frames and mirrors West. Skin
counts are 70 for each start/end and 59, 57, 57, 59 for the loop.

Eiland's seated reading and standing/seated writing use the existing five world
skin shades. Face, neck and moving fingers recolor while books, orange quill,
brown writing surface, gold trim, clothing, eyes and hair stay original. The
warm quill colors are materials, as in the reviewed Spring writing strips.
Reading has 3/4/3 frames; both writing cycles have 2/4/2. All face South, with no
West mirrors. Unlike his Spring uniform, this Summer artwork needs no shared
skin-color clothing exclusions.

Local source inventories, raw sidecars, per-frame skin counts, candidate recipes
and material landmarks use the prefixes `tmp/balor-autumn-actions-author-`,
`tmp/valen-summer-finish-author-` and `tmp/eiland-summer-writing-author-`.
Fresh complete character corpora are under the corresponding
`extracted/<character>-<batch>-study` directories.

## Offline review

- [Five-choice sample summary](../../generated/balor-autumn-actions-valen-heal-eiland-writing-preview/summary.png).
- [Complete Vanilla/Debug Blue review](../../generated/balor-autumn-actions-valen-heal-eiland-writing-preview/index.html).

The combined index links all three character galleries and covers the entire
batch, including every new source frame and native West mirror. The compact
summary contains samples; each character also has a five-choice sample sheet.
All images and packages remain local and ignored by Git.

## Verification

Formatting, Clippy with warnings denied, all 207 active tests and the release
build passed. The normal suite leaves 254 local opt-ins ignored. Thirty-six local
tests passed separately: three new material tests, thirty retained corpus tests
and three GML runtime tests. Six deliberate omission/spill controls generate
successfully and then fail the intended pixel assertions. The synthetic package
test rejected unregistered Balor Autumn blink East before the registry additions,
then passed with every new animation's properties, pixels, frame counts and hotkeys.

All 14,788 combined variants validate; the 2,048 standalone character variants
also pass strict recipe validation and match the combined output. All
4,890 prior original/variant PNG and metadata files for these characters and
all 32,015 files for the other 33 characters remain byte-identical. Previous
runtime rows and hotkeys remain exact. The thirty retained tests change only
their input corpus paths and total source counts.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 405 frame/palette observations, 75 linear completions,
80 last-frame hold checks and 100 complex-phase transitions. These exercise
native direction/mirroring, palette binding, wrapper idempotence, frame phase,
cycle counters, seated flags, completion state and portrait synchronization.
Natural scheduling, interaction/outfit dispatch, attached effects and full-engine
rendering were not exercised.

All 40 review pages pass Chromium link/image loading and overflow checks. Exact
pixel checks bind all 81 cases to the current artwork, metadata and native mirrors;
the full crops contain every opaque pixel. The combined summary checks all fifteen
source/palette bindings independently. The four preview directories total
1,586,356 bytes (about 1549 KiB).

MOMI installation passed in the fresh isolated
`tmp/bve-autumn-actions-summer-finish-writing-0ef5c3f-playtest` copy, including required
compilation and installed-pixel/animation-metadata verification. Installed archive:
`55f89590f57701812e845c03eb48404afed6a33a3d1a25335955afa65346392d`.
The mounted source archive and lab's original backup retain the source hash above;
all eleven registry/recipe inputs remained unchanged through generation and
installation. No preview helper was installed or desktop launcher changed.
An uninstall roundtrip, other local opt-ins and live gameplay were not rerun.

Evidence is under `tmp/bve-autumn-actions-summer-finish-writing-`: `evidence.json`,
`final-checks.log`, `focused.log`, `{balor,valen,eiland,gml}-local.log`,
`comparison.{json,log}`, `input-audit.log`, `metadata-check.log`, `runtime.log`,
`{balor,valen,eiland}-final-output.log`, `summary-check.log`,
`landing-browser.log`, `install-report.json`, `frozen-check.log` and
`source-after.sha256`. Character `author-{omission,spill}-red.log` files record
the negative controls. Only registry/profile data, synthetic/local tests and
documentation changed; no game-derived images or packages enter Git.

## Next coverage

Balor can continue with Autumn general actions, sleep and kiss. Valen can start
Autumn idle/walk now that Summer is complete. Eiland can continue with the six
Summer magnifying-glass strips; his four tool strips would then remain.
The exact current-folder inventory is in
`tmp/bve-autumn-actions-summer-finish-writing-remaining.json`.
