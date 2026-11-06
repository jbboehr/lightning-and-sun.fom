# Second finish-plan integration batch

2026-11-05. Authoring, integration and combined verification are complete. **The user reported no issues outside Reina's mouth in the displayed review;
Reina's prior mouth appearance was rejected; a [focused correction](reina-mouth-correction.md)
is validated and accepted as displayed.** The user approved Wheedle/Zorel plus the named portrait
follow-ups with "go for it" and requested a complete batch before review.

## Content and portrait decisions

| Candidate | New world strips | Native frames | West mirrors | Evidence |
| --- | ---: | ---: | ---: | --- |
| Wheedle, all four seasons | 56 | 180 | 64 | [Materials and checks](wheedle-world.md) |
| Zorel, all four seasons | 56 | 180 | 64 | [Materials and checks](zorel-world.md) |
| Total new world artwork | 112 | 360 | 128 | 488 complete review cases |

Both new profiles retain their original 32 portraits, palette roles and outputs.
Their independent material tests preserve all 512 previous PNG/metadata files.
Wheedle's shared-color hair, beard, trouser shadow and gloves are protected while
bare hand and Summer ankle shadows recolor. Zorel's Spring/Summer feet and tiny
Winter fingertips recolor while sandal straps, hair, clothing and lute stay
original. All 112 strips have visible skin; there are no new empty/no-change
strip decisions. The authors inspected every native frame and West mirror.
Their two complete Vanilla/Debug Blue galleries bind 58,159,104 preview pixels
to current bundles, with 204 desktop/mobile CPU Chromium loads passing.

The portrait follow-ups produced audits, **not speculative corrections**:

- [Hayden embarrassed expression](hayden-embarrassed-followup.md): all six strips,
  twelve native frames and all five choices. The original blush and mouth details
  remain. No concrete mask defect was isolated. The complete full-frame/face/mouth
  review has sixty frame/palette cases and 10,158,720 bound preview pixels.
- [Reina/Juniper mouths](reina-juniper-mouth-followups.md): all 235 portrait strips
  and 470 native mouth frames in five choices. Original warm lip/interior details
  and shared-color boundaries remain. No concrete defect was isolated. The full
  mouth review has 2,350 cells and 42,039,200 bound preview pixels.

The old reports did not identify exact expressions/pixels. Source-backed material
classification supports preserving the existing treatment, but does not establish
that the user finds it pleasing or close those subjective concerns. Historical
approvals remain unchanged. No candidate was written into an accepted fixture.

## Integrated coverage and source boundaries

The selected collection now has 5,154 strips: all 2,201 portraits plus 2,953 world
strips, generating 20,616 nonvanilla variants for the existing 36 characters.
Wheedle and Zorel each have 88 selected regions, and 22 characters have complete
seasonal world coverage. All current profiles and registry memberships agree.
Only 112 world mappings were added; every earlier mapping and other registry
field is unchanged.

Compared with the frozen original omission inventory, 245 strips are now covered.
The remaining 1,206 raw omissions comprise 1,158 authoring/review strips, thirty
audited shadows and eighteen spectral no-change strips. P2 has 233 remaining
world strips; P3 has 558 and P4 has 367. Raw counts are not effort estimates.

Every selected PNG/metadata member, 10,308 in total, matches both preserved
`local/game/assets.zip` and the user-supplied `tmp/fom/assets.zip`. The preserved
archive remains pinned to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
This member agreement does not establish broad game-update compatibility.

## Combined verification

Fresh combined formatting, all-target Clippy with warnings denied, 426 regular
tests (492 ignored in that invocation), release build, ten affected material/package/
retained cases and five GML interpreter cases pass. The mouth author additionally
ran both retained Reina/Juniper portrait material tests. Hand/foot omission, hair
spill and mouth-interior spill controls fail at their intended source landmarks.

The paired Wheedle/Zorel CLI installation generated all 704 variants for 176
selected source strips, passed strict MOMI compilation and verified installed
atlas pixels, animation metadata and all three production scripts. Removal
removed recovery state and restored the exact preserved archive hash. The same
601 MiB disposable lab was reused; no user installation was modified.

