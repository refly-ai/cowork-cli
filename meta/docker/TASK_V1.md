# Cold-start Task V1

You are dropped into a cold-start environment.

## Available Context

- Knowledge repository URL
- cowork install script URL
- cowork beta version: `0.1.4-beta`

## Goal

1. Install and use the cowork workflow to understand the repository layout and collaboration boundary.
2. Make one minimal, safe documentation-only improvement that does not change repository meaning.
3. Create a draft PR with a concise summary of what you changed and why.

## Constraints

- Prefer understanding before editing.
- Do not make structural refactors.
- Do not delete content.
- Keep the change reversible and low risk.
- Treat `preview`, `contribute`, and `resources` as guide-only commands.
- Use `COWORK_HOME=/workspace/.cowork` for this task.
- Install the beta explicitly with `--version 0.1.4-beta`.

## Success Signals

- cowork is installed and usable.
- The repository boundary is understood before editing.
- The change is documentation-only and low risk.
- A draft PR is created with a concise why-focused summary.

## Observation Focus

- Whether the agent naturally installs cowork first.
- Whether the agent locates the local knowledge repository path correctly.
- Whether the agent correctly interprets guide-only cowork commands.
- Whether the agent chooses a safe first change.
- Whether the PR summary is coherent without extra human correction.
