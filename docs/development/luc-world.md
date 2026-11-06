# Luc: complete four-season world candidate

2026-11-06. **Authored and validated; displayed world artwork accepted.** The approved
Luc/Maple batch adds all 99 omitted Luc world strips through new
`luc-world-trial` profile/set files. All 32 portrait regions, nine prior source
roles/groups and four preset IDs, labels and existing mappings remain exact.
Shared registry, collection selection, planning and full collection integration
belong to the integrator. This candidate changes no runtime code or accepted
fixture. See the [finish plan](../plans/finish.md).

## Source and material decisions

Source archive: `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The complete 131-strip export contains 262 PNG/metadata members, verified
byte-identical to the preserved archive. Every source PNG is pinned in the
profile. Original metadata and the native NPC definition remain in scoped local
evidence; no archive was copied.

| Outfit | Strips | Native frames | West mirrors | Skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 24 | 78 | 31 | 2,877 |
| Spring | 27 | 81 | 31 | 3,768 |
| Summer | 24 | 78 | 31 | 3,731 |
| Winter | 24 | 78 | 31 | 2,386 |
| Total | 99 | 315 | 124 | 12,762 |

Every outfit includes action, blink, drink, eat, idle, sit and walk cycles, plus
a seven-frame East net swing and South head-scratch start/loop/end. Spring has
three shocked strips. Native metadata fixes 80×80 frames, the Default atlas,
offsets (40,54), frame counts and durations. West uses native East mirroring;
no invented West source assets are registered.

The corpus has 205 exact unique native frames, 110 duplicate native occurrences,
94 unique West mirrors and 150 native mask shapes. The 7,457 new component seeds
select exposed forehead/face, ears, mouth-edge skin, neck, hands and exposed
Spring/Summer legs. Net grips, scratching hands and shocked hands are included.
Every frame has exposed skin: 8–66 selected pixels. No empty mask or no-change
exception is needed.

World skin uses `#C48E68` light, `#B36844` medium, `#8E4538` shadow and `#712922`
deep. Only `#8E4538` is appended, in a singleton group mapped to each preset's
existing shadow target at index 3. The other three shades already exist. All
nine prior roles, groups and mappings keep their positions and meaning. Original
source inspection found no clothing/object use of these four world shades.

Brown hair/eyebrows, gold glasses rims (`#EABF67`), white lenses (`#ECF0E9`), dark
pupils, true mouth interiors, clothes, backpack, shoes, net and its effects,
food and drink retain their source pixels. The glasses therefore retain visible
gold/white pixels beside the recolored skin. Open eating/shocked mouths keep
`#410808` and `#9E2626`; adjacent brown skin changes. Winter's red gloves remain
red in every pose, including scratching and net grips. Yellow/tan cuffs, shoes
and Winter trousers are fabric; exposed Spring/Summer legs change. The original
portraits and their fine glasses-blend treatment are unchanged.

## Review artifacts and inspection

- [Five-choice SAMPLE](../../generated/luc-world-finish-review/summary.png):
  Autumn, Spring, Summer and Winter rows; Vanilla, Debug Blue, Hayden, Ryis and
  Seridia columns. The heading explicitly identifies Luc and sample scope.
- [Complete every-frame gallery](../../generated/luc-world-finish-review/index.html):
  all 315 native frames plus 124 West mirrors, 439 cases across 167 small case
  pages, four outfit indexes and one root index. Vanilla is left and Debug Blue
  right, with full 80×80 sprites plus enlarged face/hand/leg details.
- [Component authoring gallery](../../generated/luc-world-mask-review/index.html):
  99 strips, 315 occurrences and 205 exact unique frames; zero conflicts or
  unresolved groups. Its reconstructed masks are candidate references, without
  historical user art acceptance.
- Bundle: `generated/luc-world-finish-build/`. Source export:
  `extracted/luc-world-finish-study/`, stable link `extracted/test-corpus/luc`.
  Reproduction scripts and reports: `local/authoring/luc-world/`.

