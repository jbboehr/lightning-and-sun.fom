# Hayden's Spring action expansion

The accepted idle/walk pilot now adds 11 normal Spring blink, sit, eat and drink
strips: 31 source frames and 43 direction/frame cases including native West
mirrors. The extended profile contains 150 sources and 600 variants. This batch
has been inspected offline and accepted by the user.

## Sources and masks

Sources remain read-only in `tmp/momi-lab/assets.bak.zip` (archive provenance in
[the first pilot](hayden-world.md)). The local corpus is
`extracted/hayden-actions-study`; exact source metadata is inventoried in
`tmp/hayden-actions-metadata.json`. All new sources are under
`assets/animations/NPCs/Hayden/Sprites/Spring/`, named
`spr_npc_hayden_spring_{cycle}_{direction}.png`, with 80×80 frames on the Default
atlas and Middle/54 origin.

| Cycle | Source directions | Frames per strip | Duration |
| --- | --- | --- | --- |
| Blink | South, East | 3 | `[0.075, 0.125, 0.075]` |
| Sit | North, South, East | 1, implicit | Engine default |
| Eat | South, East | 5 | `[0.125, 0.15, 0.175, 0.125, 0.6]` |
| Eat | North | 3 | `1.0` |
| Drink | North, South, East | 3 | `1.0` |

The existing four world colors, their separate component groups, all target
mappings and all 139 earlier region objects are unchanged. The additions use
607 component seeds to recolor 1,404 skin pixels per target while excluding 334
matching brown pixels. Eyes, hair, beard, mouth colors and shirt material stay
original. A raised-hand boundary in the second East drink frame keeps the
`AB7E3F` shirt pixel at `[34,41]` and recolors the adjacent `523C26` forearm pixel
at `[35,41]`. South eating likewise separates raised-hand shadows from the
same-color folds below them. Frame numbers here are one-based; coordinates are
frame-local.

All 31 frames were inspected in Vanilla and all four target palettes on
`tmp/hayden-actions-art-{cycle}-{direction}.png`. Component evidence is in
`tmp/hayden-actions-refinement.json`. The frozen profile SHA-256 is
`ea83370ea141a784f128e30fdb706e85223bb82fd981793be9a07972e2196dfe`;
`tmp/hayden-actions-frozen-inputs.sha256` also pins the set and stylized recipe.

## Review and verification

- [Five-choice summary](../../generated/hayden-actions-preview/summary.png):
  four sampled action poses, about 57 KB.
- [Complete Vanilla/Blue review](../../generated/hayden-actions-preview/blue-review/index.html):
  all 31 source frames and 12 West mirrors, in six pages with at most eight cases
  each. Preview files occupy about 944 KB.

The fresh crop `[28,23,24,34]` contains every visible source pixel. Exact checks
verify 7,017,600 full-review pixels, 587,520 summary pixels, all palette/sample
bindings, source and variant metadata, no clipping and native West reversal.
Chromium decoded every image and validated all six pages, 43 cases and links.
Evidence: `tmp/hayden-actions-preview-pixel-check.log` and
`tmp/hayden-actions-preview-browser-check.json`.

The focused opt-in corpus test passes for all four targets, checking 23 literal
skin/material landmarks, alpha, metadata, each frame's nonempty mask and shared
selection across palettes. Run it with
`nix-shell --pure --run 'cargo test --test hayden_actions -- --ignored'`.
The ignored broad-color-group negative control is rejected by this test.
Evidence: `tmp/hayden-actions-focused-test.log` and
`tmp/hayden-actions-broad-mask-red.log`.

The standalone bundle is `generated/hayden-actions-trial`. Exact comparison with
`generated/characters-celine-world-trial/characters/hayden` preserves all 1,390
prior original/variant PNG and metadata files; see
`tmp/hayden-actions-comparison.log`. No game files or review artwork are tracked.

This data slice does not change Rust or GML runtime behavior. Combined packaging,
native animation probes and isolated installation are handled by the larger
Spring action integration. Static review does not exercise natural schedules,
live outfit/state changes, game timing, or separately rendered food/cup overlays.
