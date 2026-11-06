# Adeline Beach and Wedding finish

2026-11-05: authored and automatically validated. **Displayed world art was accepted
with qualification on 2026-11-06; live play remains unverified.** This completes
Adeline's remaining source inventory for P1 of the [finish plan](../plans/finish.md).

The candidate appends all 18 Beach and 15 Wedding world strips to the prior 282
regions. Its 315 regions cover the archive's complete 126 portrait and 189 world
strips for Adeline. Every original region, source shade, color group and preset
mapping is preserved exactly. The integrator owns the shared registry, collection
selection and central count; this batch changes no runtime code.

The new native inventory contains 110 frames: Beach 61 and Wedding 49. The source
NPC definition and animation metadata establish action, blink, idle, kiss, sit
and walk in both outfits, plus Beach bath-swim. East animations supply another
44 mirrored West cases. The complete review therefore has 154 cases.

## Material decisions and unchanged art

The existing world ramp is sufficient: `#E9A980`, `#DE8F5D`, `#BA6A4C` and
`#7D3B14` use the existing light/middle/shadow/dark target roles. No source colors,
group semantics, target colors or shade roles were added or changed. Source
inspection found these four shades confined to exposed skin in the new corpus.
The 539 new component seeds select 4,390 source pixels per target: 3,332 Beach
and 1,058 Wedding pixels.

Beach masks cover face, ears, neck, arms, hands, legs and toes. Pink hair,
glasses and eye details, the swimsuit, white sandals, and blue swimming water
retain their source pixels. The Wedding gown and gloves cover the body and hands;
its visible face and neck follow the selected palette. Crown, gold trim, gown,
gloves, hair and eye details retain their original colors.

The crown and gown use `#9F5544`, which is a mapped **portrait** source shade in
the inherited profile. Those new clothing/accessory components are not selected.
Literal source landmarks distinguish the crown and gold trim from face/neck
skin, and Beach toes from sandals. Negative omission and crown-spill controls
both fail at their intended source-material landmarks; the final candidate passes.

Five strips intentionally have empty masks, covering 17 native frames:

- Beach `bath_swim_north`: all four frames show hair and water, with exposed skin
  hidden from this direction.
- Wedding `action_north`, `idle_north`, `sit_north` and `walk_north`: all 13 frames
  show hair, crown, gown and gloves, with no visible skin.

These strips remain present in the profile, all variants, package and complete
review. Pixel checks prove that their output stays unchanged. The other 28 strips
have visible selected skin in every frame.

## Candidate review

- [Five-choice SAMPLE](../../generated/adeline-finish-review-v3/summary.png)
- [Complete static review, all 154 cases](../../generated/adeline-finish-review-v3/index.html)
- [Beach, all 85 cases](../../generated/adeline-finish-review-v3/Beach/index.html)
- [Wedding, all 69 cases](../../generated/adeline-finish-review-v3/Wedding/index.html)
- [Source component gallery](../../generated/adeline-finish-authoring-review/index.html)

Each case has Vanilla on the left and Debug Blue on the right, with the entire
visible body and enlarged details. Swimming details follow the lowered head;
the complete-body view also includes every water splash. Pages contain at most
four frames and are split by outfit and animation. The author inspected every
native and mirrored source/candidate case, plus enlarged swimming, unchanged
North views and Wedding kissing details. This records author inspection, not
user acceptance. The five-choice overview is explicitly a two-case sample.

The gallery verifier reconciles all cases with the native NPC cycles and source
metadata, checks that no opaque source pixel is cropped, pins the current profile
and set hashes, and compares 42,515,200 preview pixels against the actual current
bundle. It caught a clipped swimming splash and an overlapping detail view in
initial gallery drafts before the final gallery was produced. The final crop
contains every visible pixel; the final pixel-binding check passes.

Headless Chromium 155.0.8059.84 loaded all 57 final HTML pages, all 55 referenced
comparison images and every local link, with no horizontal overflow at a
1280-pixel window. GPU use was disabled. Other browsers, mobile widths and live
rendering remain unverified.