The author inspected all 205 unique native source frames and coordinate/color
grids for skin, hair, glasses, mouth interiors/fringes, hands, seasonal fabric,
legs, net and scratching poses. The final actual bundle was then inspected on
13 native and six West contact pages, each containing complete Vanilla/Blue
sprites and enlarged details. All 205 native and 94 unique West cases were
viewed; decoded identity accounts for their duplicate occurrences. The complete
five-choice sample was also inspected. This is author assessment only.

Gallery reconciliation proves all 439 cases against native metadata. All
89,117,184 rendered comparison pixels match final source/bundle bytes: 878 full
views, 878 enlarged details and 20 sample cells. Full views include all source
pixels, including the net/effects outside the body; details supplement them.
CPU Chromium loaded all 172 desktop pages and all five mobile indexes: 177
loads, all images/links present, with no viewport overflow and GPU rendering
disabled. Every HTML title identifies Luc. Final browser results are recorded in
`local/authoring/luc-world/browser-check.json`.

## Focused checks and regression controls

Fresh focused release checks pass:

```sh
nix-shell --pure --run 'cargo test --release --locked --test luc_world'
nix-shell --pure --run 'cargo test --release --locked --test luc_world -- --ignored'
nix-shell --pure --run 'cargo test --release --locked --test luc_portraits -- --ignored'
nix-shell --pure --run 'cargo clippy --locked --test luc_world -- -D warnings'
```

Two regular tests pin the literal 99-strip inventory, retained recipe semantics
and native package geometry, timing, offsets, atlas and 315-frame total. The new
source test independently checks every world pixel in all four generated
palettes, literal anatomical/material landmarks, per-frame skin counts, unchanged
alpha/metadata, and every prior portrait PNG/metadata member. The retained
portrait material test passes. A separate prior-profile release rebuild confirms
all 256 retained portrait PNG/metadata files are identical. Scoped rustfmt and
Clippy pass. Exact recipe validation passes for all four presets, and the source
archive SHA remains unchanged.

Two intentionally incorrect candidates fail the independent source test at
literal material landmarks: removing the Autumn idle hand component seed at
(34,47), and adding a separate hair color/group plus a seed at (37,34) so the
forehead hair recolors. Negative logs and mutated recipes remain local; neither
mutation enters the candidate. Initial test-only transcription corrections
matched original garment/glove colors and the net pose's shifted skin bound;
no mask, target mapping, original source or valid assertion was weakened.

Evidence includes `independent-verification.json`, `retained-output-verification.json`,
`material-evidence.json`, `measurements.json`, `final-inspection.json`,
`gallery.log`, focused check logs and final input/output fingerprints under
`local/authoring/luc-world/`.

## Integration boundary and remaining acceptance

The parent owns combined standard/GML checks, all-character installation/removal,
prior accepted-output comparisons, planning and one consolidated Luc/Maple review.
Focused checks do not establish those integrated results. No natural gameplay,
rendered in-game UI, GPU workload or local model inference was used. The user
accepted displayed Luc/Maple world artwork on 2026-11-06; inspection depth is
unspecified and live-game acceptance remains separate. The
[batch record](sixth-finish-batch-2026-11-06.md) owns combined verification and the
actual assessment. The accepted regression snapshot is
`local/fixtures/finish-luc/`; earlier approval limits remain unchanged.

Final profile SHA-256: `d2c59617688fd27cb162042bd159c0db50b81623e4394391e44e3c409bc62501`.
Final set SHA-256: `85b9118daecb22599ddcd6c8391dff51182b43082ee637f68f0c77e2256581ee`.
The final-input/output manifest pins 2,450 files.

About 21 MiB of source export, bundle/reviews and scoped evidence, plus 35 MiB of
source grids, original enlargements, color tables and inspection contacts remain
for reproduction and review. About 28 MiB of completed temporary helper binaries
were removed; their Rust sources and compile commands remain. These are
ignored local artifacts; no game images or archives are tracked. No complete
archive or collection snapshot was duplicated.
