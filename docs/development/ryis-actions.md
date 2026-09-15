# Ryis's Spring action expansion

This slice follows the accepted idle/walk pilot and adds 11 normal Spring
blink, sit, eat and drink strips: 31 source frames plus 12 native West mirrors.
The existing F10 choices remain Vanilla, Debug Blue, Adeline, Hayden and Seridia.
The extended profile has 126 sources and 504 variants. All 115 earlier region
objects, source colors and target values remain unchanged.

## Sources and masks

The read-only archive is `tmp/momi-lab/assets.bak.zip`; local extraction is
`extracted/ryis-actions-study`. Sources are under
`assets/animations/NPCs/Ryis/Sprites/Spring/`, with names
`spr_npc_ryis_spring_{cycle}_{direction}.png`. Every frame is 80×80 with Default
atlas and Middle/54 origin. `tmp/ryis-actions-metadata.json` records exact native
sidecar text and frame counts.

| Action | North frames | South frames | East frames | Native duration |
| --- | ---: | ---: | ---: | --- |
| Blink | — | 3 | 3 | `[0.075, 0.125, 0.075]` |
| Sit | 1 | 1 | 1 | Omitted; engine default |
| Eat | 3 | 5 | 5 | North `1.0`; South/East `[0.125, 0.15, 0.175, 0.125, 0.6]` |
| Drink | 3 | 3 | 3 | `1.0` |

The accepted world ramp (`B06C57`, `854D3C`, `63342A`, `491F1B`) needs no new
colors. The 137 component seeds select 1,496 skin pixels per target. All 31
frames were inspected in Vanilla and all four targets. The broad rear-head
`5E423B` area remains short hair. Yellow `ECC45E`/`DA904D` gloves and cups,
clothing, eyes, food and red mouth artwork remain original. In particular,
`410808` and `C83E37` in the open eating mouth are excluded while the tiny
detached fingers around raised hands are included.

| Action | North changed pixels | South changed pixels | East changed pixels |
| --- | ---: | ---: | ---: |
| Blink | — | 190 | 169 |
| Sit | 30 | 56 | 48 |
| Eat | 86 | 281 | 235 |
| Drink | 86 | 171 | 144 |

Profile SHA-256:
`a8d1c0b3e0d0c1ec3a750759fd912d1ca24362dcaf2f13056ac7a8de030b69c8`.
Component inventory: `tmp/ryis-actions-components.json`. Inspected five-choice
sheets: `tmp/ryis-actions-art-{cycle}-{direction}.png`.

## Offline review and verification

- [Summary PNG](../../generated/ryis-actions-preview/summary.png): four action
  samples across all five choices, about 56 KB.
- [Complete Vanilla/Blue review](../../generated/ryis-actions-preview/blue-review/index.html):
  all 43 direction/frame cases across eight pages, at most eight cases each;
  the complete directory is about 888 KB.

Crop `[29,23,22,34]` includes every visible source pixel. The summary uses 6×
nearest-neighbor enlargement and the full review 10×. Exact checks cover
6,432,800 full-review pixels, 538,560 summary pixels, all 20 sample bindings,
native metadata, clipping and West reversal. Chromium decoded every image
and checked all eight pages, 43 cases and links. Evidence:
`tmp/ryis-actions-preview-pixel-check.log` and
`tmp/ryis-actions-preview-browser-check.json`.

The opt-in corpus test in `tests/ryis_actions.rs` checks every new pixel in all
four targets, per-frame skin counts, alpha, sidecars and 13 material landmarks.
It passes with the authored profile. Removing the isolated South eating
finger seed `[196,40]` in an ignored profile copy makes it fail at that exact
pixel. Evidence: `tmp/ryis-actions-focused.log` and
`tmp/ryis-actions-missing-finger-red.log`.

The standalone bundle is `generated/ryis-actions-trial`. All 1,150 previous
Ryis PNG/metadata files are byte-identical to the accepted combined trial,
and all 504 final variants equal the inspected outputs. Evidence:
`tmp/ryis-actions-comparison.log`. Shared packaging, combined-bundle checks,
native runtime probing and installation are handled by the parent integration
task. These static previews do not demonstrate timing, natural NPC schedules
or live gameplay. The user accepted this expanded artwork in the offline review.
