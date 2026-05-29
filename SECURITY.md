# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The Daydream project takes security seriously, especially given our
privacy-first architectural mandate (COPPA, GDPR, FERPA).

**To report a vulnerability:**

1. **Do NOT open a public GitHub Issue.**
2. Email the maintainer directly at the address listed in the repository profile.
3. Include a description of the vulnerability, steps to reproduce, and potential impact.
4. You will receive an acknowledgment within 72 hours.

## Security Design Principles

- **Zero Cloud Leakage:** All AI inference runs locally. Student data never leaves the deployment environment.
- **No Secrets in Code:** API keys, tokens, and credentials must never be committed to this repository.
- **CORS Lockdown:** The backend restricts cross-origin requests to known deployment origins only.
- **Privacy-First Architecture:** PostgreSQL is self-hosted. No third-party analytics or telemetry.
- **Open Source Auditability:** The entire codebase is GPLv3-licensed and publicly auditable.
