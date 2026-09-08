# Batch mask authoring

`review-batch` generates an offline browser gallery from a local game archive.
It groups identical decoded frames, reuses approved masks, and suggests matching
color-connected regions for other frames. This replaces the per-slice temporary
authoring scripts with a reusable Rust command and a small browser editor.

This is an authoring tool for the maintainer or coding agent. The user-facing
review should be a small sheet of completed recolors and any uncertain cases.
Do the region selection, propagation, and full-corpus inspection before handing
over that sheet. Individual component editing is available for corrections;
it is not a required user workflow. The [Hayden portrait pass](hayden-portraits.md)
uses this division of work.

The later [Ryis portrait pass](ryis-portraits.md) uses the same workflow for a
darker source palette, including shared-color gloves and wedding clothing.
The [Reina and Juniper batch](parallel-portraits.md) runs two authors concurrently
with separate character files and an independent art reviewer. Integration and
the user-facing comparison remain one combined task.
The next [Celine and March batch](celine-march-portraits.md) applies the same
workflow to larger corpora, including gardening outfits and special expressions.

The gallery uses local files and works without a web server or JavaScript package
manager. Game images and generated galleries belong under ignored `generated/`
or `tmp/` paths. Only review configurations and approved recipe data belong in Git.

## Build and open

```sh
nix-shell --pure --run 'cargo build --locked --release'
target/release/mistria-palette review-batch \
  --archive tmp/fields-of-mistria/assets.zip \
  --config palettes/review/adeline.json \
  --output generated/review-adeline
```

Open `generated/review-adeline/index.html` in a browser. Use a fresh output
directory when rebuilding. The command reads the source archive and never
installs anything. It writes `batch.json`, the gallery files, and one local PNG
per unique frame. Output paths cannot overlap the archive, configuration, or
reference profile.

`palettes/review/hayden.json` selects Hayden's portraits and now includes the
authored masks. Its eleven source colors cover the four NPC catalog samples,
lighter patches, fine skin shadows, mouth details, and body-hair shading.
His overworld colors still need their own study.

## Review workflow

1. Start with **Needs review**, and search for an outfit or action if useful.
   Frames are ordered by asset path and frame number. Auto zoom enlarges small
   overworld frames to 6× and portraits to 2×; manual zoom remains available.
2. Compare Original, Debug Blue, and the mask overlay. Pink indicates selected
   regions; orange indicates other pixels using the configured source colors.
3. Click a region in any view to include or exclude it. This changes every
   occurrence of that exact frame. Editing removes its reviewed status.
4. Choose **Approve frame**. Matching components become suggestions in other
   unreviewed frames; manual edits are preserved. Suggestions always need review.
5. **Export reviewed profile** downloads `source_colors`, `color_groups`, and
   `regions`. Only strips whose every frame is reviewed are included. An approved
   empty mask deliberately preserves an entire frame unchanged.

The gallery stores progress in browser storage when available. **Save progress**
downloads a portable copy; **Load progress** checks the batch identity and all
component selections before replacing decisions. Keep the gallery and its frame
images together. A rebuilt batch with different inputs or mask evidence rejects
old progress instead of attaching decisions to different artwork.

The exported profile records each original PNG checksum, dimensions, and seed
coordinates in the full strip. It can be used by `apply` and `validate` through a
palette recipe's `profile` field. To reuse newly reviewed work in a later batch,
set its configuration's `reference_profile` to that exported file.

## Configuration and matching

A review configuration contains a label, one or more exact archive directory
prefixes ending in `/`, opaque source colors, one preview color per source, and
an optional `reference_profile` path relative to the configuration file. The
reference profile must use the same normalized source-color set. References may
be outside the selected directories; they are read and validated as evidence.

Optional `color_groups` partition the source colors for connected selections.
For example, `[["#E8B271", "#CA9052"], ["#CE913B"]]` lets the first two skin
shades connect while gold detail pixels form separate components. This lets a
mouth detail be selected without gold shirt trim joining the skin mask. Every
source color must occur exactly once, and each group must contain a color.
Omitting the field or using `[]` retains the original single-group behavior.
When using `reference_profile`, groups come from that profile; omit groups from
the review configuration and palette recipe. Standalone configurations and
inline recipes may specify them directly. Exported profiles preserve grouping,
and both `apply` and `build-presets` honor it.

The analyzer reads horizontal frame dimensions/counts from animation metadata.
An exact-frame group requires equal dimensions and every decoded RGBA byte,
including transparent RGB. Existing source PNG checksums are still verified
before a reference mask is used. Different approved selections for identical
frames produce a conflict requiring an explicit decision.

