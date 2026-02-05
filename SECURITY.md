# Security Policy

## Supported Versions

| Version | Supported | End of Life |
|---------|-----------|------------|
| 0.1.x   | ✅        | TBD        |
| 0.0.x   | ❌        | 2024-06-30 |

We support the latest released version and the previous minor version for security fixes.

## Reporting a Vulnerability

**Do not open public issues for security vulnerabilities.** Instead, please report security issues responsibly through private channels.

### How to Report

1. **Email**: Send a detailed report to [team email] with subject line: `[SECURITY] Vulnerability in Waylestia`

2. **Include**:
   - Type of vulnerability (e.g., privilege escalation, information disclosure)
   - Affected component (core, widgets, apps, protobuf)
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if you have one)

3. **Timeline**:
   - You should receive acknowledgment within 48 hours
   - We'll provide a security advisory within 14 days
   - Fixes will be prioritized based on severity

### Disclosure Process

- We will acknowledge receipt of your report
- We'll investigate and confirm the vulnerability
- We'll develop and test a fix
- We'll release a patched version
- We'll publish a security advisory
- We'll credit you (unless you request anonymity)

## Security Best Practices

### For Users

- **Keep Updated**: Always use the latest supported version
- **Review Permissions**: Check widget permissions before installing
- **Monitor Services**: Use `journalctl --user -u waylestia-*` to monitor for issues
- **Secure Installation**: Don't run install scripts with sudo unless necessary
- **Report Issues**: Report suspicious behavior through proper channels

### For Developers

- **Input Validation**: Always validate user input and IPC messages
- **Error Handling**: Don't expose sensitive information in error messages
- **Temporary Files**: Use secure temporary file handling
- **Dependencies**: Keep dependencies up to date
- **Code Review**: Request review for security-sensitive code
- **Testing**: Test edge cases and error conditions

## Security Considerations

### Architecture

The Waylestia security model includes:

1. **Protobuf-Based IPC**: Type-safe message validation
2. **Permission System**: Widget-level permission control
3. **Audit Logging**: Security events logged to journald
4. **Sandboxing**: Services run in user context (not as root)
5. **Type Safety**: Rust memory safety guarantees

### Known Limitations

- Widgets run in same context as core (no sandboxing)
- Assets directory permissions should be restricted
- Configuration files may contain sensitive data
- Hyprland IPC is trusted (runs on same display)

### Future Hardening

- [ ] Widget sandboxing via bubblewrap
- [ ] Cryptographic widget verification
- [ ] Encrypted configuration storage
- [ ] IPC authentication layers
- [ ] Rate limiting on public APIs
- [ ] Fuzzing-based security testing

## Vulnerability Categories

We take the following types of vulnerabilities seriously:

- **Critical**: Remote code execution, privilege escalation
- **High**: Information disclosure, denial of service
- **Medium**: Logic flaws, permission bypasses
- **Low**: Documentation issues, minor bugs
- **Informational**: Best practice recommendations

## Security Update Schedule

- **Critical**: Released immediately
- **High**: Released within 7 days
- **Medium**: Released with next minor version or within 30 days
- **Low**: Released with next minor version

## Security Headers

When Waylestia hosts web content (upcoming), we follow:

- Content Security Policy (CSP)
- X-Frame-Options
- X-Content-Type-Options
- Strict-Transport-Security (for HTTPS)
- Referrer-Policy

## Dependencies

### Trusted Dependencies

Waylestia uses only well-maintained, trusted crates:

```
# Core dependencies
tokio         - Async runtime (maintained by Tokio team)
serde         - Serialization (community standard)
prost         - Protobuf (actively maintained)
hyprland      - Wayland integration (community)
sysinfo       - System monitoring (well-reviewed)

# GJS (via system packages, not cargo)
gjs           - JavaScript engine for GTK
```

### Dependency Scanning

We regularly scan dependencies for vulnerabilities using:

- `cargo audit` - CVE checking
- `cargo outdated` - Update availability
- GitHub security alerts
- Manual security review

## Cryptography

Waylestia does not currently implement cryptographic operations. Future crypto:

- Widget signature verification: RSA-4096 or Ed25519
- Configuration encryption: AES-256-GCM
- TLS for remote services: TLS 1.3+

All crypto will use proven libraries (ring, sodiumoxide, rustls).

## Security Contacts

- **Security Team**: [team email]
- **Main Maintainer**: [lead dev email]
- **Issue Tracker**: GitHub Issues (non-security use only)

## Acknowledgments

We appreciate responsible security research. Credit will be given to:

- Security researchers who report vulnerabilities
- Contributors who improve security
- Community members who raise concerns

## References

- [OWASP Top 10](https://owasp.org/Top10/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Wayland Security Model](https://wayland.freedesktop.org/)

---

**Last Updated**: 2024
**Version**: 1.0

Thank you for helping keep Waylestia secure!