The 110 native occurrences contain 79 unique full frames and 31 exact duplicate
occurrences. The old profile supplies no identical approved frames, but offers
59 component suggestions. The existing world shade/material roles were reused,
with complete new-source inspection. Final masks have 50 distinct shapes,
including the empty mask. The final authoring gallery reconstructs every current
frame with zero unresolved or conflicting groups; that reconstruction does not
establish user art approval.

## Verification and timing

`tests/adeline_world_finish.rs` adds two regular tests for exact preservation of
prior recipes, complete new inventory, intentional empty masks and synthetic
packaging with native timing/offsets. Its opt-in source test checks every pixel
in all four variants against independently inspected source-material expectations,
fixed per-frame source counts and literal landmarks. It preserves alpha,
transparent RGB, unrelated colors and metadata. Source expectations do not read
profile seeds or use a candidate image as an accepted baseline.

Fresh focused checks passed:

```sh
nix-shell --pure --run 'cargo test --release --locked --test adeline_world_finish --test world_actions -- --include-ignored'
nix-shell --pure --run 'cargo clippy --release --locked --test adeline_world_finish -- -D warnings'
```

This includes all three retained `world_actions` tests. The final targets passed
CLI exact-recipe validation for all 315 images and metadata per variant, including
the 132 newly covered variant images. Independent checks also verified:

- all 2,256 prior PNG/metadata files match an old-profile control byte-for-byte;
- all 630 original source PNG/metadata files match the preserved archive, and
  exact profile membership equals all Adeline portrait/world PNG paths there;
- all 1,260 packaged PNGs match their actual variant files, and all 1,260 packaged
  animation metadata entries retain the original asset properties;
- native `Default` atlases, 80×80 frames, `Middle` horizontal offsets, 54-pixel
  vertical offsets and scalar/array durations are preserved, including kiss,
  blink and intentional unchanged animations.

The initial registration dependency was resolved by the integrator from the
exact 33-path manifest before final package verification. Full collection checks,
installation/removal and live gameplay belong to the integration checkpoint and
are not claimed here.

Export took 0.023 seconds. Source study, authoring, initial checks and complete
visual inspection took **7 minutes 20 seconds elapsed** to the frozen candidate.
Final verification is recorded separately in
`local/authoring/adeline-finish/measurements.json`. Independent validation and
prior-art comparison took 6.56 seconds; actual package bindings took 0.41 seconds;
final CPU browser verification took 52.75 seconds. No user review time has been
measured. Existing shade roles and exclusive world skin colors made this batch
simpler than Louis; this does not establish a rate for new shared-color materials.

Source plans, native metadata/NPC definitions, authoring helpers, negative
controls, timing records and verification logs are retained under
`local/authoring/adeline-finish/`. The final bundle is
`generated/adeline-finish-candidate-v2/`. No accepted fixture was changed.
Game images, exports and generated artifacts remain ignored and local.

The preserved archive SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The candidate profile SHA-256 is
`d77904757e9dbbeb87a2ee8cc41b1c135cab99ac4b87eb2c39dd8bdf62734c35`;
the preset set SHA-256 is
`3c12bc746f4a60d42c684e44f7cd6389a2592e99144e7239ea4c44b55129543b`.

## Displayed-art acceptance, 2026-11-06

The user replied "looks okay to me" to the consolidated remaining-art review,
including Adeline's 33 Beach/Wedding world strips and 154 comparison cases.
Record qualified acceptance of the displayed world artwork; inspection depth
is unspecified. This closes the finish batch's Adeline art gate without
establishing live-game, menu or input acceptance. The accepted snapshot is frozen
under `local/fixtures/finish-adeline`. See the
[release-candidate assessment](linux-release-candidate-2026-11-06.md) for shared
review and snapshot/install binding evidence and overall limits. No artwork changed.
