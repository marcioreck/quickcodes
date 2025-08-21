# Contributing to QuickCodes

Thank you for your interest in contributing to QuickCodes! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** from `main`
4. **Make your changes** following our guidelines
5. **Test thoroughly** and ensure all tests pass
6. **Submit a pull request** with a clear description

## 🛠️ Development Setup

### Prerequisites
- Rust 1.70+ (latest stable recommended)
- Git
- (Optional) Python 3.8+ for Python bindings

### Setup
```bash
# Clone the repository
git clone https://github.com/marcioreck/quickcodes.git
cd quickcodes

# Run tests to ensure everything works
cargo test --all

# Check code formatting
cargo fmt --check

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Run examples
cargo run --example basic_usage
```

## 📝 Contribution Guidelines

### Code Style
- Follow standard Rust formatting (`cargo fmt`)
- Pass all Clippy lints (`cargo clippy`)
- Write clear, self-documenting code
- Add documentation for public APIs
- Include examples in documentation when helpful

### Testing
- **All new code must include tests**
- Maintain 100% test coverage for public APIs
- Add integration tests for new features
- Include edge cases and error conditions
- Performance tests for critical paths

### Commit Messages
Follow [Conventional Commits](https://www.conventionalcommits.org/):
```
feat: add DataMatrix barcode support
fix: correct EAN-13 checksum calculation
docs: update Python usage examples
test: add concurrent generation tests
```

### Pull Request Process
1. **Update documentation** if needed
2. **Add tests** for new functionality
3. **Update CHANGELOG.md** with your changes
4. **Ensure CI passes** on all platforms
5. **Request review** from maintainers

## 🎯 Areas for Contribution

### Phase 2 Features (Help Wanted!)
- **Barcode Reading/Decoding**: Implement readers for existing formats
- **New Formats**: DataMatrix, PDF417, Aztec codes
- **JavaScript Bindings**: WebAssembly support
- **CLI Tool**: Command-line interface
- **Performance**: SIMD optimizations
- **Mobile**: React Native/Flutter bindings

### Always Welcome
- **Bug fixes** and error handling improvements
- **Documentation** improvements and examples
- **Test coverage** expansion
- **Performance** optimizations
- **Platform support** (new OS/architectures)
- **Examples** for specific use cases

## 🐛 Reporting Issues

### Bug Reports
Please include:
- **Clear description** of the issue
- **Steps to reproduce** the problem
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version)
- **Code sample** if applicable

### Feature Requests
Please include:
- **Clear use case** and motivation
- **Proposed API** or interface
- **Alternatives considered**
- **Implementation ideas** (if any)

## 📚 Development Resources

### Project Structure
```
src/
├── lib.rs              # Main library interface
├── types/              # Core types and enums
├── generators/         # Barcode generators
├── exporters/          # Output format handlers
├── readers/            # Barcode readers (Phase 2)
└── python.rs          # Python bindings

tests/                  # Integration tests
examples/              # Usage examples
python/                # Python package
.github/               # CI/CD workflows
```

### Key Concepts
- **Modular Architecture**: Each barcode type is a separate module
- **Unified API**: Consistent interface across all formats
- **Error Handling**: Comprehensive error types with context
- **Performance**: Zero-copy where possible, efficient algorithms
- **Cross-Platform**: Pure Rust with minimal dependencies

## 🏆 Recognition

Contributors will be:
- **Listed in AUTHORS.md** 
- **Mentioned in release notes**
- **Given credit** in documentation
- **Invited to collaborate** on future features

## 📞 Getting Help

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Email**: marcio@fazmercado.com for private matters

## 📜 Code of Conduct

This project adheres to a **Code of Conduct** that we expect all participants to follow:

- **Be respectful** and inclusive
- **Assume good intentions** from others
- **Provide constructive feedback**
- **Focus on the code**, not the person
- **Help create a welcoming environment**

## 🎉 Thank You!

Every contribution, no matter how small, helps make QuickCodes better for everyone. We appreciate your time and effort!

---

**Happy Coding! 🚀**

*QuickCodes - Desenvolvido por Márcio Reck*
