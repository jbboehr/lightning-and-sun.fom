# Louis four-season world pilot

2026-11-05: authored and automatically validated. **Displayed world art was accepted
with qualification on 2026-11-06; live play remains unverified.** This is the
whole-character P0/P2 pilot in the [finish plan](../plans/finish.md).

The new `louis-world-trial` profile covers all 44 native world strips, comprising
168 frames, across Autumn, Spring, Summer and Winter. Each outfit has action,
idle and walk facing North/South/East, plus blink facing South/East. Each native
frame has visible skin; no empty masks or omitted no-change paths are needed.
The native NPC definition and original animation metadata independently establish
these cycles and frame counts. The review also includes 60 mirrored West cases.

The profile appends 44 source-pinned regions to the existing 32 portrait regions.
All prior portrait regions, color groups and preset mappings are retained exactly.
Louis keeps Vanilla, Debug Blue, Hayden, Ryis and Seridia, with V assigned by the
shared registry. Shared integration and collection selection belong to the finish
integrator; this batch changes no runtime code.

## Material decisions

Each source shade keeps its own component group. World skin adds `#FFDBB8`
(light) and `#B46D5F` (shadow), using the existing light/shadow target roles.
The existing `#F6D2AF`, `#CB9879` and `#784B42` roles are unchanged.
The 3,303 new seeds select 5,442 source pixels per target: Autumn 1,325,
Spring 1,329, Summer 1,478 and Winter 1,310.

The source study distinguishes face, forehead, ears, hands and exposed dark
forearm shading from shared-color clothing. The Autumn outfit supplies reusable
source anatomical coordinates; seasonal changes are inspected separately. Spring
has a one-pixel wrist extension in four South action frames. Summer's cream
cuffs and cravat contain middle/shadow skin colors that remain original, while
its dark forearm components follow the target. Winter's longer coat hides part
of the far hand and keeps those coat pixels original.

The first complete sweep and source-landmark checks exposed four Summer North
action cuff corners on row 39 that an overly broad face cutoff recolored. The
literal cuff assertion failed before correction and passed after it. The final
review below reflects the correction. A second negative control removes the
Autumn East action palm component and fails the independent hand landmark.
These are source-material expectations; no candidate output was promoted to an
accepted fixture.

Brown hair and sideburn components (`#634040` and `#40252A`), glasses, lenses,
gold eyes, black outlines, the dark mouth interior, cream cuffs, cravats, outfit
colors and boots retain their original pixels. The darkest selected anatomical
cheek/chin and finger shading follows the target. Palette application is limited
to reviewed connected components and never replaces shared colors globally.

### Eye-band source check

The user's 2026-11-05 review question prompted a comparison with Louis's original
Spring portrait and Autumn front-facing idle/blink sprites. The gold band is his
glasses: source `#F4AB50` frame/bridge pixels and `#C2B9BE` / `#ECF0E9` lens/eye
detail remain unchanged. Ten literal eye-band pixels in the idle sprite were
compared directly against the current Debug Blue PNG. The portrait independently
shows the gold-rimmed glasses and reflected lenses. Surrounding normal skin
changes; the stronger contrast makes the glasses more prominent in Debug Blue.
The user subsequently described the glasses treatment as "looks a little weird
but I guess that's okay." This records qualified acceptance of that specific
treatment. At that point, approval of the full Louis batch remained unrecorded.
The candidate files were unchanged.

## Candidate review

- [Five-choice SAMPLE summary](../../generated/louis-pilot-review-v2/summary.png)
- [Complete static review, all four outfits](../../generated/louis-pilot-review-v2/index.html)
- [Autumn](../../generated/louis-pilot-review-v2/Autumn/index.html),
  [Spring](../../generated/louis-pilot-review-v2/Spring/index.html),
  [Summer](../../generated/louis-pilot-review-v2/Summer/index.html),
  [Winter](../../generated/louis-pilot-review-v2/Winter/index.html)
- [Source component authoring gallery](../../generated/louis-world-authoring-gallery-v2/index.html)

