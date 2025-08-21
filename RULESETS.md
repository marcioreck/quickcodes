# QuickCodes Repository Rulesets

This document outlines the basic rulesets and policies for the QuickCodes repository to ensure code quality, security, and maintainable development practices.

## Branch Protection Rules

### Main Branch (`main`)
- **Required status checks**: All CI checks must pass
  - Code formatting (`cargo fmt --check`)
  - Linting (`cargo clippy`)
  - Tests (unit, integration, doc tests)
  - Security audit (`cargo audit`)
  - Cross-platform compatibility (Linux, Windows, macOS)
- **Require branches to be up to date**: Yes
- **Required reviews**: At least 1 approval required
- **Dismiss stale reviews**: Yes
- **Require review from code owners**: Yes
- **Restrict pushes**: Only allow pull requests
- **Force push restrictions**: Not allowed
- **Deletion restrictions**: Not allowed

### Development Branch (`develop`)
- **Required status checks**: Basic CI checks
- **Required reviews**: At least 1 approval for external contributors
- **Allow force pushes**: Only for maintainers

## Code Quality Rules

### Required Checks
1. **Formatting**: `cargo fmt --check` must pass
2. **Linting**: `cargo clippy --all-targets --all-features -- -D warnings` must pass
3. **Testing**: All tests must pass (unit, integration, doc tests)
4. **Documentation**: Public APIs must have documentation
5. **Security**: `cargo audit` must pass
6. **Dependencies**: Dependabot security updates

### Code Standards
- Follow Rust standard formatting (configured in `rustfmt.toml`)
- Address all clippy warnings (configured in `clippy.toml`)
- Maintain test coverage above 90%
- Use conventional commit messages
- Include examples in documentation

## Security Rules

### Dependency Management
- **Automated updates**: Dependabot enabled for weekly updates
- **Security scanning**: Cargo audit in CI pipeline
- **Vulnerability alerts**: Enabled for all dependencies
- **Private vulnerability reporting**: Enabled

### Secret Management
- **No secrets in code**: Use environment variables or secret management
- **Credential scanning**: Automated detection of exposed credentials
- **API keys**: Store in repository secrets for CI/CD

## Pull Request Requirements

### Required Information
1. **Clear description**: What changes were made and why
2. **Breaking changes**: Clearly marked if any
3. **Testing**: Evidence that changes were tested
4. **Documentation**: Updated if public APIs changed
5. **Changelog**: Updated for user-facing changes

### Review Process
1. **Automated checks**: All CI checks must pass
2. **Manual review**: At least one maintainer approval
3. **Code owner review**: For core functionality changes
4. **Integration testing**: Verify examples still work

## Release Rules

### Version Management
- **Semantic versioning**: Follow semver strictly
- **Changelog**: Update CHANGELOG.md for all releases
- **Git tags**: All releases must be tagged
- **Release notes**: Generated automatically from changelog

### Release Process
1. **Update version**: In Cargo.toml and related files
2. **Update changelog**: Document all changes
3. **Create PR**: For release preparation
4. **Tag release**: After merge to main
5. **Publish**: Automated via CI/CD to crates.io

## Issue and Project Management

### Issue Templates
- **Bug reports**: Require reproduction steps and environment details
- **Feature requests**: Require use case and proposed implementation
- **Security issues**: Private reporting enabled

### Labels and Milestones
- **Priority levels**: P0 (critical), P1 (high), P2 (medium), P3 (low)
- **Component labels**: Core, Python bindings, Examples, Documentation
- **Status tracking**: In progress, under review, blocked

## Enforcement

These rules are enforced through:
1. **GitHub branch protection**: Prevents direct pushes to protected branches
2. **CI/CD pipeline**: Automated quality checks
3. **Required reviews**: Human oversight for all changes
4. **Automated tooling**: Dependabot, security scanning, formatting checks

## Exceptions

Exceptions to these rules may be granted by repository maintainers in the following cases:
- **Emergency security fixes**: May bypass some review requirements
- **Critical bug fixes**: May be fast-tracked with reduced review time
- **Documentation-only changes**: May have relaxed requirements

All exceptions must be documented and justified in the pull request.