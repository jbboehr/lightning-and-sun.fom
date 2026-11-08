# Native complex-animation end-route investigation — 2026-11-07

Status: authorized CPU investigation complete. No suitable bounded native route
to a recolorable character's distinct complex stop stage was found. The live
criterion remains open. No additional rendering or visual acceptance is claimed.

The owner approved this follow-up with "go for it" after the proposal to
investigate the missing route using CPU tools, with GPU confirmation if a suitable
native route was found. The accepted
[seven-case review](remaining-live-verification-2026-11-07.md#owner-feedback)
and its limits remain unchanged.

## Source and result

The preserved original archive hashes to
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The [source receipt](../../tmp/animation-end-route/source-route-check.json)
records member fingerprints and exact field references across its 561 GML and
196 MiST scripts. All 34 NPC configuration files parse; they contain 93 complex
cycle definitions. This is textual source inspection, not a formal proof of
whole-engine reachability or compatibility with other versions/mods.

The relevant native paths explain why the earlier fishing/writing samples do
not establish a distinct end strip:

- `NpcAnimationHandler.gml:22` resets changed packs to phase zero and
  `loops_requested = I32_MAX`. At lines 190–205, phase one enters phase two
  only when the accumulated loop count reaches that request. Phase two selects
  `pack.stop` through `retrieve_sprite` at lines 112–119.
- `set_loop_x_times` exists at lines 133–135, but has no caller in the inspected
  GML or MiST scripts. The default named infinite loop is a large finite integer;
  this finding concerns a practical bounded gameplay route.
- The only other assignment to `loops_requested` is
  `par_NPC.gml:1546`, guarded by the Dozy/Henrietta pet interaction at lines
  1519–1559. Dozy's `tailwag` and Henrietta's `crow` are both **linear** packs in
  their NPC configuration. Neither character is in the 36-character palette
  roster. These interactions do not supply the missing complex recoloring case.
- `ActivitiesAndRoutines.gml:184–197` clears an activity by selecting idle/sit.
  Replacing the pack resets the handler, without requesting its prior stop strip.
- MiST's `__animate` path in `Std.gml:481–504` selects an animation by name.
  `__await_npc_animation` at lines 566–569 checks whether the current sprite's
  next tick completes. Neither path requests a finite complex loop count.
  Cutscene cameos use the same handler and supply no additional finite caller.

The supplied launch archive has 559 GML and 196 MiST scripts. Its
[comparison receipt](../../tmp/animation-end-route/supplied-source-check.json)
confirms identical phase/request field references and the exact finite pet
interaction block. Some surrounding NPC source differs between the two archives;
the comparison does not assert whole-source equality.

## CPU execution

The [four-case receipt](../../tmp/animation-end-route/cpu-phase-check.json)
binds the interpreter, harness and native source hashes. The unchanged complete
native handler executes in Fabricator with synthetic one-frame sprite metadata:

1. The ordinary default remains in the loop phase after 240 simulated ticks.
2. An explicitly requested finite count visits start, loop and stop and completes.
3. Activity-style pack replacement resets the handler without the old stop stage.
4. Reselecting the same pack preserves the loop count and fractional frame.

All four cases pass. Reproduction uses
`local/tools/fabricator/bin/interpreter run tmp/animation-end-route/native-phase-cases.gml`.
The explicit finite request is a CPU positive control, not a discovered live
caller. These checks do not establish actual sprite pixels, palette substitution,
rendered gameplay, native timing/pathfinding or new visual acceptance.

## Preservation and next option

No suitable native route was established, so the conditional GPU confirmation
was not started. No installation, game launch, custom hook/scene, asset-archive
mutation or GPU reservation occurred. The
[preservation check](../../tmp/animation-end-route/preservation-check.json)
passes all 1,182 frozen production inputs, eight original seed/config files and
31 historical receipts. The original lab archive remains exact with no recovery
directory. The accepted review ZIP remains
`9a1a0d26bc62589dd18a51af209ef2cda4b8cb689f3eafe488284f51e6557fd8`.
Only the small CPU harness, logs and receipts are retained for this follow-up.

The recommended next option is a separately authorized isolated instrumented
check: start Terithia's existing fish activity, request a finite count through the
actual native handler after the pack is selected, and capture start→loop→stop
with Vanilla and Debug Blue. Live confirmation requires a checked/reserved local
GPU. Label the finite request as instrumentation; it would test native rendering
and recoloring at the end stage without establishing an ordinary gameplay caller.
No instrumentation is approved or implemented by this investigation. The original
live criterion and physical-controller acceptance remain open.

## Instrumented follow-up planning

2026-11-07: the owner replied "go for it" to Astra planning for the isolated
instrumented check. This approved planning only. The
[execution proposal](../plans/animation-end-live-check.md) records the bounded
recommendation and next approval checkpoint; no probe, archive patch or live run
has been implemented by this planning pass.

Astra recommends a single copied `BuggerInitialize.gml` console extension that
temporarily wraps Terithia's bound actor `animate` method, requests a finite count
only after fresh native fish selection, and calls the original actor method
exactly once per invocation. Native `par_NPC.gml:905–916,1602–1616` advances the
handler, reads the production-wrapped sprite and assigns the rendered asset.
This lets the proposed trace distinguish the original handler stop sprite from
the actual selected render variant without changing handler or palette logic.

Native `NpcAnimationHandler.gml:175–205` counts start completion toward the loop
total. On stop completion it sets `animation_is_over`, resets `complex_index` to
zero and calls `update_sprite`; fish has no reset-to-idle callback. Preserve the
resulting first-start-frame hold as native finite-request behavior. A one-frame
fish loop needs count/phase evidence rather than apparent movement. Astra's CPU
source/image inspection reports six Summer horizontal start frames, one loop
frame and five stop frames, with every visible stop pose distinct from every
start/loop pose; these findings do not establish live rendering.

Sol spot-checked the actual original handler against the prior receipt
(`b0f5935b793afd165b23b7e90a187b4399eeea3584058884a1c69213c7e44646`),
the native actor's animation/update call sites and the production
`lns_palette_world_sprite`/bound-method attachment. The existing launcher accepts
a separate archive override and copies seed state; the recorder captures
lossless 30-fps video for at most 60 seconds. The explicit current package link
still resolves to the package identified in the support record. These are
read-only source/path checks, not a fresh package build, install or runtime test.

Sol retains all five stop-subimage observations in the proposed trace but asks
the 30-fps footage to corroborate the distinct rendered stop stage, with each
actually observed pose identified. It cannot guarantee every native subimage
appears in video. Existing exact offline/atlas evidence remains separate from
screen colors under native lighting/compositing. No native speed change is
proposed to obtain footage.

Routing was verified from actual session metadata: parent
`01a1b90b-e7a8-7490-88f9-ae36ff73ad10` uses `gpt-6.1-sol`/`xhigh`; child
`01a1b90d-d2bb-7fb1-a0a9-6c7275197416` at `/root/animation_end_plan` uses
`gpt-6-astra`/`xhigh`. Astra returned a read-only plan and finished. Sol owns
planning-document updates and focused documentation validation. Planning used
no GPU or local inference and supplied no new art/live acceptance.

The [documentation receipt](../../tmp/animation-end-live-planning/documentation-checks.json)
records local link/anchor checks, the five-document change boundary and that
planning snapshot's document hashes. Whitespace checks and final diff inspection pass. No runtime,
build, installation or game tests were run for this documentation-only change.

The owner subsequently approved the bounded lab implementation and execution.
The separate [execution record](animation-end-live-verification-2026-11-07.md)
owns the implemented probe/CPU controls, fresh installation/removal and failed
rendered attempts. No native finite request ran; the original criterion remains
open. This follow-up does not change the earlier source investigation or art
acceptance limits.
