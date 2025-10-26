# 🚀 Rust Hybrid Workflow Engine

A high-performance, multi-language workflow orchestration engine built in Rust that seamlessly executes Python, Lua, and Shell scripts in complex dependency graphs.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024+-blue.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.6+-green.svg)](https://www.python.org)
[![Lua](https://img.shields.io/badge/lua-5.4-blue.svg)](https://www.lua.org)

## 📋 Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Tech Stack](#tech-stack)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [Workflow Examples](#workflow-examples)
- [Architecture](#architecture)
- [Testing](#testing)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## 🌟 Overview

The Rust Hybrid Workflow Engine is a powerful orchestration tool that allows you to define complex multi-step workflows using Lua configuration files. Each step in your workflow can be executed in different programming languages (Python, Lua, Shell), with automatic dependency resolution and data passing between steps.

Perfect for:
- **Data Processing Pipelines**: Chain together data fetching, transformation, and storage operations
- **DevOps Automation**: Combine shell commands, Python scripts, and Lua logic for deployment workflows
- **Multi-Language Projects**: Leverage the best of each language in a single workflow
- **Complex Orchestration**: Handle intricate dependency graphs with automatic topological sorting

## ✨ Key Features

- 🔀 **Multi-Language Support**: Execute Python, Lua, and Shell scripts seamlessly
- 📊 **Dependency Management**: Automatic topological sorting of workflow steps
- 🔄 **Data Flow**: Pass results between steps across different languages
- 🚀 **High Performance**: Built in Rust for speed and memory safety
- 📝 **Declarative Configuration**: Define workflows using intuitive Lua syntax
- 🧪 **Comprehensive Testing**: Built-in test suite with validation tools
- 🔧 **Error Handling**: Robust error reporting and recovery mechanisms
- 📦 **Easy Integration**: Simple command-line interface and library usage

## 🛠 Tech Stack

### Core Technologies
- **[Rust](https://www.rust-lang.org/)** (Edition 2024) - Core engine and orchestration
- **[Lua 5.4](https://www.lua.org/)** - Workflow configuration and scripting
- **[Python 3.6+](https://www.python.org/)** - Data processing and external integrations
- **Shell/Bash** - System operations and command execution

### Dependencies
- **[mlua](https://crates.io/crates/mlua)** (0.9) - Lua integration with Rust
- **[pyo3](https://crates.io/crates/pyo3)** (0.23) - Python integration with auto-initialization
- **[anyhow](https://crates.io/crates/anyhow)** (1.0) - Error handling and context
- **[serde_json](https://crates.io/crates/serde_json)** (1.0) - JSON serialization for data exchange
- **[tempfile](https://crates.io/crates/tempfile)** (3.0) - Temporary file management

## 🚀 Quick Start

### Prerequisites
- Rust (Edition 2024 or later)
- Python 3.6+
- Lua 5.4 (optional, for validation)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/seanvizm/rust-hybrid-workflow.git
   cd rust-hybrid-workflow
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run demo workflows:**
   ```bash
   cargo run
   ```

### Running Specific Workflows

```bash
# Run a specific workflow file
cargo run hybrid_workflow.lua
```

## 💡 Usage

### Basic Workflow Structure

Create a Lua file defining your workflow:

```lua
-- my_workflow.lua
workflow = {
  name = "data_processing_pipeline",
  description = "Process and analyze data using multiple languages",

  steps = {
    fetch_data = {
      language = "python",
      code = [[
def run():
    # Your Python code here
    return {"data": [1, 2, 3, 4, 5]}
]]
    },

    process_data = {
      depends_on = { "fetch_data" },
      language = "lua",
      code = [[
function run(inputs)
    local data = inputs.fetch_data.data
    -- Your Lua processing logic
    return {processed = data}
end
]]
    },

    save_results = {
      depends_on = { "process_data" },
      language = "shell",
      code = [[
#!/bin/bash
echo "Saving results: $1"
# Your shell commands here
echo '{"status": "saved"}'
]]
    }
  }
}
```

### Command Line Options

```bash
# Run all demo workflows
cargo run

# Run specific workflow from "workflows" folder
cargo run your_workflow.lua

# Run with absolute path
cargo run /path/to/your/workflow.lua
```

## 📚 Workflow Examples

The project includes several example workflows:

### 1. Hybrid Workflow (`hybrid_workflow.lua`)
Demonstrates Python data processing with dependency chains:
```lua
-- Fetches data → Processes data → Stores results
-- All in Python with automatic dependency resolution
```

### 2. Pure Lua Workflow (`workflow.lua`)
Shows native Lua scripting capabilities:
```lua
-- Mathematical calculations and data transformations
-- Using pure Lua functionality
```

### 3. Shell Integration (`shell_workflow.lua`)
Combines shell commands with Python processing:
```lua
-- System operations → Python analysis → Shell reporting
```

### 4. Comprehensive Workflow (`comprehensive_workflow.lua`)
Complex multi-language pipeline demonstrating all features:
```lua
-- Python → Lua → Shell → Python
-- Full dependency graph with cross-language data flow
```

## 🏗 Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Lua Loader    │───▶│ Workflow Engine │───▶│  Step Runners   │
│                 │    │                 │    │                 │
│ • Parse .lua    │    │ • Dependency    │    │ • Python Runner │
│ • Extract steps │    │   Resolution    │    │ • Lua Runner    │
│ • Validate      │    │ • Execution     │    │ • Shell Runner  │
│                 │    │   Order         │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

- **Engine (`src/core/engine.rs`)**: Main orchestration logic and workflow execution
- **Lua Loader (`src/core/lua_loader.rs`)**: Parses and validates Lua workflow files
- **Runners (`src/runners/`)**: Language-specific execution engines
  - `python_runner.rs` - Python script execution with PyO3
  - `lua_runner.rs` - Lua script execution with MLua
  - `shell_runner.rs` - Shell command execution

## 🧪 Testing

The project includes comprehensive testing tools:

### Run Tests
```bash
# Unit tests
cargo test

# Integration tests with all workflows
./test_workflows.sh

# Quick validation test
./quick_test.sh
```

### Test Features
- ✅ Syntax validation for Lua workflows
- ✅ Dependency resolution testing
- ✅ Multi-language execution validation
- ✅ Error handling verification
- ✅ Performance benchmarking

See [`docs/TESTING.md`](docs/TESTING.md) for detailed testing documentation.

## 🗺 Roadmap

### Phase 1: Core Enhancements (v0.2.0)
- [ ] **JavaScript/Node.js Runner** - Add JavaScript execution support
- [ ] **WebAssembly Support** - Execute WASM modules as workflow steps
- [ ] **Configuration Management** - External config files and environment variables
- [ ] **Improved Error Reporting** - Better error messages with line numbers and context

### Phase 2: Advanced Features (v0.3.0)
- [ ] **Parallel Execution** - Execute independent steps concurrently
- [ ] **Workflow Visualization** - Generate dependency graphs and execution flows
- [ ] **REST API Interface** - HTTP API for remote workflow execution
- [ ] **Database Integration** - Built-in connectors for common databases

### Phase 3: Enterprise Features (v0.4.0)
- [ ] **Workflow Scheduling** - Cron-like scheduling and triggers
- [ ] **Monitoring & Metrics** - Performance monitoring and observability
- [ ] **Plugin System** - Custom language runners and extensions
- [ ] **Distributed Execution** - Multi-node workflow execution

### Phase 4: Cloud & Integration (v0.5.0)
- [ ] **Cloud Connectors** - AWS, GCP, Azure integrations
- [ ] **Container Support** - Docker and Kubernetes integration
- [ ] **Message Queue Integration** - RabbitMQ, Kafka, Redis support
- [ ] **GraphQL Interface** - Modern API with real-time subscriptions

## 🔮 Future Improvements

### Performance Optimizations
- **Memory Pool Management** - Reduce allocation overhead
- **Lazy Loading** - Load workflow steps on demand
- **Compilation Caching** - Cache compiled Python/Lua code
- **Streaming Data Processing** - Handle large datasets efficiently

### Developer Experience
- **VSCode Extension** - Syntax highlighting and debugging for workflow files
- **Interactive Debugger** - Step-by-step workflow debugging
- **Hot Reloading** - Automatic workflow reloading during development
- **Template System** - Pre-built workflow templates

### Integration & Ecosystem
- **CI/CD Integration** - GitHub Actions, GitLab CI, Jenkins plugins
- **Monitoring Integrations** - Prometheus, Grafana, DataDog support
- **Authentication & Authorization** - RBAC and security features
- **Workflow Marketplace** - Community-driven workflow sharing

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test && ./test_workflows.sh`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Contribution Areas
- 🐛 **Bug Fixes** - Help us squash bugs and improve stability
- 🚀 **New Features** - Implement items from our roadmap
- 📚 **Documentation** - Improve docs, examples, and tutorials
- 🧪 **Testing** - Add test cases and improve coverage
- 🎨 **Examples** - Create new workflow examples and use cases

### Code Style
- Follow Rust conventions and use `cargo fmt`
- Add comprehensive tests for new features
- Update documentation for API changes
- Use descriptive commit messages

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community** - For the amazing ecosystem and tools
- **MLua & PyO3 Teams** - For excellent language binding libraries
- **Contributors** - Thank you to all who have contributed to this project

---

<div align="center">

**[⭐ Star this project](https://github.com/seanvizm/rust-hybrid-workflow)** if you find it useful!

Made with ❤️ by [Sean Vizm](https://github.com/seanvizm)

</div>