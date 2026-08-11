# Security Policy

## Supported Versions

Security fixes are applied to the latest GitHub Release.

## Reporting A Vulnerability

Do not open a public issue for a vulnerability involving bridge authentication,
token handling, local process control, or unintended network exposure.

Report it privately to [harry261@naver.com](mailto:harry261@naver.com) with:

- affected release version and Android/Termux versions
- reproduction steps and impact
- whether a token, terminal input, or shell output may be exposed

Do not include a live token, private key, or sensitive terminal output in the
report. Acknowledgement and a mitigation plan are normally provided within
seven days.

## Security Boundary

Termux Terminal's bridge is designed for `127.0.0.1` only and requires a local
installation token. User-created port forwards, tunnels, modified bridge
bindings, and copied tokens are outside the supported security boundary.
