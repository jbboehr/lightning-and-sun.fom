# Caldarus portraits

All 98 main portrait strips are covered: 19 each in Spring, Summer, Autumn and
Winter, 12 Beach (including bathing), seven Wedding and three Dragon Statue.
Each 592×180 strip contains two 296×180 frames. The 196 frame occurrences contain
193 distinct full images; each Dragon Statue strip repeats its frame. The
seasonal atlases retain 19 Spring, 19 Autumn, 19 Winter and 31 Summer strips
(Summer plus Beach); `PortraitsMisc` holds Wedding and Dragon Statue.

The source was the read-only `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Originals are exported to `extracted/caldarus-portraits-study`. Game-derived
images, galleries, packages and temporary helpers remain ignored.

## Colors and boundaries

Vanilla retains Caldarus's original Seridia ramp. The alternatives are Debug
Blue, Adeline, Hayden and Ryis; another Seridia target would duplicate Vanilla.
Nine singleton color groups distinguish skin from disconnected matching horn
and tail components. The profile contains 49,418 seeds in 98 hash-bound regions.

| Source | Target role | Selected pixels | Matching protected pixels |
| --- | --- | ---: | ---: |
| `#C1AFA5` | Light | 169,855 | 51,214 |
| `#A69084` | Medium | 17,241 | 3,772 |
| `#8E746D` | Shadow | 34,175 | 19,728 |
| `#624A48` | Deep | 18,810 | 9,028 |
| `#755A54` | Deep; skin contours beside eyebrows and ears | 2,272 | 380 |
| `#C3AA9B` | Light; fine ear and under-eye transition | 1,710 | 0 |
| `#AA8E7E` | Medium; chest contours | 3,484 | 0 |
| `#B8A396` | Light minus RGB `[9,12,15]`; subtle chest detail | 2,872 | 0 |
| `#BEABA0` | Light; small inner-brow transition | 245 | 0 |

The custom chest shade subtracts each channel with clamping at zero. It gives
Blue `#94ADC5`, Adeline `#DA956C`, Hayden `#DFA662` and Ryis `#A76048`.
Each target changes 250,664 selected pixels and preserves 84,122 matching
non-skin pixels. There are no identity mappings in these four targets.

Selections include exposed face, ears, neck, chest, arms, fingers and the narrow
wrists beside cuffs at the bottom edge. Pale horns and bony tail spines remain
original despite sharing skin colors. In particular, the viewer-right horn
ends around `[149..151,48..50]`, above the hair separating it from ear skin.
White and `#E7D5D5` face/body markings, white nails, teal hair, gray eye details,
golden irises and jewelry, clothing, blush and red/pink mouth interiors remain
unchanged. Original blush contrast is a subjective cosmetic detail.

The three Dragon Statue forms are explicit no-ops with empty seed lists. One is
stone; the other two use distinct scale and horn colors rather than the human
skin ramp. All three PNGs and their metadata remain byte-identical in every
target. This pass does not attempt a separate dragon-scale recoloring design.

## Authoring verification

Final output is `generated/caldarus-portraits-reviewed/variants`, containing all
98 strips in each of `blue`, `npc_adeline`, `npc_hayden` and `npc_ryis`. All four
passed exact recipe validation. The author inspected all 193 distinct full
Source/Blue/Ryis trios, all 188 distinct human face trios and 45 distinct mouth
crops in all four targets. Enlarged all-target checks also covered bathing chest
detail, raised hands, Spring/Summer/Wedding wrists and the horn-to-ear boundary.

The initial fine-ear hex was transcribed incorrectly; source inspection fixed
it to `#C3AA9B`. Independent review also identified a spill onto the right horn
base. Preserving its three disconnected components restored exactly 1,330 horn
pixels per target across the corpus, with no other difference from that
candidate. The final source/output sweep and independent art review passed.

```sh
nix-shell --pure --run 'cargo test --locked --test caldarus_portraits -- --ignored'
```

The focused corpus test checks all four targets, dimensions, metadata, alpha,
exact replacement colors, unchanged unrelated colors and equal selections.
Its 25 skin and 25 protected literal landmarks per target were chosen from
source art independently of stored seeds. They include every source shade,
fine ear/brow skin, chest texture, hands, horn roots, tail, markings and mouth
interiors. Dragon no-op seeds and exact PNG preservation are also asserted.
`FOM_CALDARUS_RECIPE` overrides Blue for local negative controls: omitting the
ear shade failed at `[106,52]`; the saved horn-spilling candidate failed at
`[149,48]`. Both failures were observed, then the final all-four test passed.

