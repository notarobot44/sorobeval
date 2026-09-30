# Context files for the "skill" condition

Put SDF's public `stellar-dev-skill` content here as `stellar-dev-skill.md`
(copy the SKILL.md and any reference files it ships, concatenated). The
runner prepends it to the system prompt for every sample in the `skill`
condition and records its SHA-256 in each sample's metadata, so the exact
version tested is reproducible.

The `bare` condition uses only `system_prompt.md`.
