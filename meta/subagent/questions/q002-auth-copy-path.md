# Q002 Auth Copy Path

In cold-start Docker validation, where should opencode auth credentials be copied so `opencode auth list` can read them without local mounts?

## Expected Baseline

- Copy auth file to `~/.local/share/opencode/auth.json` in the container user home.
- Keep auth payload in JSON object keyed by provider id.
- Do not mount host auth/data directories for this check.

## Verdict Contract

- `PASS`: points to `~/.local/share/opencode/auth.json` and explains no-mount constraint.
- `WARN`: path mostly correct but explanation on XDG/data path is incomplete.
- `FAIL`: uses mounted host path or wrong in-container destination.