Component suggestions require the same frame dimensions, pixel coordinates,
and source colors. Their matches are conservative: changed poses and moved hands
may need new selections. Conflicting positive and negative examples do not
produce a positive suggestion. Equal colors alone never approve a region.

The recipe format selects whole four-connected components within each color
group. Connected skin and clothing using the same color still cannot be
separated by this editor. Missing colors are outside the candidate mask and
need configuration review. The analyzer rejects same-group source-color
connections across frame boundaries,
because per-frame selections could otherwise export misleading whole-strip
flood-fill seeds. Batch limits are 2,000 strips and 50 million decoded pixels.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test review_batch batch_reuse_reproduces_every_approved_adeline_mask -- --ignored --exact'
FOM_REVIEW_BROWSER="$(command -v chromium)" nix-shell --run \
  'cargo test --locked --test review_batch browser_ -- --ignored'
```

The synthetic archive checks duplicate frames, disconnected skin versus
same-color clothing, conservative suggestions, conflicting reference decisions,
stale source checksums, joined frame boundaries, deterministic output, and an
unchanged source archive. The Chromium test clicks the actual gallery, saves and
loads progress, exports a profile, and applies that exported mask through Rust.
It also checks that editing invalidates shared approvals, newly reviewed regions
propagate only as suggestions, and clicks wait for the current preview to load.
The local corpus test rebuilds Adeline's batch and verifies the reconstructed
143 approved strip masks against the previous exact recoloring recipe.

The independent tests also cover transparent RGB and frame dimensions in exact
matching, references outside selected prefixes, conflict-driven withdrawal of
live suggestions, and preservation of manual edits during later approvals.
Chromium regressions exercise negative evidence from reference artwork outside
the selected batch, unresolved contradictory reference masks, manual conflict
resolution, and withdrawal of suggestions when their approvals are removed.

### Reliability review, 2026-09-07

Verdict: **PASS_WITH_RESIDUAL_RISK**.

Independent review found lost negative evidence from reference-only frames.
A browser regression reproduced it before the fix. Follow-up tests reproduced
two related cases: suggestions surviving withdrawal of every supporting approval,
and negative evidence disappearing from unresolved conflicting reference masks.
All three failures were fixed and their regressions passed. No accepted static
findings remain unresolved.

The final combined check ran with Chromium 152.0.7977.75:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && FOM_REVIEW_BROWSER=/nix/store/phxvwdr3rk8a31if4xay68smn74jz0r4-home-manager-path/bin/chromium cargo test --locked --test review_batch -- --ignored --nocapture && cargo build --locked --release'
```

It passed formatting, Clippy, 74 normal tests, three browser tests, the local
Adeline corpus test, and the release build. Final galleries were regenerated
with that release binary and visually inspected in Chromium. The final Hayden
interaction probe again produced suggestions in 151 other frames. Generated
galleries and screenshots are ignored by Git; no source game material was staged.

Other browsers and newly authored mask coverage have not been verified. MOMI
installation and live gameplay were not rerun for this authoring-only change.

## Scope after this slice

This slice added authoring tooling. The later
[independent character palettes](characters.md) slice made the package allowlist
and GML NPC selection data-driven and added Hayden installation. New masks still
need source inspection and recoloring checks; a successful analysis alone does
not establish coverage. The subsequent Hayden authoring pass records its own
checks.

## Initial corpus measurements

Before the Hayden authoring pass, the supplied archive produced these batches:

| Batch | Strips | Frames | Unique frames | Frame occurrences with reused masks | Unique frames needing review |
| --- | ---: | ---: | ---: | ---: | ---: |
| Adeline main portraits and all overworld outfits | 315 | 846 | 691 | 298 | 407 |
| Hayden main portraits | 133 | 266 | 256 | 0 | 256 |

Adeline has 296 pending unique frames with matching-component suggestions and
zero conflicting exact references. All 143 previously approved strips can be
exported immediately; their reconstructed masks passed exact recipe validation.
Hayden initially had no approved masks. His first reviewed frames provided examples
for live component suggestions. These counts measure available reuse, not art
approval or a measured reduction in human review time.

The initial four-color Chromium interaction probe selected the two neutral-face components containing
Hayden's four catalog sample points. Approving that test selection produced
suggestions in 151 other unique frames. This exercises the controls and matching
on the real second-character corpus; the partial face selection is not a reviewed
character mask. Probe output stays under ignored `tmp/` and `generated/` paths.
