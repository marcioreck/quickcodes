# Security Policy

## Supported Versions

We actively support the following versions of QuickCodes with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in QuickCodes, please report it responsibly.

### Private Reporting

For security vulnerabilities, please use GitHub's private vulnerability reporting feature:

1. Go to the [QuickCodes repository](https://github.com/marcioreck/quickcodes)
2. Click on the "Security" tab
3. Click "Report a vulnerability"
4. Provide detailed information about the vulnerability

### What to Include

Please include the following information in your report:

- **Description**: Clear description of the vulnerability
- **Impact**: Potential impact and severity assessment
- **Reproduction**: Step-by-step instructions to reproduce the issue
- **Environment**: Rust version, operating system, and QuickCodes version
- **Proposed fix**: If you have suggestions for fixing the vulnerability

### Response Timeline

We will acknowledge receipt of your vulnerability report within **48 hours** and provide a detailed response within **7 days** indicating:

- Confirmation of the vulnerability
- Our assessment of its impact and severity
- Timeline for developing and releasing a fix
- Whether public disclosure is appropriate

### Disclosure Policy

- We will work with you to understand the scope and impact of the vulnerability
- We will develop a fix and coordinate the release timeline with you
- We will publicly disclose the vulnerability after a fix is available
- We will credit you for responsible disclosure (unless you prefer to remain anonymous)

### Security Best Practices

When using QuickCodes, please follow these security best practices:

1. **Keep dependencies updated**: Regularly update QuickCodes and its dependencies
2. **Validate input data**: Always validate and sanitize input data before generating barcodes
3. **Error handling**: Properly handle errors and don't expose sensitive information
4. **Audit dependencies**: Use `cargo audit` to check for known vulnerabilities
5. **Secure deployment**: Follow secure coding practices in your applications

### Security Features

QuickCodes includes the following security features:

- **Input validation**: Comprehensive validation of input data
- **Error handling**: Safe error handling without information leakage
- **Memory safety**: Built with Rust for memory safety guarantees
- **Dependency management**: Regular automated dependency updates via Dependabot
- **CI/CD security**: Automated security scanning in our build pipeline

### Known Security Considerations

- **Input size limits**: Very large input data may cause memory issues
- **Resource consumption**: Complex barcodes may consume significant CPU/memory
- **Output validation**: Generated barcodes should be validated in production systems

### Contact Information

For non-security issues, please use our regular issue tracker:
- **Issues**: https://github.com/marcioreck/quickcodes/issues
- **Discussions**: https://github.com/marcioreck/quickcodes/discussions

For urgent security matters, you can also contact:
- **Email**: marcio@fazmercado.com (please include "QuickCodes Security" in subject)

## Security Updates

Security updates will be released as patch versions and will be clearly marked in:
- Release notes
- CHANGELOG.md
- Security advisories

We recommend enabling GitHub's dependency alerts and security updates for your projects using QuickCodes.