A final full 36-character CLI installation generated and installed all 20,616
variants, passed strict MOMI compilation, and independently verified installed
atlas pixels, metadata and all three production scripts. Removal restored the
exact preserved archive hash and removed recovery state. This covers atlas
packing with the enlarged collection, alongside the earlier paired round trip.

No runtime/installer source changed. The first batch's native engine IO evidence
remains historical; this content batch does not repeat physical rendering or
gameplay probes.

Reproduction inputs and logs:

- `local/authoring/second-batch/checks.sh`, `tmp/second-batch-checks.log`.
- Pair-only explicit character config: `local/authoring/second-batch/characters.json`.
- Pair installation: `tmp/second-batch-install.log`; removal:
  `tmp/second-batch-uninstall.json`.
- Full collection installation: `tmp/second-batch-full-install.log`; removal:
  `tmp/second-batch-full-uninstall.json`.
- Combined local batch build: `generated/second-batch-build`; all 1,408 variant
  PNG/metadata files agree exactly with the worker review bundles.
- Complete source membership/counts: `tmp/second-batch-counts.json`.
- Frozen-inventory reconciliation: `tmp/second-batch-inventory-progress.json`.
- Per-character development records own material decisions, hashes, timings and
  negative controls; historical accepted source/output fixtures remain intact.

The world authors' useful source-to-material-check checkpoints were approximately
six minutes per character. These simple repeated-pose characters do not establish
special-form throughput or human review time; the provisional remaining-effort
range retains its low confidence.

## Review, acceptance and retained limits

The [consolidated review](../../tmp/second-batch-review/index.html) contains world
samples and links all 488 complete native/mirror cases plus the portrait audits.
World samples are labeled SAMPLE; columns are Vanilla, Debug Blue, Hayden, Ryis,
Seridia. Complete world views place Vanilla left and Debug Blue right. Portrait
pages name every actual choice; Hayden uses Adeline as the natural alternative.
All cases are bound to the exact candidate fingerprints. The consolidated page
and four linked review indexes pass ten desktop/mobile CPU Chromium loads, with
all local resources available and no horizontal overflow. The fixed ignored
`generated/review` and `generated/build` shortcuts point to the combined review
and paired build. The self-contained
`tmp/second-batch-review.zip` is an alternative to the local HTML folder.

Automated validation does not establish visual acceptance. Rendered menu,
physical input and natural gameplay remain P6 release gates. The previous
software display was black with both original and modded game archives; this
batch performs no game-rendering test and needs no GPU or local model inference.

The existing restored 601 MiB install lab is reused rather than copying another
archive. New character source/build/gallery evidence is roughly 46 MiB combined;
Hayden's retained audit build/source/helper/review evidence is roughly 63 MiB.
Reina/Juniper current builds, historical comparison evidence and the review occupy
about 201 MiB and remain useful reproduction evidence. Source archives, accepted fixtures and the useful shared Rust cache
are retained. No real saves/settings or Steam library are exposed or changed.

## User review and Reina follow-up

After viewing the consolidated second-batch review, the user said, "reina has
some issues around the mouth. otherwise I don't see any issues." Record this as
acceptance of the displayed Wheedle/Zorel world artwork and retained Hayden/Juniper
details, with inspection depth unspecified. It does not establish live rendering
or input acceptance and does not approve Reina's mouth. The frozen original
review and its fingerprints remain unchanged; the correction gets a separate
Reina-only candidate and review. Earlier statements above describe the pre-review
audit and must not be read as a later finding that Reina's reported issue is fixed.

The [focused Reina correction](reina-mouth-correction.md) now removes the 425
missed lower-mouth skin pixels in every speaking portrait. Standard checks and
all-Reina installation/removal pass; the displayed correction is accepted.
The full-collection verification recorded above describes the frozen second-batch
checkpoint before this correction.
