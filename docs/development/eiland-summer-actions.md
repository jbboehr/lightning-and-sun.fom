# Eiland's Summer blinking, sitting, eating and drinking

11 strips add 31 source frames and 12 native West mirrors: 43 review cases.
Eiland now has 142 sources and 568 variants, covering 17/41 Summer strips.
The user approved the offline artwork on 2026-09-29.

The fresh source paths are under `assets/animations/NPCs/Eiland/Sprites/Summer/`.
Every PNG has a strict hash pin and dimensions in the tracked profile. Raw
metadata is unchanged. The [shared notes](balor-summer-finish-valen-eiland-actions.md) record archive provenance,
cycle types, direction coverage and native timing.

The 518 new component seeds select 1,015 skin pixels per target.
Every source frame was inspected in Vanilla and Debug Blue; five-choice
summaries cover representative poses. Existing world roles suffice, with no
new color roles or material exceptions.

Faces, necks, exposed arms and hands change. Pink clothing, Summer gold trim,
pale sleeves, trousers, boots, eyes, hair and mouth interiors remain original.

- [Five-choice summary](../../generated/eiland-summer-actions-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/eiland-summer-actions-preview/blue-review/index.html)

The complete gallery checks all 43 cases against original frame metadata,
complete artwork crops, palette bindings and native mirroring. Saved-image
checks compare 7,766,304 exact displayed pixels. Chromium checks all 20 HTML
pages, local links, loaded images and horizontal overflow. Final variant PNGs
and metadata match the inspected candidate byte for byte.

The focused test checks 335 literal source-grid landmarks, every pixel in every
target, per-frame skin counts, alpha, metadata and all earlier outputs.
Both negative controls fail their intended pixel assertion after successful
generation: omission at strip [40,40]; spill at strip [40,46] (#533061).
Definitions are in `tmp/bve-summer-finish-actions-controls.json`; failure logs use
`tmp/eiland-summer-actions-author-{omission,spill}-red.log`.
The eight retained corpus tests keep their earlier material expectations;
only corpus paths and totals change.

All 568 standalone variants pass strict recipe validation. The full combined
comparison checks these against the collection and preserves all 1310
earlier original/variant PNG and metadata files. Evidence:
`tmp/eiland-summer-actions-author-final-output.log`,
`tmp/bve-summer-finish-actions-eiland-retained.log` and the shared integration report.

Static images do not establish live scheduling or rendering. Game-derived
artwork and packages remain ignored.
