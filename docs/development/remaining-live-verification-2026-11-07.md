# Remaining live gameplay verification — 2026-11-07

Status: authorized seven-case batch executed; exact original lab restored.
Portable packet integrity and CPU browser checks pass.
The displayed samples have qualified owner acceptance. Prior artwork and
qualified basic-live approvals retain their recorded scope.

## Candidate and isolation

The approved [bounded plan](../plans/remaining-live-verification.md) reuses the
current twelve-choice candidate: Vanilla plus eleven generated families, including
Player 27. Production Rust/GML, recipes and selected artwork were not changed.
The retained Nix package has 356 files matching the previously verified candidate.

A fresh strict install into `tmp/finish-batch-install-lab` passed in 640.502 seconds:
69,432 generated bindings, 6,312 original PNG/metadata pairs, and 219,780 original
and variant native-frame validations. Runtime GML and generated table bytes match
the current package. The installed archive fingerprint was
`5cd0a880af530bca22a1091750646313b4b33721afd26bcf433b069d77a1cca8`;
table SHA256 was `5f43b73f33f3803442cc1da3c0918bd39806cf6867c1b6863fb279fd74264cc4`.
Fresh receipts are under `tmp/remaining-live-verification/`; historical Player 27
receipts were not overwritten.

All rendering used hardware AMD rendering in private Gamescope/X11 sessions,
checked with `gpu-status` and held under foreground `gpu-run` reservations.
Game saves/configuration were copied into disposable branches. Native OS events
operated the released journal and game UI. Built-in console commands prepared
calendar, clock, travel and selected scenes/activities; their use is explicit in
the matrix. There were no arbitrary fact/save edits, custom scenes or new runtime
probe hooks. Real user saves were not exposed to the game.

## Results and limits

The machine-readable record is `tmp/remaining-live-verification/matrix.json`.
Each row identifies stable palette IDs, the actual observed form, preparation,
media, receipts and unobserved behavior. The validated portable review is linked
below.

| Case | Observed result | Material limit |
| --- | --- | --- |
| Adeline seasons and Spring→Summer | Spring/Summer Player 27, Fall Debug Blue, Winter Vanilla portrait/world anchors; Hayden/Player 35/Vanilla world changes; native sleep and Summer 1 autosave | 25-second sleep clip misses the final NextDay activation; logs/save establish it. Some palette choices are world-only because same-day talk flags suppress repeat portraits. |
| Juniper Beach→Summer | Calm Summer Wednesday, Player 27 Beach portrait, native 18:45 outfit change/departure; Summer seated actor after room recreation | Debug calendar/clock preparation; subsequent full Summer portrait not captured. |
| Elsie Winter writing | Player 27 arrival/writing, journal pause/change to Blue, resumed writing and native departure on Winter 23; Vanilla and full Player 27 portrait anchors | Direct NPC teleport initially obstructed movement; that attempt is excluded. Exact internal loop phase was not measured. |
| Terithia fishing/baiting/West | Player 27 wave fishing, Blue requested dock baiting, native West walk and West-facing horizontal fishing | Travel partly simulated by clock jumps. Dock baiting is debug-triggered. No full portrait sample or distinct complex end-strip dispatch established. |
| Juniper special chain | Summer→Beach swim/spell→Beach Accident→cure→Beach→Summer West-seated portrait/world, retaining Player 27 | Direct debug scene bypasses relationship eligibility. Water, curtain and native overlays obscure portions of the world actor. Ending/story cleanup not exercised. |
| Spectral Priestess | Direct scene shows native blue spectral portrait, arrival, spell/prop effects, idle and alpha/removal/cleanup with Player 27 selected | Intentionally unchanged spectral form, not recolorable Flashback Priestess. Generic early-dungeon warp failed before actual scene route. |
| Wedding | Native Continue/Yes completion; Player 27 wedding portrait/world, ceremony construction/aisle and South-seated reception actor recreation | Native Juniper fallback. Opening/preparation and later ceremony/vows/kiss are simulated. Full spouse reception conversation and story cleanup not exercised. |

These are representative integration observations. The plan's requirement for a
complete distinct complex-animation start→loop→end dispatch is **not established**:
native activity expiry and script animation changes reset the pack, rather than
proving the separate end strip. This criterion remains open and requires further
verification or an explicit accepted limit. Native West dispatch is visible.
Physical-controller acceptance, exhaustive seasonal/form/action combinations,
recolorable Flashback Priestess, unsampled artwork and broader approximate tone
coverage are not closed by this batch. A visual review cannot establish physical
hardware behavior or omitted animation dispatch.

The subsequently authorized
[native end-route investigation](animation-end-route-investigation-2026-11-07.md)
found no suitable bounded native caller and passed four CPU controls using the
unchanged native handler. It did not add live evidence or close the end-stage
criterion; the accepted packet remains exact.

## Owner feedback

On 2026-11-07 the owner replied "looks okay to me" to the request to assess the
displayed colors and transitions in the linked seven-case review. Record this as
qualified acceptance of the displayed samples, with inspection depth unspecified
and no new defect reported. Vanilla was identified as the baseline, Debug Blue as
the recoloring diagnostic and Player 27 as deep umber. The exact reviewed ZIP is
identified by its fingerprint below; the packet remains unchanged.

This feedback closes the displayed-sample assessment. It does not waive the
physical-controller or distinct complex-animation end-stage criteria, accept
unsampled forms/actions or broader approximate tone coverage, or establish
ordinary story progression through debug-triggered scenes. Release licensing
and publication remain separate decisions.

