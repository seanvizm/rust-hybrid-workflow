# 🚀 Rust Hybrid Workflow Engine

A high-performance, multi-language workflow orchestration engine built in Rust that seamlessly executes Python, JavaScript/Node.js, WebAssembly, Lua, and Shell scripts in complex dependency graphs.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024+-blue.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.6+-green.svg)](https://www.python.org)
[![JavaScript](https://img.shields.io/badge/javascript-ES6+-yellow.svg)](https://nodejs.org)
[![WebAssembly](https://img.shields.io/badge/webassembly-1.0-purple.svg)](https://webassembly.org)
[![Lua](https://img.shields.io/badge/lua-5.4-blue.svg)](https://www.lua.org)

## 📋 Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Tech Stack](#tech-stack)
- [Rust Edition 2024 Ready](#rust-edition-2024-ready)
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

The Rust Hybrid Workflow Engine is a powerful orchestration tool that allows you to define complex multi-step workflows using Lua configuration files. Each step in your workflow can be executed in different programming languages (Python, JavaScript/Node.js, WebAssembly, Lua, Shell), with automatic dependency resolution and data passing between steps.

Perfect for:
- **Data Processing Pipelines**: Chain together data fetching, transformation, and storage operations
- **DevOps Automation**: Combine shell commands, Python scripts, JavaScript automation, and Lua logic for deployment workflows
- **Multi-Language Projects**: Leverage the best of each language in a single workflow
- **Complex Orchestration**: Handle intricate dependency graphs with automatic topological sorting

## ✨ Key Features

- 🔀 **Multi-Language Support**: Execute Python, JavaScript/Node.js, WebAssembly, Lua, and Shell scripts seamlessly
- 📊 **Dependency Management**: Automatic topological sorting of workflow steps
- 🔄 **Data Flow**: Pass results between steps across different languages
- 🚀 **High Performance**: Built in Rust for speed and memory safety
- 📝 **Declarative Configuration**: Define workflows using intuitive Lua syntax
- 🌐 **Web UI**: Interactive Leptos-based interface with real-time execution visualization and hot reload
- 🔌 **REST API**: HTTP endpoints for remote workflow execution and management
- 🧪 **Comprehensive Testing**: Built-in test suite with validation tools
- 🔧 **Error Handling**: Robust error reporting and recovery mechanisms
- 📦 **Easy Integration**: Command-line interface, library usage, and web interface

## 🛠 Tech Stack

### Core Technologies
- **[Rust](https://www.rust-lang.org/)** (Edition 2024) - Core engine and orchestration
- **[Lua 5.4](https://www.lua.org/)** - Workflow configuration and scripting
- **[Python 3.6+](https://www.python.org/)** - Data processing and external integrations
- **[Node.js](https://nodejs.org/)** - JavaScript runtime for modern web and backend logic
- **[WebAssembly](https://webassembly.org/)** - High-performance, secure code execution
- **[Leptos](https://leptos.dev/)** (0.6) - Reactive web framework for the UI frontend
- **[Axum](https://github.com/tokio-rs/axum)** (0.8) - Web server framework for REST API
- **Shell/Bash** - System operations and command execution

### Dependencies
- **[mlua](https://crates.io/crates/mlua)** (0.9) - Lua integration with Rust
- **[pyo3](https://crates.io/crates/pyo3)** (0.23) - Python integration with auto-initialization
- **[wasmtime](https://crates.io/crates/wasmtime)** (26.0) - WebAssembly runtime for Rust
- **[leptos](https://crates.io/crates/leptos)** (0.6) - WebAssembly frontend framework
- **[axum](https://crates.io/crates/axum)** (0.8) - HTTP server and REST API
- **[trunk](https://trunkrs.dev/)** - WASM build tool and dev server with hot reload
- **[anyhow](https://crates.io/crates/anyhow)** (1.0) - Error handling and context
- **[serde_json](https://crates.io/crates/serde_json)** (1.0) - JSON serialization for data exchange
- **[tempfile](https://crates.io/crates/tempfile)** (3.0) - Temporary file management
- **[chrono](https://crates.io/crates/chrono)** (0.4) - Date and time handling

## 🎯 Rust Edition 2024 Ready

This project is built with **Rust Edition 2024**, leveraging the latest language features and improvements for enhanced performance and developer experience.

### 🚀 Edition 2024 Benefits

**Performance Enhancements:**
- ⚡ **Advanced Compiler Optimizations** - Improved code generation and smaller binary sizes
- 🔋 **Enhanced Async Runtime** - Better async/await performance and reduced overhead
- 🎯 **Smart Memory Management** - More efficient memory allocation patterns

**Developer Experience:**
- 🛠️ **Improved Error Messages** - Clearer diagnostics with actionable suggestions
- 🔍 **Enhanced Pattern Matching** - More expressive `match` arms and `let-else` patterns
- 📝 **Better Type Inference** - Reduced need for explicit type annotations

**Language Features:**
- 🔄 **Advanced Macro System** - More powerful procedural macros for code generation
- 🧩 **Refined Lifetime Management** - Simplified lifetime annotations and better ergonomics
- 🌟 **Future-Ready Syntax** - Latest language idioms and best practices

**Why This Matters for Workflow Engine:**
- **Reliability**: Latest safety improvements reduce runtime errors
- **Performance**: Optimized execution for multi-language orchestration
- **Maintainability**: Modern syntax makes the codebase easier to extend
- **Compatibility**: Forward-compatible with future Rust developments

> 💡 **Pro Tip**: All dependencies are verified compatible with Edition 2024, ensuring a stable and modern development experience.

## 🚀 Quick Start

### Prerequisites
- Rust (Edition 2024 or later) with `wasm32-unknown-unknown` target
- Python 3.6+
- Node.js 14+ (for JavaScript steps)
- Lua 5.4 (optional, for validation)
- WebAssembly modules (*.wasm files) for WASM steps

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
# Run a specific workflow file (automatically looks in workflows/ folder)
cargo run workflow_name.lua
```

### Web UI

For an interactive web interface with real-time execution visualization:

```bash
# Production mode
./run_web_ui.sh

# Development mode (with hot reload)
./run_web_ui_dev.sh
```

Then open your browser:
- **Production**: http://localhost:3000
- **Development**: http://localhost:8080

Features:
- 📋 Browse all workflows in a visual grid
- ▶️ One-click workflow execution
- 📊 Real-time step-by-step results
- 🔄 Hot reload during development
- 📱 Responsive mobile-friendly design

See [docs/WEB_UI.md](docs/WEB_UI.md) for complete documentation.

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

    analyze_data = {
      depends_on = { "process_data" },
      language = "javascript",
      code = [[
function run(inputs) {
    const processed = inputs.process_data.processed;
    // Your JavaScript analysis logic
    const analytics = {
        count: processed.length,
        sum: processed.reduce((a, b) => a + b, 0),
        average: processed.reduce((a, b) => a + b, 0) / processed.length
    };
    return { analytics: analytics };
}
]]
    },

    compute_heavy_task = {
      depends_on = { "analyze_data" },
      language = "wasm",
      module = "workflows/compute_module.wasm",
      func = "process_data",
      -- High-performance computation using WebAssembly
    },

    save_results = {
      depends_on = { "compute_heavy_task" },
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

# Run specific workflow (automatically looks in workflows/ folder)
cargo run your_workflow.lua

# Using the compiled binary directly
./target/release/hybrid-workflow-engine your_workflow.lua

# Note: The engine automatically searches in the workflows/ directory
# So you don't need to specify "workflows/" in the command
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

### 4. JavaScript Workflow (`javascript_workflow.lua`)
Pure JavaScript/Node.js data processing pipeline:
```lua
-- Data generation → JavaScript analytics → Result formatting
-- Demonstrates modern JavaScript features and JSON processing
```

### 5. Multi-Language Workflow (`multi_language_workflow.lua`)
Ultimate demonstration using all supported languages:
```lua
-- Python → JavaScript → Lua → Shell → Python
-- Complete multi-language integration with complex data flow
```

### 6. WebAssembly Workflow (`wasm_workflow.lua`)
High-performance computing with WebAssembly integration:
```lua
-- Python → WASM → WASM → WASM → JavaScript → Python
-- Demonstrates secure, high-performance computation with WASM modules
```

### 7. Comprehensive Workflow (`comprehensive_workflow.lua`)
Complex multi-language pipeline demonstrating core features:
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
│ • Extract steps │    │   Resolution    │    │ • JS/Node.js    │
│ • Validate      │    │ • Execution     │    │   Runner        │
│                 │    │   Order         │    │ • WASM Runner   │
│                 │    │                 │    │ • Lua Runner    │
│                 │    │                 │    │ • Shell Runner  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

- **Engine (`src/core/engine.rs`)**: Main orchestration logic and workflow execution
- **Lua Loader (`src/core/lua_loader.rs`)**: Parses and validates Lua workflow files
- **Runners (`src/runners/`)**: Language-specific execution engines
  - `python_runner.rs` - Python script execution with PyO3
  - `javascript_runner.rs` - JavaScript/Node.js execution with process spawning
  - `wasm_runner.rs` - WebAssembly module execution with Wasmtime
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

### Phase 1: Core Components (v0.1.0)
- [x] **Main Hybrid Workflow Engine** - ✅ Main orchestration logic and workflow execution
- [x] **Lua Workflow Script Loader** - ✅ Parses and validates Lua workflow files
- [x] **Python Runner** - ✅ Python script execution with PyO3 integration
- [x] **Shell Script Runner** - ✅ Shell command execution support
- [x] **JavaScript/Node.js Runner** - ✅ JavaScript execution support with Node.js integration
- [x] **WebAssembly Support** - ✅ Execute WASM modules with Wasmtime runtime integration

### Phase 2: Advanced Features (v0.2.0)
- [ ] **Configuration Management** - External config files and environment variables
- [ ] **Improved Error Reporting** - Better error messages with line numbers and context
- [ ] **Parallel Execution** - Execute independent steps concurrently (currently sequential)
- [x] **REST API Interface** - ✅ HTTP API for remote workflow execution with Axum backend
- [x] **Web UI** - ✅ Interactive Leptos-based web interface for workflow management
  - ✅ Workflow list view with one-click execution
  - ✅ Real-time step-by-step execution results
  - ✅ Hot reload support for development
  - ✅ Responsive design with mystical blue theme
  - ✅ 404 error handling
- [ ] **Workflow Visualization** - Generate dependency graphs (execution flow visualization implemented)

### Phase 3: Enterprise Features (v0.3.0)
- [ ] **Database Integration** - Built-in connectors for common databases
- [ ] **Workflow Scheduling** - Cron-like scheduling and triggers
- [ ] **Monitoring & Metrics** - Performance monitoring and observability
- [ ] **Plugin System** - Custom language runners and extensions
- [ ] **Distributed Execution** - Multi-node workflow execution

### Phase 3.5: AI & Machine Learning Integration (v0.3.5)
- [ ] **AI-Powered Workflow Optimization** - Automatic workflow step reordering and parallelization suggestions
- [ ] **Smart Error Recovery** - AI-driven error pattern recognition and automated retry strategies
- [ ] **Intelligent Resource Management** - ML-based resource allocation and scaling predictions
- [ ] **Natural Language Workflow Generation** - Generate workflows from plain English descriptions
- [ ] **Anomaly Detection** - AI monitoring for unusual execution patterns and performance degradation
- [ ] **Code Completion & Suggestions** - AI-powered code completion for workflow scripts across languages
- [ ] **Automated Testing Generation** - AI-generated test cases based on workflow structure and data flow
- [ ] **Performance Prediction** - ML models to predict execution time and resource requirements
- [ ] **Smart Data Transformation** - AI-suggested data mapping and transformation between workflow steps
- [ ] **Workflow Pattern Recognition** - Learn from existing workflows to suggest optimizations and best practices

### Phase 4: Cloud & Integration (v0.4.0)
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

Made with ❤️ by [Sean VizM](https://github.com/seanvizm)

</div>