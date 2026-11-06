# Finish inventory and archive-review repair

2026-11-05. Source: preserved `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The [reviewed inventory](world-inventory-2026-11-05.json) freezes all 1,451 world
omissions at `bd5980d`, including source PNG/metadata hashes, frame counts and the
selected configuration fingerprints. Later implementation does not rewrite this
baseline; current progress belongs in [planning](../planning.md).

## Dispositions

| Disposition | Strips | Evidence |
| --- | ---: | --- |
| Needs recolor authoring review | 1,403 | Registered character world artwork; review can deliberately preserve covered/no-skin frames |
| Preserve ground shadows | 30 | All visible pixels are opaque `#252B43`, with transparent pixels otherwise |
| Preserve spectral Priestess | 18 | All 48 frames visually inspected; spectral/flash colors, no reviewed human-ramp colors |

The shadow decision includes Hayden's 29 audited strips and Balor's 22-frame
shadow-jump strip. The complete pixel audit confirms the earlier
[Hayden finding](hayden-shadow-audit.md) on this preserved archive. No new masks or
generated variants are required for these independent ground-shadow renderables.

Seridia's Priestess frames use blue/white spectral coloring and purple flash
effects. Their complete 48-frame source montage was inspected, including blinking,
walking, action and spell phases. None of their visible colors appears in her
reviewed human portrait ramp. The existing portrait policy preserves spectral
forms; the same no-change decision applies to these world effects. Flashback
Priestess remains in the authoring inventory, separate from spectral Priestess.

Caldarus's world folders, Seridia's human/Flashback folders and the four cameos'
root-level world paths remain explicit authoring work. No folder disappears merely
because it is outside a normal seasonal prefix. All baseline omissions now have a
disposition; no speculative new complexion is applied to a supernatural effect.

## Reproducing the source audit

From the repository root, choose a fresh ignored output path:

```sh
nix-shell --pure --run 'cargo run --release --locked --example world_inventory -- local/game/assets.zip generated/world-inventory-new'
```

The maintainer example resolves current selected profiles, rejects duplicate ZIP
members, verifies PNG strip dimensions against metadata and records every omitted
path. Its source sheets and `spectral-frames.png` stay local. Source image/metadata
hashes and source colors remain evidence for human material decisions.

Its fresh spectral results are deliberately labeled unresolved until reviewed;
the dated checked-in snapshot records the manual decision above. A different
source image must be reviewed again, even if its directory/name matches. Current
omission counts decrease as profiles grow, so a later run need not report 1,451.

## Archive-review test contract

The old local review test stopped at `143 expected / 282 actual`, hiding its later
pixel comparison. Its initial asset set was recovered from commit `b249af1` into
`tests/fixtures/adeline-original-batch-assets.json` without altering a source image
or reference mask.

The repaired test requires every reviewed asset in the preserved 282-strip action profile, rejects duplicate
reconstructed assets, and retains every original asset identity. Negative checks
cover an omitted new asset, duplicate output and a historical asset removed from
both reconstructed and expected inventories. The central profile coverage check
continues to own current totals.

After correcting inventory agreement, the retained pixel validation found a
second test-harness defect: converting its referenced recipe to inline regions
dropped inherited color groups. The recipe now carries the report's groups and
asserts they match the reviewed profile. The original independent recipe/pixel
validator remains unchanged. The complete legacy 282-asset recolor/validation round trip
passes; removing those groups reproduces the palette mismatch.

This repair changes test scope/accounting and its recipe construction; it does
not change the batch authoring runtime, masks, target palettes or accepted pixels.
