# Seridia world artwork

2026-11-06: authoring and focused verification are complete for all 188
recolorable world strips. Displayed world artwork is accepted after the
consolidated review; inspection depth is unspecified and live-game acceptance
remains pending. The parent owns the shared collection, registry, full checks,
installation/removal and consolidated review in the
[eighth-batch record](eighth-finish-batch-2026-11-06.md).

That record owns the user's 2026-11-06 assessment. The accepted regression
snapshot is `local/fixtures/finish-seridia/`; retaining portraits does not broaden
the displayed world-art approval.

The preserved source is `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The independent archive enumeration contains 206 world strips: 188 human/horned
and Flashback Priestess strips plus 18 spectral Priestess strips. The 164 previous
portrait regions, including 17 spectral portraits and Dragon, remain exact.
The new profile has **352 regions**; the 18 spectral world strips stay outside
both this profile and its registry extension.

| World form | Strips | Native frames | West mirrors | Skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 37 | 119 | 29 | 6,889 |
| Beach | 17 | 57 | 24 | 3,983 |
| Flashback Priestess | 8 | 21 | 8 | 458 |
| Spring | 37 | 119 | 29 | 6,107 |
| Summer | 37 | 119 | 29 | 6,665 |
| Wedding | 15 | 49 | 20 | 2,470 |
| Winter | 37 | 119 | 29 | 5,682 |
| Recolor candidate total | **188** | **603** | **168** | **32,254** |
| Unchanged spectral Priestess audit | 18 | 48 | Outside recolor candidate | 0 |

Every seasonal meditate, seated read/write, standing write and spell-cast
start/loop/end is included, along with Beach bathing/swimming, kiss and sleep
where supplied. There are 410 exact unique native frames and 115 unique West
frames. All 771 native/mirror cases have complete Vanilla/Debug Blue views.
The 18,124 world seeds represent 308 distinct frame masks. Fifteen entirely
occluded rear strips have explicit empty masks; 33 native frame occurrences
have no visible skin, including the hidden Flashback neck on alternating rear
walking frames. Visible skin is present in the other 570 occurrences.

## Material and palette decisions

The first 20 source colors, their singleton groups, prior region order/seeds,
preset IDs/labels and every existing target mapping are unchanged. The actual
existing choices are Vanilla, Debug Blue, Adeline, Hayden and Ryis. Thirty
needed world shades are appended, giving 50 source colors and groups:

- `#8A716A` uses the existing shadow role; `#594240` uses the existing deep role.
- Flashback `#FCD9B3`, `#F0B988` and `#D37A57` use the existing Flashback light,
  medium and shadow roles respectively.
- Twenty-five meditation glow shades use their existing skin role plus the
  literal RGB difference between the glowing source shade and its ordinary
  source shade, clamped per channel. This preserves the purple cast on
  recolored skin while keeping the original glow on every other material.

The existing 32-color policy could not represent the retained portrait roles
and all visible glowing human skin. The parent extended the validated tool
limit to 64; see [batch authoring](batch-review.md). This adds source shading,
without adding a palette choice or changing existing portrait semantics.
Five unused glow colors belonging only to horn highlights were pruned; all
2,816 candidate variant PNG/metadata files remained byte-identical afterward.

Selections include the small forehead patches between the circlet and hairline,
face/ears, jaw/neck, exposed body and limbs, fingers and toes. Brown horn/tail
material, white face/body markings, nails, makeup, eyes, true mouth interiors,
hair, circlet/gold, purple clothes, gloves/gauntlets, white bone armor, books,
parchment, pens and magic retain their source pixels. Literal checks distinguish
skin from adjacent horns, tail and shared-color material. In addition to all
unrelated colors, 9,917 pixels matching the world skin-color set are protected.

The spectral form has the same blue or purple treatment across skin, hair and
robes. All 18 source PNG/metadata pairs match the preserved source pins; all
48 native frames were inspected at full size and enlarged. There is zero overlap
with the final 50-color human source set. They retain the earlier
`unchanged_spectral` disposition; this pass does not reinterpret a spectral
effect as human skin or add spectral replacement assets. Installation preservation
is checked separately in the parent record.

## Focused verification and review

The release CLI builds all four non-Vanilla choices. Independent material tests
check every RGBA pixel against literal source-art exclusions and per-frame
counts, plus anatomical/material landmarks. The test uses its own source/role
contracts, not the candidate seeds. Omission of forehead skin at `[39,28]` and
a spill onto the horn highlight at `[35,30]` both fail the expected literal
material assertion on Autumn idle South. Both final tests pass. The existing
portrait material test also passes, including unchanged spectral/Dragon output.

Two regular profile/package cases and the new and retained source-backed cases
pass in release mode; scoped Clippy denies warnings and rustfmt passes. All
four exact recipe validations pass. All 704 candidate source PNG/metadata
members match the original archive; the 36 separately retained spectral members
have independent pins and zero-ramp checks. A prior-profile release rebuild
matches every one of the **1,312 retained portrait PNG/metadata files** exactly.
No accepted fixture was changed.

The author inspected all 410 unique native and 115 unique West Vanilla/Blue
pairs, full sprites and enlarged details, on 45 contact sheets. Identical source
frames produce identical candidate pixels, verified across duplicates. The
seven-form five-choice sample was inspected separately. Review coverage binds
**163,304,064 pixels** to the current source/bundle: 1,542 full views, 1,542
extended details and 35 sample cells. Full crops include every visible pixel;
details extend from `[24,23]` over 32×41 pixels. The sample header and all five
column labels pass an independent 88,000-pixel literal raster check.

CPU Chromium with `--disable-gpu` completed 296 desktop/mobile page loads, then
two final-header loads and four final root/spectral-audit loads: **302 successful
loads**, all referenced images/links present and no horizontal overflow. The
summary title was corrected before its literal final check. No local GPU or
model inference was used. These checks establish offline coverage and bindings;
they do not establish subjective acceptance or native gameplay behavior.

The complete review is `generated/seridia-world-finish-review/index.html`.
Detail pages show Vanilla left and Debug Blue right; the explicitly sampled
summary columns are Vanilla, Debug Blue, Adeline, Hayden and Ryis. The separate
spectral source audit contains all 48 unchanged native frames and is labeled
original art only. The parent publishes the portable combined review under `tmp/`.

## Retained evidence

Recipes, review configuration and `tests/seridia_world.rs` are tracked. Source
images, bundles, galleries and evidence remain ignored. The stable source corpus
is `extracted/test-corpus/seridia`, pointing to
`extracted/seridia-world-finish-study`. The 18 spectral world sources are stored
separately under `local/authoring/seridia-world/spectral-source` so they do not
silently enter a human build.

`local/authoring/seridia-world/verification.json` summarizes the frozen candidate;
`final-input-and-output-hashes.json` binds recipes/tests, source members, output
and gallery files. The same directory retains material/negative-control logs,
four exact validations, prior-output hashes, form audit and browser reports,
reproducible Rust helpers and inspected source/contact sheets. No source archive
was copied. Three superseded task-created output directories were removed after
recording their manifests and proving the pruned candidate PNG/metadata unchanged,
reclaiming 168,106,741 bytes. The final build is about 76 MiB, sources 3 MiB,
complete review 15 MiB, mask gallery 10 MiB and scoped evidence about 26 MiB.
