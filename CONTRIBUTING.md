# Contributing

We welcome contributions! Whether you're fixing bugs, improving documentation, adding new instrumentation examples, or enhancing existing services, your help is appreciated.

## Getting Started

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Ideas for Contributions

- **New language examples** - Add instrumentation examples in Go, C#, PHP, Ruby, etc.
- **Enhanced telemetry** - Add metrics, logs, or advanced tracing patterns
- **Documentation improvements** - Better guides, troubleshooting tips, or API docs
- **Testing and CI** - Improve test coverage or build processes
- **Performance optimizations** - Database queries, caching, or instrumentation overhead
- **Bug fixes** - Fix issues with existing services or configurations

## Code Standards

- Follow existing code style and naming conventions
- Use proper span naming: `{verb} {object}` pattern (e.g., `fetch song`, `validate input`)
- Add meaningful span attributes and follow semantic conventions
- Include error handling and proper status codes

## Questions?

Feel free to open an issue first to discuss larger changes or ask questions. We're here to help!
