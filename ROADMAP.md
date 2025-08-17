# Zenocode Development Roadmap

## Milestone 1: Core Chat Interface

### Basic TUI Implementation
- [ ] Create message input/output components
- [ ] Implement conversation history management
- [ ] Add basic message formatting (text)

### OpenAI Provider Integration
- [ ] Implement API client for OpenAI
- [ ] Connect provider to TUI interface
- [ ] Add configuration for API keys

### Core Functionality
- [ ] Implement message queuing system
- [ ] Add error handling for API failures
- [ ] Create session persistence

---

## Milestone 2: Enhanced Interaction

### Response Formatting
- [ ] Add syntax highlighting for code blocks
- [ ] Implement markdown rendering
- [ ] Support table/structured data display

### Input Modes
- [ ] Implement "ask" mode (Q&A)
- [ ] Create "code" mode (context-aware editing)
- [ ] Add file attachment support

### Multi-provider Support
- [ ] Add Anthropic provider
- [ ] Add Ollama/local LLM provider
- [ ] Implement provider switching UI

---

## Milestone 3: Advanced Features

### LSP Integration
- [ ] Connect to language servers
- [ ] Implement code intelligence features
- [ ] Add diagnostics/error display

### MCP Integration
- [ ] Implement MCP protocol handlers
- [ ] Create plugin system architecture
- [ ] Add extension management UI

### Workflow Automation
- [ ] Add command templating
- [ ] Implement context-aware shortcuts
- [ ] Create task automation system

---

## Milestone 4: Optimization & Scaling

### Performance Enhancements
- [ ] Implement response streaming
- [ ] Add caching mechanisms
- [ ] Optimize memory usage

### Project Context Awareness
- [ ] Add workspace/file context loading
- [ ] Implement codebase indexing
- [ ] Create cross-file reference support

### Collaboration Features
- [ ] Add multi-user sessions
- [ ] Implement shared context pools
- [ ] Create collaboration history

---

## Future Considerations
- Custom plugin marketplace
- Voice input/output support
- CI/CD integration
- Self-hosted deployment options
- Fine-tuning interface