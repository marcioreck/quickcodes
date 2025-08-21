# QuickCodes Rulesets Setup Guide

This document provides a comprehensive guide for setting up and maintaining the basic rulesets for the QuickCodes project. These rulesets ensure code quality, security, and consistent development practices.

## Overview

The QuickCodes project uses a multi-layered approach to enforce quality and security standards:

1. **Code Quality Tools**: rustfmt, clippy, cargo audit
2. **GitHub Features**: Branch protection, pull request templates, issue templates
3. **CI/CD Pipeline**: Automated testing, security scanning, cross-platform verification
4. **Dependency Management**: Dependabot for automated updates
5. **Security**: CodeQL analysis, vulnerability reporting

## Files Added/Modified

### Configuration Files

- **`rustfmt.toml`**: Rust code formatting configuration
- **`clippy.toml`**: Clippy linting configuration  
- **`.gitignore`**: Enhanced to exclude build artifacts and temporary files
- **`RULESETS.md`**: Repository ruleset documentation
- **`SECURITY.md`**: Security policy and vulnerability reporting

### GitHub Templates

- **`.github/ISSUE_TEMPLATE/bug_report.yml`**: Structured bug report template
- **`.github/ISSUE_TEMPLATE/feature_request.yml`**: Feature request template
- **`.github/pull_request_template.md`**: Pull request template

### Workflow Files

- **`.github/workflows/codeql.yml`**: CodeQL security analysis
- **`.github/dependabot.yml`**: Enhanced dependency management

## Ruleset Components

### 1. Code Quality Standards

#### Formatting (rustfmt)
```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

**Configuration in `rustfmt.toml`:**
- Max line width: 100 characters
- Unix-style line endings
- 4-space indentation
- Reorder imports automatically

#### Linting (clippy)
```bash
# Run clippy with project-specific rules
cargo clippy --all-targets --features "png,svg" -- -D warnings -A clippy::needless_range_loop -A clippy::collapsible_if -A clippy::incompatible_msrv
```

**Configuration in `clippy.toml`:**
- Cognitive complexity threshold: 25
- Type complexity threshold: 100
- Maximum arguments: 8
- MSRV: 1.70.0

### 2. Branch Protection Rules

#### Main Branch (`main`)
- **Required Checks**: All CI jobs must pass
- **Required Reviews**: Minimum 1 approval
- **Up-to-date requirement**: Branch must be current
- **Force push**: Disabled
- **Deletion**: Disabled

#### Development Branch (`develop`)
- **Basic CI checks**: Required
- **Review requirement**: 1 approval for external contributors
- **Force push**: Maintainers only

### 3. Security Measures

#### Automated Security Scanning
- **CodeQL**: Weekly security analysis
- **Cargo Audit**: Dependency vulnerability scanning
- **Dependabot**: Weekly dependency updates

#### Vulnerability Reporting
- **Private reporting**: Enabled via GitHub Security tab
- **Response timeline**: 48 hours acknowledgment, 7 days detailed response
- **Disclosure policy**: Coordinated disclosure after fix

### 4. Issue and PR Management

#### Issue Templates
- **Bug reports**: Structured with reproduction steps, environment details
- **Feature requests**: Include use case, proposed API, implementation complexity

#### Pull Request Requirements
- **Description**: Clear summary of changes
- **Testing**: Evidence of testing
- **Documentation**: Updated if APIs changed
- **Breaking changes**: Clearly marked
- **Checklist**: Quality assurance items

## Setting Up the Rulesets

### 1. Repository Settings

To fully implement these rulesets, repository administrators should:

#### Branch Protection Rules
1. Go to Settings → Branches
2. Add protection rule for `main`:
   ```
   - Require status checks: ✓
   - Require branches to be up to date: ✓  
   - Required status checks: check, test, coverage, security
   - Require pull request reviews: ✓
   - Required approving reviewers: 1
   - Dismiss stale reviews: ✓
   - Restrict pushes to matching branches: ✓
   - Allow force pushes: ✗
   - Allow deletions: ✗
   ```

#### Security Settings
1. Go to Settings → Security & analysis
2. Enable:
   - Dependency graph
   - Dependabot alerts
   - Dependabot security updates
   - Code scanning (CodeQL)
   - Secret scanning

### 2. Required Secrets

Add these secrets in Settings → Secrets and variables → Actions:
- `CODECOV_TOKEN`: For code coverage reporting
- `CARGO_REGISTRY_TOKEN`: For automated publishing (if needed)

### 3. Local Development Setup

Developers should:

1. **Install Rust tools**:
   ```bash
   rustup component add rustfmt clippy
   cargo install cargo-audit
   ```

2. **Pre-commit hooks** (optional but recommended):
   ```bash
   # Add to .git/hooks/pre-commit
   #!/bin/sh
   cargo fmt --check || exit 1
   cargo clippy --all-targets --all-features -- -D warnings -A clippy::needless_range_loop -A clippy::collapsible_if -A clippy::incompatible_msrv || exit 1
   cargo test --all || exit 1
   ```

## Enforcement Mechanisms

### Automated Enforcement
- **CI/CD Pipeline**: Prevents merging of non-compliant code
- **Branch protection**: Enforces review and status check requirements
- **Dependabot**: Automatically creates PRs for security updates

### Human Review Process
- **Code owners**: Automatic review assignment for core changes
- **Pull request templates**: Ensure complete information
- **Issue templates**: Standardize bug reports and feature requests

## Monitoring and Maintenance

### Weekly Tasks
- Review Dependabot PRs
- Check security alerts
- Monitor CI/CD performance

### Monthly Tasks  
- Review and update ruleset documentation
- Analyze code coverage trends
- Update issue/PR templates if needed

### Quarterly Tasks
- Review MSRV compatibility
- Update clippy configuration
- Assess security posture

## Customization Guidelines

### Adding New Rules
1. Update appropriate configuration file (`clippy.toml`, `rustfmt.toml`)
2. Test changes locally
3. Update CI workflows if necessary
4. Document changes in this guide

### Exemptions Process
1. Create issue describing why exemption is needed
2. Get approval from maintainers
3. Document exemption reason
4. Set time limit for review

### Tool Updates
1. Test tool updates in development branch
2. Update CI workflows
3. Update local development instructions
4. Communicate changes to team

## Troubleshooting

### Common Issues

#### Clippy Failures
```bash
# Check specific lint
cargo clippy --all-targets -- -W clippy::specific_lint

# Allow specific lint temporarily  
cargo clippy --all-targets -- -A clippy::specific_lint
```

#### Formatting Issues
```bash
# See what would change
cargo fmt --check

# Apply all formatting
cargo fmt
```

#### CI Failures
- Check if all required secrets are set
- Verify branch protection rules are correctly configured
- Ensure all required status checks are defined

### Getting Help

- **Issues**: Use the bug report template
- **Discussions**: For questions about rulesets
- **Security**: Use private vulnerability reporting
- **Urgent**: Contact maintainers directly

## Conclusion

These rulesets provide a comprehensive foundation for maintaining code quality and security in the QuickCodes project. They balance automation with human oversight to ensure consistent, secure, and maintainable code.

Regular review and updates of these rules ensure they continue to serve the project's evolving needs while maintaining high standards for contributors and users.