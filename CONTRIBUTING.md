# Contributing to VR Game Platform

Thank you for your interest in contributing to VR Game Platform! We welcome contributions from the community.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- Clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Your system specs (OS, VR headset, etc.)
- Screenshots or videos if applicable

### Suggesting Features

We love feature suggestions! Please create an issue with:
- Clear description of the feature
- Use cases and benefits
- Any implementation ideas you have

### Code Contributions

1. **Fork the repository**
   ```bash
   git fork https://github.com/yourorg/vr-game-platform.git
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **Make your changes**
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

4. **Test your changes**
   ```bash
   # Build and test C++
   mkdir build && cd build
   cmake .. && make
   ./VREngineTests
   
   # Test Rust
   cargo test
   ```

5. **Commit with clear messages**
   ```bash
   git commit -m "Add: New VR hand gesture recognition"
   ```

6. **Push to your fork**
   ```bash
   git push origin feature/amazing-feature
   ```

7. **Create a Pull Request**
   - Describe what changes you made
   - Reference any related issues
   - Include screenshots/videos if relevant

## Code Style Guidelines

### C++
- Use C++17 features
- Follow Google C++ Style Guide
- Use meaningful variable names
- Comment complex logic
- Keep functions focused and small

### Rust
- Use `rustfmt` for formatting
- Follow Rust API guidelines
- Write documentation comments
- Use `clippy` for linting

### Commit Messages
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line should be under 50 characters
- Reference issues and pull requests

## Development Setup

### Prerequisites
- C++17 compiler
- Rust 1.70+
- CMake 3.15+
- VR headset (for testing)

### Building from Source
See [QUICKSTART.md](docs/QUICKSTART.md) for detailed instructions.

## Testing

- Write tests for all new features
- Ensure all tests pass before submitting PR
- Test on multiple VR platforms if possible
- Test both standalone and multiplayer modes

## Documentation

- Update README.md if adding major features
- Add/update comments in code
- Update API documentation
- Include examples for new features

## Code Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged
4. Your contribution will be credited!

## Community

- Be respectful and inclusive
- Help others learn and grow
- Share your creations
- Have fun building VR experiences!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to ask questions in:
- GitHub Discussions
- Discord server
- Email: dev@vrgameplatform.com

Thank you for contributing! 🎮✨
