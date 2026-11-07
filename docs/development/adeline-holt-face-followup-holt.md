# Holt moustache material follow-up

2026-11-06. The user reported that Holt's moustache was damaged, identifying it
as a pre-existing issue, while judging the rest of the displayed roster treatment
"pretty good" apart from Adeline's two remaining blush dots. This record owns the
bounded Holt correction. Adeline and combined installation/review are owned by
the parent batch. This is implemented and source-verified; the new moustache
appearance still requires the user's visual assessment.

## Cause and current-only correction

Holt's portrait moustache reuses skin colors. The original
`palettes/profiles/holt-portraits.json` selected 368 isolated moustache components
as skin across all 32 portraits. The inherited world profile retained those
seeds. In the second frame of embarrassed/happy/neutral/wink, this recolored a
large central hair component and its contour, leaving a conspicuous gap. Other
expressions lost smaller contour pixels. All ten current non-Vanilla choices
showed the defect before this follow-up. Source-bound comparison also confirms
that every affected pixel already had the same wrong color in the candidate
before the roster blush correction.

The current native-tone recipe now uses
`palettes/profiles/holt-native-tone-base.json`, an exact clone of the historical
world profile with only those reviewed moustache seeds removed. The six-tone
facial override receives the same removals, retaining the existing blush seeds.
No source color, group, target value, source binding, region inventory or other
seed changes. All ten preset objects are exact; only the current recipe's shared
profile path changes. The original portrait profile, historical world profile,
legacy portrait/world recipes, accepted snapshots and prior review evidence
remain unchanged.

The current machine-readable [Holt record](native-tone-face-corrections/holt.json)
retains the existing blush `regions` and adds a separate `restored_components`
array: every restored portrait is bound by asset path, source PNG hash and strip
size, and each component records its seed, source color, pixel count and
`moustache` role. It records the historical base path/hash separately from the
corrected current base. Exact source-colored moustache landmarks are protected
for all ten choices. This is the sole intentional exception to old-four output
preservation for this current-only fix: only these moustache pixels change.

## Fresh evidence

The candidate is `generated/adeline-holt-face-followup/holt`. Evidence and exact
pre-follow-up copies are under `tmp/adeline-holt-face-followup/holt`; retained
CPU authoring/checking helpers are under
`local/authoring/adeline-holt-face-followup/holt`.

- Observed the literal component-landmark regression fail before editing:
  **3,680 of 3,680 assertions failed** with the expected source hair colors.
  The same check passes after rebuilding, with zero failures.
- Audited all **32 portrait strips / 64 native frames / 640 palette frame cases**.
  All 64 seasonal source mouth crops exactly match the 16 reviewed Spring
  expression/frame patterns. The complete all-tone sheet displays Vanilla plus
  all ten choices for those 16 patterns (176 crops).
- Removed **368 component seeds / 896 source pixels** from each current profile.
  Exactly **320 portrait PNGs / 8,960 RGBA pixels** change: 128 PNGs / 3,584 pixels
  in the old four choices and 192 PNGs / 5,376 pixels in the six additions.
  Every changed pixel is restored to the original source moustache color.
- Independently constructed every expected image by restoring only the recorded
  flood-filled source components into the frozen prior candidate. All
  **34,099,200 decoded affected pixels** match, proving exact alpha, unchanged
  skin/blush and all other portrait material preservation.
- All **1,110 world PNGs** and **1,430 metadata files** remain byte-exact.
  All **1,430 actual candidate PNGs** are hashed. Six blush assertions and
  **3,720 protected-material assertions** pass, including all moustache seeds,
  sclera and tongue/outline landmarks inherited from the previous record.
- All ten preset objects/targets are exact, as are both historical Holt sets and
  the original world profile. Both corrected current profiles differ only by
  the listed seed removals. No Rust/GML/runtime/configuration changes were made
  by this author.

`checks.json` owns the complete corpus counts, candidate report hash and current
input hashes; `candidate-files.json` pins every output PNG. The before/after
regression logs, `literal-fail-before.json`, `restored-components.json`,
`preexisting-origin.json`, `pattern-audit.json` and retained source mouth grids
provide reproduction details. `before-inputs.json` pins six pre-follow-up
recipes/profiles/records, with byte-exact copies under `before/`.

CPU image inspection covered every source expression/frame pair and the final
all-tone sheet. No game rendering, local GPU work, model inference or image
generation was used. Numerical checks and offline visual inspection do not
establish user acceptance or a new live-game result. The parent owns combined
regressions, Nix packaging, fresh installation/removal and the consolidated
Adeline/Holt review. No commit, staging or publication was performed.

## Retention

Retain the fresh complete candidate (approximately 50 MiB) and the small frozen
input/proof packet for review and reproduction. Prior candidates and galleries
were left untouched. No redundant game archive was created.
