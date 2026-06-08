# Security Policy

## Supported Versions

QuantumCart is experimental. Security fixes target the latest `main` branch until stable releases are introduced.

## Reporting a Vulnerability

Please do not open a public issue for sensitive vulnerabilities. Use GitHub private vulnerability reporting when enabled, or contact the maintainers privately.

## Security Notes

- Do not commit `.env` files, private keys, JWT secrets, database passwords, or cloud credentials.
- Set `REQUIRE_AUTH=true` outside local demos.
- Replace `JWT_SECRET` before any shared or public deployment.
- Keep Dependabot enabled for Cargo, Docker, and GitHub Actions updates.
- Treat webhook endpoints as untrusted input and add signature verification before production use.