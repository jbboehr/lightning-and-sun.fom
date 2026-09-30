# Balor's Summer reading, coin flip and gem inspection

7 strips add 42 source frames and 0 native West mirrors: 42 review cases.
Balor now has 173 sources and 692 variants, covering 29/29 Summer strips.
The user approved the offline artwork on 2026-09-29.

The fresh source paths are under `assets/animations/NPCs/Balor/Sprites/Summer/`.
Every PNG has a strict hash pin and dimensions in the tracked profile. Raw
metadata is unchanged. The [shared notes](balor-summer-finish-valen-eiland-actions.md) record archive provenance,
cycle types, direction coverage and native timing.

The 1,356 new component seeds select 1,884 skin pixels per target.
Every source frame was inspected in Vanilla and Debug Blue; five-choice
summaries cover representative poses. Existing world roles suffice, with no
new color roles or material exceptions.

Faces, necks, hands and fingers change. Hair, eyes, pale and purple clothing,
belt/trousers/shoes, coin gold, blue/cyan gem and white glints, and book pages
and binding remain original. All 236 occurrences of portrait shade `#612026`
in these strips are belt/trouser material and remain excluded.

- [Five-choice summary](../../generated/balor-summer-finish-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/balor-summer-finish-preview/blue-review/index.html)

The complete gallery checks all 42 cases against original frame metadata,
complete artwork crops, palette bindings and native mirroring. Saved-image
checks compare 7,687,900 exact displayed pixels. Chromium checks all 8 HTML
pages, local links, loaded images and horizontal overflow. Final variant PNGs
and metadata match the inspected candidate byte for byte.

The focused test checks 283 literal source-grid landmarks, every pixel in every
target, per-frame skin counts, alpha, metadata and all earlier outputs.
Both negative controls fail their intended pixel assertion after successful
generation: omission at strip [39,40]; spill at strip [37,52] (#612026).
Definitions are in `tmp/bve-summer-finish-actions-controls.json`; failure logs use
`tmp/balor-summer-finish-author-{omission,spill}-red.log`.
The eight retained corpus tests keep their earlier material expectations;
only corpus paths and totals change.

All 692 standalone variants pass strict recipe validation. The full combined
comparison checks these against the collection and preserves all 1660
earlier original/variant PNG and metadata files. Evidence:
`tmp/balor-summer-finish-author-final-output.log`,
`tmp/bve-summer-finish-actions-balor-retained.log` and the shared integration report.

Static images do not establish live scheduling or rendering. Game-derived
artwork and packages remain ignored.
