# Security policy

`rbx-ops` handles credentials: an Open Cloud API key on every command, and,
for `group members` given a role, a Roblox session cookie. It also sends the
secrets `secret` manages. That makes the threat model worth stating.

## Reporting a vulnerability

Report it privately, through GitHub: on this repository, open the **Security**
tab and choose **Report a vulnerability**, or go straight to
<https://github.com/rbx-forge/rbx-ops/security/advisories/new>.

Do not open a public issue for a vulnerability. Do not paste a real API key,
cookie or secret into a report: a redacted reproduction is always enough.

What helps: the `rbx-ops --version`, the command you ran, what you expected,
what happened, and why it is a security problem rather than a bug. A proof of
concept is welcome but never required.

**What to expect.** This is a solo-maintained project. An acknowledgement
within a week is the realistic commitment; a fix lands as fast as the severity
warrants. You will be credited in the advisory and the changelog unless you
ask otherwise. There is no bounty program.

## Supported versions

Only the latest release. At 0.x, fixes go out as a new release rather than
being backported.

## What counts

In scope:

- A credential (API key, cookie) or a secret's value written to disk, logged,
  printed, or sent anywhere other than the Roblox host the command is talking
  to.
- The Studio cookie read without asking, or on a run that has nobody to ask.
- A write sent on a dry run (a command that takes `--apply`, run without it),
  or without the confirmation it asks for.
- A path traversal or arbitrary write from remote-controlled data.
- Dependency vulnerabilities with a plausible path to any of the above.

Out of scope:

- An API key doing what its scopes allow. A key scoped too broadly is fixed
  where the key was created.
- Anything requiring an attacker who already has your credentials, or write
  access to your machine or repository.
- Rate limits, quotas, and Roblox-side API behavior. Report those to Roblox.

## Verifying a download

Each release archive carries a build provenance attestation, a signed
statement of which workflow run and commit produced it:

```sh
gh attestation verify rbx-ops-v0.0.0-x86_64-unknown-linux-gnu.zip --repo rbx-forge/rbx-ops
```