## Setup failures and controls

- A large date advance from an unplayed early save queued repeated pet-dream
  scenes. A retry completed the native pet prompt before advancing. Failed
  preparation is retained, not counted as a seasonal result.
- A debug warp during native EndDay produced an Adeline pathfinding error.
  Reloading the same Summer 1 save and conversing succeeds both on the candidate
  and on the exact supported original archive. The original timing sequence was
  not reproduced, so the error's cause remains unclassified.
- The helper's `--baseline` archive is the supplied launch archive
  (`bb40a6a3…d117b`), not the preserved supported original (`8354ff03…04fbe0b4`).
  That first comparison was corrected and is not the supported-original control.
  A dedicated read-only lab bound `local/game/assets.zip` for the valid control.
  Source-version differences and critical scene-member comparisons are retained.
- Direct NPC teleports can place Ari inside furniture and block departure.
  Moving to an existing nearby trellis waypoint and then onto clear floor restored
  Elsie's native route. Corrected footage establishes the schedule/action result.
- An early menu-input attempt did not reach the palette menu, then Interact
  opened native Sleep. The config assertion caught the failure; the attempt is
  excluded. Later input followed a confirmed normal-world capture.
- Generic `goto priestess_quarters/pq2_ari_start` on the early branch failed in
  native `enter_dungeon.gml:25` with `bad object undefined`. The actual
  `mist run priestess_quarters_pt_2` route initializes the dungeon and succeeds.
  Recovery used two launches within the original 900-second elapsed feasibility
  allocation, enforced with a remaining-time timeout. This is an explicit
  adjustment from the planned single-launch session, without an extended budget.
  Native quest cleanup warns that `find_the_magic_key` was absent because debug
  entry bypassed normal progression. Neither warning is a classified palette bug.

Wedding customization first activated highlighted No with E, then completed via
an explicit Yes click. The retained 45-second editor clip ends before the Yes click
at 51.169 seconds; stills, OS input events and subsequent ceremony progression
establish completion. Native checkpoint simulation emits textbox-not-visible
warnings. Reception setup also logs `Invalid food item: undefined`; its cause is
unclassified, while rendering and the target actor sample continue. No palette
failure is inferred from these native debug warnings.

## Technical evidence

The new FFV1 recorder captures only the private game window at 30 fps, for 1–60
seconds, with decoded frames, timestamps, geometry and source hashes. Twelve
focused authenticated Xvfb checks cover capture bounds, literal console text,
no-overwrite behavior, failures and cancellation. The pinned closure uses
FFmpeg 8.1.2 with X11 capture. Raw clips remain lossless and immutable.

Playable H.264/yuv420p previews are explicitly lossy derivatives. Each receipt
binds the original clip hash, matching decoded frame count/duration/geometry,
strict full decoding and twelve timestamped raw-frame inspection samples.
Raw files and source receipts remain private; the portable packet contains
labelled previews/stills and relative-path receipts, without saves or archives.

A fresh CPU spectral check of this exact installed archive passes all 18 original
Priestess PNGs and 18 metadata byte comparisons, and all 48 visible native RGBA
atlas frames across 11 pages. A fresh changed-visible-pixel control fails as
expected. Both archives remain exact. Fully transparent hidden RGB is outside
the atlas helper's visible-pixel comparison. See
`tmp/remaining-live-verification/spectral/check.json` and its source binding.

## Restoration, review and retention

Uninstall passes in 6.507 seconds, restoring the exact source archive
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The recovery directory is absent. Fresh preservation checks pass all 1,182 frozen
production inputs, eight original seed/config files and 31 historical receipts.
All ten owned game launches ended, their PIDs are absent and GPU reservations
are released. The writing/beach session reached its existing 900-second cap;
that supervised termination is not a game-crash finding.

The [seven-case review](../../tmp/remaining-live-verification/review/index.html)
and [portable ZIP](../../tmp/remaining-live-verification/review.zip) contain 24 stills,
15 playable clips and relative-path receipts (59 unique evidence files, 62 ZIP
entries). ZIP size is 43,034,822 bytes; SHA256 is
`9a1a0d26bc62589dd18a51af209ef2cda4b8cb689f3eafe488284f51e6557fd8`.
The packet contains no game archives or saves. The detached
[review checks](../../tmp/remaining-live-verification/review-checks.json) pass all
62 file/ZIP bindings and 132 local links. CPU Chromium 155 loads all 24 images and
decodes a midpoint frame after seeking each of the 15 clips at both 1440×1000 and
390×844, without horizontal overflow or playback limitations. The checker's
loopback HTTP server needed byte-range support; the corrected check passes with
the frozen packet unchanged. Focused helper tests cover the range responses and
reject broken bindings, media and links. Browser sampling does not establish
every-frame playback or audio; full FFmpeg decoding is recorded separately in
the derivative receipts. Direct `file://` playback was not tested.

Private task sessions retain 3,566,272,278 bytes, including 3,474,135,105 bytes of
lossless clips; task outputs/previews/packet use about 237 MB at inventory time.
These include failed setup evidence needed to interpret the successful retries.
The scoped inventory is `tmp/remaining-live-verification/retention.json`.
Original sources, prior packets and shared Nix caches are preserved. No game or
recorder job remains. Owner acceptance is limited to the displayed samples above;
release licensing, physical-controller acceptance and publication remain open.
