# Zorel: complete four-season world candidate

2026-11-05. **Authored and validated; displayed artwork accepted, inspection depth unspecified.**
This approved finish-plan batch adds all 56 seasonal world strips to a new
`zorel-world-trial` profile/set. The 32 prior portrait regions, ten existing color
groups, source roles and preset mappings are retained exactly. The integrator
owns shared registry and collection changes; no runtime code changes here.

## Source and material decisions

The preserved source archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 176 source PNG/metadata files in the complete portrait/world export match it
byte-for-byte. Every profile region pins its source PNG. Export and source
metadata evidence remain under `local/authoring/zorel-world/`.

The native `assets/fiddle/npcs/zorel.toml` defines four outfits and action, blink,
idle, sit and walk cycles. Each outfit has 14 native strips: action East/North/South
with seven frames each; blink East/South with three; idle and sit East/North/South
with one; and walk East/North/South with four. There are 180 frame occurrences,
117 exact decoded unique frames and 63 duplicates. None reuse previously accepted
world masks. The authored selections have 82 distinct mask shapes.

World skin uses existing `#B0755D` (light), plus `#994A4C` (medium) and
`#3B1F29` (deep finger/hand shading). The new shades receive singleton groups and
the existing medium/deep target values. No prior palette role changes. The
2,379 new seeds select 4,376 skin pixels per target: Autumn 1,044, Spring 1,306,
Summer 1,285 and Winter 741.

The selected materials are face, ears, neck, exposed hands/fingers, Spring's
small back-neck patch, and Spring/Summer exposed ankles and feet between sandal
straps. Source views distinguish those light/shadow feet from the purple straps,
Autumn red boots and Winter purple boots. Winter gloves conceal most of each
hand but leave small fingertips exposed, including one-pixel North action
fingertips and a shaded North seated fingertip. All 56 strips have visible skin;
there are no empty masks or omitted no-change strips.

Purple hair and eyeliner, gray/pale eyes, dark mouth details, seasonal clothing,
glove fabric, sandal straps, boots and the carried lute preserve their original
pixels. All world occurrences of the three selected skin shades are anatomical
skin in this corpus; no same-color clothing exclusion was required. Selections
remain component masks rather than unrestricted color replacement. Other
portrait source shades cannot spill into world art without an explicit seed.

## Review artifacts

- [Five-choice SAMPLE overview](../../generated/zorel-world-finish-review/summary.png):
  one front idle case per season; columns are Vanilla, Debug Blue, Hayden, Ryis,
  Seridia.
- [Complete every-frame review](../../generated/zorel-world-finish-review/index.html):
  four outfit indexes and 92 small comparison pages. All 180 native frames and
  64 native West mirrors appear, with Vanilla left and Debug Blue right. Each
  side shows the full visible sprite and enlarged face/hand details.
- [Component authoring gallery](../../generated/zorel-world-mask-review/index.html):
  all 180 occurrences reconstruct from the candidate with zero conflicts or
  unresolved groups. Its reference-mask status does not establish acceptance.
- Candidate variants/package: `generated/zorel-world-finish-build/`.
  Complete source export: `extracted/zorel-world-finish-study/`, linked from
  `extracted/test-corpus/zorel`. Accepted portrait baselines remain untouched.

The author inspected all native source/candidate frames and all West source/Blue
mirrors in seasonal sheets, with coordinate/color detail views for face, hands,
Winter fingertips and Spring/Summer feet. These are author inspections, not user
acceptance or live-game observations.

The gallery helper independently reconciles every case with native metadata,
requires full visible bounds in both directions, and verifies 29,079,552 enlarged
preview pixels against the actual current bundle. It checks all 488 full
Vanilla/Blue bindings, 488 enlarged details and 20 sampled natural-palette cells.
CPU Chromium loaded all 97 desktop pages and five mobile indexes: all images and
links loaded, with no horizontal overflow. The final 102 page loads took 52.11s.
Final integration review caught a copied `MERRI` heading in the sampled PNG.
The Zorel helper and summary were corrected to `ZOREL WORLD SPRITES`, and the
full gallery pixel/bounds checks and CPU browser checks were rerun. Scoped label
inspection found no other copied Merri labels; profile and set pins are unchanged.
The correction is recorded in `local/authoring/zorel-world/heading-correction.json`.
No local GPU or local model inference was used. Other browsers and natural
gameplay remain unverified.

## Focused verification

Fresh checks passed:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --release --locked --test zorel_world -- -D warnings && cargo test --release --locked --test zorel_world --test zorel_portraits -- --include-ignored'
target/release/mistria-palette build-presets \
  --original extracted/zorel-world-finish-study \
  --presets palettes/sets/zorel-world-trial.json \
  --output generated/zorel-world-finish-build
target/release/mistria-palette review-batch \
  --archive local/game/assets.zip \
  --config palettes/review/zorel-world.json \
  --output generated/zorel-world-mask-review
```

Use fresh output names when reproducing. All three new tests passed, together
with the existing source-backed portrait material test. Two regular tests check
complete seasonal membership and unchanged portrait roles, then synthetic native
package frames, timing, offsets, pixels and atlas. The opt-in material test uses
fixed per-frame counts and independent source-color/location expectations to
check every world RGBA pixel in all four targets, preserves all metadata bytes,
and rebuilds the old portrait set to compare all 256 retained PNG/metadata files
byte-for-byte. The existing portrait material and fine-finger checks still pass.

Each target also passed CLI exact-recipe validation over all 88 portrait/world
strips, including dimensions, alpha, metadata and source pins. Two deliberate
negative recipes failed at the intended material assertions: omitting a Summer
foot component failed at `(37,53)`; selecting adjacent purple hair failed at
`(36,37)`. Their logs and recipes remain in the authoring evidence. The
unmodified candidate passed afterward. `FOM_ZOREL_WORLD_PRESETS` selects a
negative recipe only for the opt-in world-material test.

Final logs, measurements, source grids, native NPC definition, author/gallery
helpers and recipe validations are under `local/authoring/zorel-world/`.
`measurements.json` records the source-export start and checked-candidate
checkpoint: 5m47s elapsed, including source study, authoring, inspection, focused
tests and browser checks. It excludes documentation and combined integration,
and does not measure human review time. The final material test took 0.58s;
gallery generation/pixel checks took 1.62s; four CLI validations plus source
archive/hash checks took 2.27s.

Profile SHA-256:
`4695e50802f8d9bc9ba4e93665d1bbc15e1b000b6fd338b074f271993e2512b7`.
Set SHA-256:
`e2901db92e6890ae783c9739498b3130427e322871f452178574f581f32c52b9`.
The gallery's `check-results.json` pins these inputs and records coverage limits.

The user later accepted the displayed candidate; no accepted world baseline has
been written. The parent batch passed full collection checks and isolated
install/remove; live rendering/input and gameplay remain release gates. All game-derived images
and packages stay ignored. The retained source, bundle, review, authoring gallery
and evidence occupy about 25 MiB; small inspection sheets and executable helpers
remain under the project `tmp/` for reproduction.

After the combined second-batch review, the user reported Reina mouth issues
and no other issues. This records acceptance of the displayed artwork for this
character, without inferring full-gallery inspection depth or live-game acceptance.
