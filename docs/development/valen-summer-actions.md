# Valen's Summer everyday actions

Eleven strips add blinking East/South and sitting, eating and drinking
North/South/East. They contain 31 source frames plus 12 native West views,
giving 43 review cases. Valen now has 149 sources and 596 variants; Summer
coverage is 17/34 strips. The user approved the offline artwork on 2026-09-29.

Fresh files are named `spr_npc_valen_summer_{blink,sit,eat,drink}_{direction}.png`
under `assets/animations/NPCs/Valen/Sprites/Summer/`. Raw sidecars match
independent archive reads: 80×80 frames, Default atlas and Middle/54 origins.
Blink has three frames at 0.075/0.125/0.075 seconds. South/East eating has five
at 0.125/0.15/0.175/0.125/0.6; North eating and all drinking have three one-second
frames. Sitting keeps single-frame defaults. Strict pins use archive SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

The 808 new seeds select 1,495 skin pixels per target. Every actual frame was
inspected in all four target palettes. Faces, closed eyelids, neck openings,
forearms, moving fingers, ankles and sandal skin recolor. Pink shirt/sleeves,
blue wristbands and sandal straps, tan trousers, goggles, hair, eyes and red
mouth interiors stay original. Existing roles suffice; there are no new material
exceptions. Eating/drinking poses do not include separately drawn food or cups.

- [Five-choice summary](../../generated/valen-summer-actions-preview/summary.png)
- [Every-frame Vanilla/Debug Blue review](../../generated/valen-summer-actions-preview/blue-review/index.html)

The complete preview is 784,297 bytes. Checks cover all 43 cases, 86 full-case
bindings plus enlarged details, 15 summary bindings and 7,766,304 exact pixels.
Chromium loads all 20 pages and local images/links without horizontal overflow.
Complete artwork and West mirrors fit within crop `[25,20,32,39]`.

The focused test checks every pixel in four targets against the independent
source-color inventory, per-frame counts, metadata and 359 literal source-grid
landmarks. Effective forehead/finger omissions and a mouth-color spill control
prove that missing skin and changed mouth material are detected. The seven
retained tests change only corpus paths and totals.

All 596 variants pass strict recipe validation; all 1,380 earlier PNG/metadata
files remain exact. All 1,192 final variant files match the inspected candidates.
The 138 prior region objects, 11 source roles, six groups, palette targets and
portrait-only definitions remain unchanged. Evidence:
`tmp/valen-summer-actions-author-final-output.log`,
`tmp/bve-summer-expansion-new-local.log` and
`tmp/bve-summer-expansion-valen-retained.log`.

See [shared integration](balor-valen-eiland-summer-expansion.md) for complete
verification and native/installation checks. Static images do not establish
live timing, scene attachments or scheduling. Generated artwork remains ignored.