The final gallery reuses all 196 occurrences in 193 groups without conflicts.
Reconstructing its component selections reproduces every final Blue PNG and
metadata file byte for byte. Inclusive visible bounds are `[22,14,239,179]`;
the full-art crop `[20,14,246,166]` contains them. Human face and mouth crops are
`[104,40,55,47]` and `[118,66,29,19]` respectively.

Ignored evidence includes `tmp/caldarus-author-sheets.json`, full/face/mouth
PNGs under the same prefix, `tmp/caldarus-author-detail-*.png`,
`tmp/caldarus-author-test-{omission,spill,green}.log`,
`tmp/caldarus-author-final-audit.json`, `tmp/caldarus-author-validated-*.json`
and `tmp/caldarus-art-review-final.md`. The independent review checked all 392
target strips, 41,771,520 pixels and 66 literal landmarks per target.

Final profile SHA-256:
`5d82ae264b7b2129e156fce57d53ab8984c52c91203a09cf9e2a25b599f294b5`.
Final preset set SHA-256:
`6c3f7f5bb19d3ad9a0eeedde4e33cf9a9baeedd5b0f868fca16ec9e4d880e606`.

## Integration and offline review

The combined collection now contains 35 characters and 2,054 source animations:
2,037 portrait strips plus 17 Adeline world animations. It generates 8,216
variants, including unchanged copies of Caldarus's three dragon/statue strips.
Delete cycles his five choices. Shared Rust and GML runtime code is unchanged.
All 98 registry paths and atlas assignments match the original archive and
Caldarus's native portrait table. `dragon_statue` and `bath_neutral` retain
their complete native outfit/expression names.

- [Compact five-palette summary](../../../generated/caldarus-preview/summary.png):
  the first Spring neutral frame.
- [Complete Vanilla/Debug Blue review](../../../generated/caldarus-preview/blue-review/index.html):
  every expression and both frames, including unchanged dragon forms.

The full review covers 196 frame occurrences and 392 Vanilla/Blue views across
16 pages, with at most eight expressions per page. Exact preview checks matched
69,142,528 placed detail pixels and 638,820 overview pixels; no visible art is
clipped. Chromium decoded every sheet and checked all links and expression
coverage. The overview and sampled Spring, Beach, and Dragon Statue layouts
were visually inspected. The whole review is about 18 MB, split into separate
images and small pages. Evidence: `tmp/caldarus-preview-pixel-check.log` and
`tmp/caldarus-preview-browser-check.json`.

Verification on 2026-09-12 passed formatting, Clippy with warnings denied,
87 active tests, the local Caldarus pixel test, three opt-in Fabricator runtime
tests, and the release build (`tmp/caldarus-final-checks.log`). The standalone
and combined character fixture first failed with `Unsupported character:
caldarus`, then passed after registry integration.

The fresh combined build validates all 8,216 generated variants. The previous
34 character trees are byte-identical to the prior batch (19,730 files), and
Caldarus's packaged originals and four variants byte-match the reviewed trees
(985 files). Evidence: `tmp/caldarus-all-variants-validation.log`,
`tmp/caldarus-bundle-comparison.log`, and `tmp/caldarus-integration-audit.json`.
All six recipe, registry, and review inputs still match
`tmp/caldarus-frozen-inputs.sha256`.

A probe using the final generated GML table passed all 98 rows and five
choices, preserved fractional animation phase, restored Vanilla, retained
selection across textbox reopening, and registered the key only once.
Delete lookup, registration, and polling passed against the pinned MMAPI
hotkey module; the installed module byte-matches that tested source. End is
absent from this MMAPI mapping. Evidence: `tmp/caldarus-runtime.log`,
`tmp/caldarus-hotkey-check.log`, and `tmp/caldarus-installed-hotkeys.log`.

The player package is installed separately in `tmp/caldarus-playtest`. The
MOMI installation verified packed pixels, metadata, and the runtime table.
Its archive SHA256 is
`308b52ff5667faab726e4c238c263dd27300246fa45d8d994569d38713353a99`.
The original backup and retained `previous.zip` both still match the original
source hash above. See `tmp/caldarus-install-report.json` and
`tmp/caldarus-source-after.sha256`.

The mounted game directory was empty. This isolated player package has no
preview helper; `tmp/play-characters` retains the earlier Zorel trial. The
previous four cameos still require cutscene context for native preview.
An uninstall roundtrip and native Caldarus gameplay were not exercised.
