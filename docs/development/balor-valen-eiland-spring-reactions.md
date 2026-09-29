# Balor, Valen and Eiland: Spring shocked reactions and seated reading

Following the approved action, sleep and kiss batch (`ab3c665`), this slice adds
six strips per character: the start, loop and end phases of shocked reactions
and seated reading. Eighteen strips contain 39 source frames. Both cycles face
South only, with no native West mirrors. The user approved the offline artwork on 2026-09-29.

| Character | Total sources | Spring folder coverage | Material notes |
| --- | ---: | ---: | --- |
| Balor | 138 | 28/34 | [Balor](balor-spring-reactions.md) |
| Valen | 120 | 28/40 | [Valen](valen-spring-reactions.md) |
| Eiland | 106 | 28/47 | [Eiland](eiland-spring-reactions.md) |

The combined 36-character trial has 3,549 sources and 14,196 variants. I, O and
J retain their five choices and Vanilla defaults, synchronizing portraits and
supported overworld sprites. Other actions and outfits remain original. No
runtime Rust or GML behavior changed.

Existing color roles cover the new skin. The masks preserve each book's cover
and pages, mouth interiors, clothing and accessories. Balor retains 26 trouser
pixels that share a portrait skin color, and Eiland retains 32 shared-color
trim pixels. Valen needs no new material exclusions. Exposed fingers beside
the opening and closing books recolor; the character notes record the detailed
material checks.

## Sources and native behavior

The mounted read-only archive retains SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Source pins remain strict; no prior pin was refreshed or mismatch override used.
All eighteen strips retain 80×80 frames, Default atlas and Middle/54 origin.
Each shocked phase uses a single frame with native defaults. Reading start/end
have three frames at 0.1 seconds; its four-frame loop uses
`[3.0,0.1,3.0,0.1]` seconds. Both cycles use native start/loop/end transitions,
and reading retains its seated flag. Native animator and NPC object sources
match the preceding slice byte for byte.

The shared synthetic package test first rejected unregistered Balor reading
end, then passed after all eighteen paths were registered. It checks complete
animation properties, frame counts, PNG outputs and existing hotkeys. Evidence:
`tmp/bve-spring-reactions-package-{red,green}.log`,
`tmp/bve-spring-reactions-{inventory,timing-check,native-source}.log`.

## Offline review

- [Compact five-choice summary](../../generated/balor-valen-eiland-spring-reactions-preview/summary.png).
- [Complete Vanilla/Debug Blue gallery](../../generated/balor-valen-eiland-spring-reactions-preview/index.html):
  all 39 source frames, grouped by character, with individual five-choice summaries.

The combined HTML index is the entry point for this entire batch. Individual
character preview folders are linked parts of the same review, regardless of
folder timestamps. The summary PNG contains samples; the linked galleries
cover every source frame.

Static previews cover recoloring. Natural schedules, interaction and outfit
dispatch, pause policy and live rendering still need gameplay checks.
Game-derived files, previews and packages remain ignored.

## Verification

Formatting, Clippy with warnings denied, all 200 active tests and the release
build passed. The standard suite leaves 233 local opt-ins ignored; fifteen
local tests passed separately: three new material tests, nine retained corpus
tests and three GML runtime tests. Character notes record the material checks
and omitted-skin/material-spill controls. Other opt-ins and live gameplay were
not rerun. Evidence: `tmp/bve-spring-reactions-{final-checks,gml}.log` and the
character-focused logs.

All 14,196 combined variants pass exact recipe validation; all 1,456 standalone
Balor/Valen/Eiland variants match the combined output. All 3,460 earlier original
and variant PNG/metadata files for these characters remain byte-identical, as do
all 32,015 files for the other 33 characters. Previous runtime rows and hotkeys
remain exact. Evidence: `tmp/bve-spring-reactions-comparison.{json,log}`.

The definition audit preserves all 346 prior regions, source-color roles, groups
and target mappings. Sets, portrait-only files and the collection are unchanged.
The nine retained tests change only corpus paths and totals; their earlier
material assertions remain exact. Exactly eighteen registry/profile paths were
added. All 36 export reports retain their earlier source entries and archive
hash. All eighteen author raw sidecars match independent archive reads.
Evidence: `tmp/bve-spring-reactions-{input-audit,metadata-check,timing-check}.log`.

With simulated engine services and direct cycle selection, the current native
animator and NPC object pass 195 frame/palette observations and 120 start/loop/end
transitions across all five choices. They check palette binding, wrapper
idempotence, frame phase, cycle counters, seated flags, completion state and
portrait synchronization. Only South is exercised for these cycles. Natural
schedules, interaction/outfit dispatch, pause policy, attached effects and
full-engine rendering were not exercised. Evidence:
`tmp/bve-spring-reactions-{native-source,runtime}.log`.

Each complete gallery passes source/palette/frame, crop, pixel and Chromium
checks; details are in the character notes. The combined summary passes fifteen
source/palette bindings and exact displayed-pixel checks against raw metadata
and complete artwork crops. The combined landing loads all local links/images
without horizontal overflow. The four preview directories total
912,824 bytes (about 891 KiB). Evidence:
`tmp/bve-spring-reactions-{summary-check,landing-browser}.log`.

MOMI installation passed in a fresh isolated `tmp/bve-spring-reactions-ab3c665-playtest`
copy, including required compilation and installed-pixel/animation-metadata
verification. Installed archive SHA-256:
`817e03487430236830792871a8c1a1d43c4d669a3e2a3a5f7bd7f389dfdf6173`.
The mounted archive and lab's `previous.zip` retain the source hash above.
All eleven frozen registry/recipe inputs remained unchanged through generation
and installation. Evidence:
`tmp/bve-spring-reactions-{install-report.json,install.log,frozen-check.log,source-after.sha256}`.
No preview helper was installed and the desktop launcher remains unchanged.
An uninstall roundtrip and live gameplay were not rerun.

## Next coverage

Continue character-specific Spring work and special animations: Balor's coin,
gem and hair gestures, Valen's writing and medical work, and Eiland's writing,
poses and archaeology. Remaining archive-derived inventories are in
`tmp/bve-spring-reactions-remaining.json`.
