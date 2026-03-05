# Q001 Resource Boundary

Should `cowork clone resource` enforce strict file schema checks (hard-coded extensions/types), or remain a print-only guideline?

## Expected Baseline

- Keep `clone resource` as print-only guidance.
- Do not hard-code file schema checks in cowork.
- Reasoning must align with the boundary: cowork is transport/tooling layer, target repo baseline belongs to target repo CI.

## Verdict Contract

- `PASS`: chooses print-only and explains boundary correctly.
- `WARN`: chooses print-only but reasoning is partial/ambiguous.
- `FAIL`: proposes hard-coded schema checks as default behavior.
