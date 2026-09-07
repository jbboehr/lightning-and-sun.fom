# NPC portrait palette study

`palettes/catalog/npc-spring.json` records sampled skin and facial-detail ramps
for 36 characters in the supplied archive. There are 34 distinct sampled ramps:
Caldarus and Seridia match, as do Darren and Wynne. Each character retains its own
entry and source evidence; the later duplicate has `same_ramp_as`.

This continues the [creator palette and face-detail study](palette-presets.md).
It is a first NPC target-color catalog, not a claim that every skin-colored pixel
in the game has been classified. It adds data and an Adeline trial definition;
the Rust pipeline and GML runtime need no changes.

## Scope and source evidence

The archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The inventory contains 2,303 NPC/cameo portrait strips, including 39 named spring
neutral portraits. This review sampled frame zero of those neutral portraits.
Dozy, Great Bird, and Henrietta are recorded as excluded animal portraits. Gray
skin on Caldarus and Seridia is included.

Every classified entry records the exact source asset, PNG SHA-256, dimensions,
frame number, ordered colors, and one zero-based PNG coordinate per color.
All sampled strips are 592×180, with two 296×180 frames. The 142 recorded points
are opaque and lie in the first frame. The points are evidence for the selected
colors, not flood-fill seeds or boundaries for recoloring those NPCs.

Colors run from light to dark within each selected ramp. They include the main
skin shade and selected shadows or facial contours. Some are also used for lips,
hair, or clothes elsewhere in the same portrait. Blush, cosmetics, specular
highlights, intermediate shades, and expression-specific colors are not
exhaustively classified. In particular, a four-color entry is not proof that
the source character has exactly four skin colors.

Dell, Luc, and Zorel have three selected shades; Olric has five. These entries
are kept at their observed lengths. The existing Adeline profile needs exactly
four targets, so these ramps need an explicit art decision before use in a set.
The remaining entries have four selected shades. Equal RGBA values across
characters do not establish equal artistic roles.

The review used enlarged source portraits, local color counts, and annotated
sample points. This caught several misleading histogram candidates: Hayden's
`#9A682E` shirt brown, Luc's `#83462E` eye color, and Nora's `#B35A35` hair brown
are omitted from their skin selections. Hayden's selected `#B27146` has a visible
sample below the eye; Nora's selected `#B65932` is sampled at the neck. The
automatic `catalog` command still correctly labels full portrait histograms
`unreviewed_colors`; it does not load or infer this manual classification.

Only colors, coordinates, names, and provenance belong in Git. Source PNGs,
annotated cards, comparisons, and generated MOMI packages remain local.

## Adeline trial

`palettes/sets/adeline-npc-trial.json` uses the existing spring profile and cycles
Vanilla → Debug Blue → Hayden palette → Ryis palette → Seridia palette → Vanilla.
Vanilla remains the session default. Each native target copies its catalog ramp
in order without interpolating or inventing colors.

| Target | Main shade | Second shade | Third shade | Deep shade |
| --- | --- | --- | --- | --- |
| Hayden | `#E8B271` | `#CA9052` | `#B27146` | `#6E4922` |
| Ryis | `#B06C57` | `#814A3A` | `#63342A` | `#491F1B` |
| Seridia | `#C1AFA5` | `#A69084` | `#8E746D` | `#624A48` |

These are adaptations of NPC colors to Adeline's shading, not reproductions of
the source NPC's rendering. All 25 Adeline spring expressions use the approved
four-color skin mask. Her existing pink lips, blush, and other detail treatment
remain as reviewed in the previous slice. The original creator trial stays in
`palettes/sets/adeline-trial.json`.

Generate the new trial from an existing full spring export:

```sh
target/release/mistria-palette build-presets \
  --original extracted/adeline-spring-study \
  --presets palettes/sets/adeline-npc-trial.json \
  --output generated/adeline-npc-presets
```

Use a fresh output directory when repeating the build. Installation uses the
same `--presets` file with `install`; follow the existing removal and mod-list
requirements in [TOOLS.md](../TOOLS.md#cli-installation).

## Local verification

The source review artifacts are `generated/npc-skin-review/<character>.png` and
`samples/<character>.png`. Originals were exported into
`extracted/npc-skin-study-1` and `extracted/npc-skin-study-2`; the exporter allows
at most 25 assets per run. Local authoring helpers are `tmp/npc-face-cards.rs`,
`tmp/npc-ramp-candidates.txt`, and `tmp/npc-ramp-samples.rs`.

`tmp/verify-npc-palettes.rs` independently read the original ZIP and the recorded
catalog. It verified the archive and all 36 PNG hashes, dimensions and frame
bounds from metadata, all 142 exact sample colors, duplicate-ramp references,
coverage of all 39 neutral portraits including exclusions, and agreement between
the trial targets and the catalog. Its output is `tmp/npc-catalog-verification.log`.

The generated bundle is `generated/adeline-npc-presets/`. Exact recipe validation
passed for all four variants: 25 strips and 80,951 changed pixels each, totaling
323,804 changes. Validation includes untouched pixels, alpha, dimensions, and
metadata. Reports are `tmp/npc-presets-<id>-validation.json`, using the local
`generated/npc-skin-review/<id>-recipe.json` files. The build report is
`tmp/npc-presets-build-report.json`.

Nine comparison pages under `generated/adeline-npc-previews/` cover both frames
of every expression across Vanilla and the four variants. All nine were visually
inspected. Page 5 includes neutral, neutral tired, and sad; page 6 includes the
sick expressions where the unchanged warm nose flush is most conspicuous on blue
and gray. The trial looks consistent in these static comparisons. This is an
art judgment, not an interactive in-game approval.

The user reviewed the comparisons and reported that they all looked pretty good.
The supplied independent review found no actionable defects after checking the
catalog against the source archive, building and validating all four presets,
and running formatting, Clippy, the 59 active tests, and the release build.
That review did not rerun MOMI or interactive rendering.

A real MOMI install in `tmp/cli-installer-lab` passed the installer's checks of
the original and all variant atlas frames, animation metadata, runtime scripts,
and existing mod selection/order. The existing Other Mod Probe remained selected.
Uninstall restored the exact prior archive, confirmed with `cmp` against
`tmp/cli-before-othermod.zip`. Reports are `tmp/npc-presets-install-report.json`
and `tmp/npc-presets-remove-report.json`. The read-only source archive was not
modified.

Fresh formatting, Clippy, all 59 active tests, and the release build passed;
the log is `tmp/npc-palettes-checks.log`. The five opt-in tests and interactive
game rendering were not rerun for this data-only slice. The real MOMI roundtrip
above exercised this new set separately from the synthetic test suite.

## Remaining work

The next useful check is an in-game visual pass of this small NPC trial. The
existing `./tmp/play-masked` desktop launcher still has the approved creator
trial; it has not been switched to these NPC choices. Other seasons, outfits,
lighting, and overworld sprites still need their own classification and masks.
Expanding the catalog into a source recoloring map for another character requires
reviewing that character's full expression set and protecting shared colors.
