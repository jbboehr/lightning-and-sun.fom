# Representative Player27 review sheet

2026-11-07. The user requested a review sheet with a representative sample of
each character. The sheet is technically checked and has qualified owner
acceptance of the displayed samples; see [owner feedback](#owner-feedback).

Open the [scrolling review sheet](../../tmp/player27-roster-sheet/index.html),
[six-page PDF](../../tmp/player27-roster-sheet/review-sheet.pdf), or
[PNG sheets ZIP](../../tmp/player27-roster-sheet/review-sheets.zip).
All 36 characters appear alphabetically, six per page. Columns read **Vanilla,
Player22, Player27, Player35**; the new Player27 column is highlighted.

Each row uses the character's neutral Spring portrait and second native frame.
The selection reuses the complete gallery's representative-portrait ranking.
The same 112 × 112 crop is used across all four choices for a character, enlarged
exactly two times with nearest-neighbor sampling. Dell and Luc use lower crops
to retain their complete faces. Preparing the sheet changed no game image,
palette, profile, runtime or acceptance baseline.

The [selection record](../../tmp/player27-roster-sheet/selection.json) identifies
every source asset, frame, crop, choice, source hash and sheet location. The
[checks](../../tmp/player27-roster-sheet/checks.json) verify:

- Exactly 36 unique characters, alphabetical order, six sheets and 144 source
  tiles. Every rendered PNG tile matches its crop from the previously verified
  current artwork, including the checkerboard alpha composition.
- All six HTML image payloads equal the exported PNGs. Desktop 1440 × 1000 and
  phone 390 × 844 direct-file loads decode every sheet without horizontal overflow.
- The PDF contains six pages. The seven ZIP members (six PNGs and selection
  record) match the files exactly and pass integrity checks.
- All 53 previously reviewed technical inputs, the complete review ZIP and the
  original rollout integration receipt remain unchanged.

Sol inspected all six pages and corrected the first preview's Dell/Luc framing.
The final packet occupies about 3.1 MiB; the preceding preview is retained
separately for the cropping comparison. Existing full review and installation
evidence remain frozen. Only CPU composition and browser export were needed;
runtime/package/GPU checks were not repeated for this presentation-only slice.

These representative portraits are the requested compact entry point. The
[complete comparison](../../tmp/player27-rollout/review/index.html) and
[expressive cosmetic focus](../../tmp/player27-rollout/review/cosmetics.html)
retain broader expression, outfit and world-sprite context.

## Owner feedback

On 2026-11-07, after receiving the HTML, PDF and PNG download links, the owner
said **"looks okay to me"**. Record qualified acceptance of the displayed
representative portraits, with inspection depth unspecified and no new defect
reported. The review contains 36 neutral Spring portraits, native frame 2,
shown as 144 cropped tiles across Vanilla, Player22, Player27 and Player35.
The particular format or pages inspected were not specified.

The [feedback receipt](player27-review-sheet-feedback.json) binds the exact
delivered artifacts and selection record by SHA256. A separate
[accepted presentation snapshot](../../local/fixtures/player27-representative-review/)
preserves the six displayed PNG sheets and their selection record. This freezes
only the reviewed presentation; it does not make all 6,312 new strips accepted
artwork. The preceding preview and historical technical receipts remain unchanged.

This feedback closes the requested representative review. Unsampled expressions,
outfits, world sprites and complete cosmetic or approximate-coverage acceptance
are not inferred. Existing coverage decisions and broader live/release gates
retain their scope. No commit or publication is authorized by this feedback.
