# Zenocode - AI Coding Assistant

A Rust-based coding assistant with CLI and TUI interfaces, supporting multiple LLM providers.

## Project Structure

zenocode/ 
├── Cargo.toml             # Workspace configuration 
│
├── crates/ 
│   ├── core/              # Shared logic and abstractions 
│   ├── providers-core/    # Shared provider abstractions 
│   └── providers/         # LLM provider implementations 
│       └── openai/        # OpenAI provider 
│
├── app/ 
│   ├── zenocode-cli/      # CLI binary entry point 
│   └── zenocode-tui/      # TUI binary entry point 
│
├── AGENTS.md              # Developer guidelines 
└── README.md              # This file


## Current Status
- **Core**: Basic LLM provider trait and config system implemented
- **Providers**: OpenAI integration partially complete
- **Interfaces**: CLI and TUI entry points scaffolded
- **Testing**: Basic unit tests in place

## Next Steps
1. **Complete OpenAI provider** with streaming support
2. **Implement config file loading** (YAML/TOML)
3. **Add more LLM providers**:
   - Anthropic
   - Ollama
4. **Improve interfaces**:
   - Add conversation history to TUI
   - Implement file editing in CLI
5. **Set up CI/CD** with GitHub Actions
6. **Expand test coverage** for core functionality