The complete review has 228 cases on 76 small comparison pages, with Vanilla on
the left and Debug Blue on the right. Every case includes the entire visible
body, an enlarged face and enlarged hands/cuffs. The four-season summary samples
only four cases and contains all five choices. The author inspected every native
and mirrored source/candidate case, then inspected the corrected Summer action
in enlarged detail. This records author inspection, not user acceptance.

The gallery checker reconciles every case against the native NPC cycles and
metadata, proves the full-body crops hide no opaque source pixel, and compares
55,523,840 rendered preview pixels against the actual current bundle. It also
pins the current profile and set hashes. Headless Chromium 155.0.8059.84 loaded
all 81 HTML pages with all 77 referenced comparison images and all links present;
no horizontal overflow was observed at a 1280-pixel window. GPU use was disabled.
Other browsers, mobile widths and live rendering remain unverified.

The component authoring gallery reconstructs all 168 frame occurrences from the
current profile with zero unresolved/conflicting groups. This includes 96 unique
full frames and 72 exact duplicate occurrences. There were zero previously
approved world masks to reuse. The final material selections have 50 distinct
mask shapes. Reconstructed authored masks do not establish art approval.

## Verification and timing

`tests/louis_world.rs` has two regular tests for complete seasonal profile
membership, preservation of prior recipes and synthetic native metadata/package
behavior. Its opt-in source test checks every pixel's alpha and permitted color
mapping in all four variants, fixed per-frame source-material counts, anatomical
face coverage, literal hand/cuff/coat landmarks and preservation of hair, mouth,
boots and unrelated colors. The package test covers all 44 native names, all 168
frames, native scalar/array durations, offsets and `Default` atlas.

Fresh focused checks passed:

```sh
nix-shell --pure --run 'cargo test --release --locked --test louis_world --test louis_portraits -- --include-ignored'
nix-shell --pure --run 'cargo clippy --release --locked --test louis_world -- -D warnings'
```

The four final targets also passed CLI exact-recipe validation for all 76
portrait/world images and metadata. A separate old-profile control matches all
256 retained portrait PNG/metadata files byte-for-byte. All 152 original
portrait/world source files match the preserved archive. Source-colored no-change
pixels, including cuffs and hair, are checked independently of profile seeds.
The full collection, repository-wide checks, isolated installation and live-game
checks belong to the integration checkpoint and are not claimed by this batch.

Measured export took 4.16 seconds. Source study, authoring, full-frame inspection,
initial checks and the cuff correction took **21 minutes 56 seconds elapsed**
from the recorded pilot start to the frozen candidate. Final verification is
recorded separately in `local/authoring/louis-pilot/measurements.json`; CPU-only
Chromium verification took 38.76 seconds, and the independent validation/prior-art
comparison script took 0.82 seconds. User review time has not been measured.
This small, repeated-pose character does not establish throughput for complex
special forms or user acceptance.

Reproducible helpers, native NPC source, source asset list, measurements, negative
controls and verification logs are retained under
`local/authoring/louis-pilot/`. They use the pinned native Rust dependencies, CPU
image operations and standard Python orchestration; there is no Pillow or local
model dependency. The final bundle is
`generated/louis-pilot-candidate-v2/`. Rebuild into fresh candidate/review names;
do not repoint an accepted fixture to candidate output.

The preserved archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The candidate profile SHA-256 is
`91eee908265d6b04e265a224b5005480d8111900ee7154f41be9ab03137bf047`;
the preset set SHA-256 is
`16cb2c72281da19b419a092432eac3ba043157cfd5da508d14f2b14e3f8a58f1`.
All game imagery and generated outputs remain ignored and local.

## Displayed-art acceptance, 2026-11-06

The user replied "looks okay to me" to the consolidated remaining-art review,
including Louis's 44 world strips and 228 comparison cases. Record qualified
acceptance of the displayed world artwork; inspection depth is unspecified.
This closes the broader Louis art gate while preserving the earlier qualified
glasses assessment. It does not establish live-game, menu or input acceptance.
The accepted snapshot is frozen under `local/fixtures/finish-louis`. See the
[release-candidate assessment](linux-release-candidate-2026-11-06.md) for shared
review and snapshot/install binding evidence and overall limits. No artwork changed.
