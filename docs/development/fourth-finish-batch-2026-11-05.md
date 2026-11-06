# Stillwell and cameo world-art batch

2026-11-05. **Authoring, integration and verification are complete. New art
acceptance is pending.** The user requested this batch after accepting displayed Taliferro/Vera
artwork and requesting the completed checkpoint commit, `cbebf4a`.

## Scope and source boundary

| Candidate | World strips | Native frames | West mirrors | Complete cases |
| --- | ---: | ---: | ---: | ---: |
| Stillwell | 68 | 232 | 84 | 316 |
| Darren | 11 | 24 | 9 | 33 |
| Linnet | 12 | 31 | 16 | 47 |
| Wiscar | 11 | 24 | 9 | 33 |
| Wynne | 11 | 24 | 9 | 33 |
| Total | 113 | 335 | 127 | 462 |

The parent independently enumerated every character's world PNG from the
preserved archive and reconciled it with the frozen omission inventory. PNG
dimensions and native metadata establish the frame counts. Every new registry
entry uses its source atlas; all prior mappings and other character fields are
unchanged. The source inventory, original registry and retained candidate hashes
are under `local/authoring/fourth-batch/`.

The source archive remains pinned to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The four cameos use root-level world paths; seasonal prefix matching would omit
them. The two authors own separate Stillwell and cameo candidates; shared
registry, selection, counts and integration belong to the parent.

## Preservation and acceptance

The prior review sources and galleries have 975 retained fingerprints, excluding
the two shared files intentionally extended by this batch. Accepted Reina,
Taliferro and Vera snapshots remain under `local/fixtures/`; Wheedle/Zorel
snapshots were also frozen without changing their previously recorded acceptance
scope. The user's inspection limits and earlier first-batch approval limits
remain in force. Keeping all portrait/world outputs for regression does not
extend the scope of the displayed-art approval.

Passing automated checks does not establish visual acceptance. New artwork is
pending one consolidated review after the complete batch and its integrated
checks. No local GPU or local model inference is required. Visible menu,
physical input, natural gameplay and release packaging remain P6 work.

## Material evidence

The [Stillwell record](stillwell-world.md) owns his new world skin ramp,
preserved hair/eyes/jewelry/mouth interiors, every-frame inspection, literal
landmarks and omission/spill controls. The [cameo record](cameos-world.md) owns
four separate material studies. In particular, Darren's same-color buttons and
Wiscar's shared-color robe shadows remain original. Linnet's scar/circlet and
Wynne's cosmetics/jewelry are protected. Linnet's three back-facing strips have
no exposed skin; all six frames remain unchanged and are still included.

Both authors inspected all native frames and West mirrors. Every world pixel in
all four generated choices passes independent material expectations, with
fixed landmarks and deliberate omission/material-spill failures. Prior portrait
regions, mappings and all 544 retained portrait PNG/metadata files remain exact.
No runtime or installer code changed. Wiscar's four native `PortraitsMisc` world
assignments are preserved; his other world strips and all other new characters'
world strips use `Default`.

## Integration checks

The selected collection has 5,387 source strips: 2,201 portraits and 3,186 world,
with 21,548 nonvanilla variants for 36 characters. Twenty-nine characters have
complete world-source coverage. All 10,774 selected PNG/metadata members match
both the preserved archive and the supplied `tmp/fom/assets.zip`; this does not
establish general game-update compatibility.

The baseline omission inventory now has 478 covered strips. The remaining 973
raw omissions are 925 authoring/review strips in P3/P4, thirty audited shadows
and eighteen audited spectral no-change strips. P2 has no unaccounted source
strips; art acceptance retains its separate limits.

Fresh formatting, all-target Clippy with warnings denied, 434 regular tests
(500 ignored), release build, fourteen affected material/package/retained cases
and five GML interpreter cases pass. The combined five-character build matches
all 1,448 author PNG/metadata files exactly. Complete galleries bind 57,883,776
preview pixels and pass 212 author CPU desktop/mobile browser loads.

A parent spot-check raised a possible missing-header issue in the Linnet/Wiscar
samples. The native raster check found zero mismatches across both 88,000-pixel
header regions against the literal SAMPLE titles and five choice labels.
Regeneration was byte-identical. A focused follow-up rechecked 11,173,440 bound
pixels and 42 browser loads; masks, recipes and tests did not change. The cameo
record and `local/authoring/cameos-world/summary-label-audit.json` own that evidence.

The full CLI install generated all 21,548 variants and passed strict MOMI
compilation plus installed atlas-pixel, native animation-metadata and all three
production-script verification. The actual install report matches all 21,096
prior variant PNG hashes to the previous checkpoint, all 724 new-character
PNGs to the author bundles, and 2,568 PNGs to the separately frozen accepted
Reina/Wheedle/Zorel/Taliferro/Vera snapshots. All 975 retained recipe/gallery
fingerprints still match. Removal restored the exact source hash above and
removed recovery state. No real user game installation was modified.

Standard/material/GML evidence is `tmp/fourth-batch-checks.log`; installation and
removal are `tmp/fourth-batch-full-install.log` and
`tmp/fourth-batch-full-uninstall.json`. The small installation summary, independent
source inventory, retained/candidate hash pins, combined configuration, review
builder and reproduction scripts are under `local/authoring/fourth-batch/`.
The check summary is `tmp/fourth-batch-check-results.json`; collection counts and
baseline reconciliation are the neighboring fourth-batch JSON files.

## Consolidated review and limits

The [review](../../tmp/fourth-batch-review/index.html) and portable
`tmp/fourth-batch-review.zip` include all 462 native/mirrored cases in small pages.
The five-choice images are samples, ordered Vanilla, Debug Blue, Hayden, Ryis,
Seridia. Complete comparisons put Vanilla left and Debug Blue right, with full
sprites and enlarged details. No new candidate has been written to an accepted
art fixture. The user's next assessment concerns Stillwell and the four cameos;
prior displayed-art approvals remain unchanged.

The user subsequently identified Linnet's apparently unrecolored scar, considered
that treatment expected and requested a commit after confirmation. Original
portrait/world art and two fresh source-backed material tests confirm that the
peach/pink scar is preserved while surrounding skin changes. No art data changed;
evidence is `tmp/linnet-scar-confirmation.log` and the cameo record. This records
the scar assessment and explicit commit authorization, without treating the
commit request as broader fourth-batch or live-game acceptance.

The consolidated page and all five character indexes passed twelve additional
CPU desktop/mobile browser loads: all images and links are available, with no
horizontal overflow. The portable ZIP carries the complete galleries, recipes,
check summary and file fingerprints. `generated/review` and `generated/build`
point to the completed fourth-batch review and combined build; historical review
directories and accepted fixtures are retained separately.

On 2026-11-06, fifth-batch preparation found a copied path in the review builder:
the fourth ZIP's `validation.html` contained the third batch's development
record. Artwork, recipes and the fourth-batch check summary were correct. The
original ZIP is preserved as `tmp/fourth-batch-review-original.zip`; only the
verification page and its portable-manifest hash are corrected in the current
ZIP. Every other entry, including all images and candidate fingerprints, remains
byte-identical. The builder now reads this owning record. This report correction
does not change artwork or recorded acceptance.

This batch used CPU tools. Rendered menu, physical input, natural gameplay and
release packaging remain unverified P6 gates, including natural cameo cutscenes.
The existing isolated 601 MiB archive lab was reused and is exactly restored;
no extra source archive copies were created. Author evidence totals about 47
MiB, plus the small consolidated review and reusable combined build. Accepted
snapshots, historical reviews and useful shared Rust caches remain intact.
