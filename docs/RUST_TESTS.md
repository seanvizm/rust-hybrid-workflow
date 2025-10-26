# Rust Unit Test Suite for Hybrid Workflow Engine

## Overview
This document describes the comprehensive Rust unit test suite that can be run with `cargo test`.

## Test Coverage Summary

### Total Tests: **32 tests**
- ✅ **32 passed**
- ❌ **0 failed**

## Test Categories

### 1. Main Integration Tests (8 tests)
Located in `src/main.rs` under the `tests` module:

- `test_hybrid_workflow_execution` - Tests the existing hybrid workflow file
- `test_pure_lua_workflow_execution` - Tests the existing pure Lua workflow file  
- `test_nonexistent_workflow_file` - Tests proper error handling for missing files
- `test_create_and_run_simple_lua_workflow` - Creates and tests a simple Lua workflow
- `test_create_and_run_python_workflow` - Creates and tests a simple Python workflow
- `test_workflow_with_dependencies` - Tests multi-step workflows with dependencies
- `test_invalid_workflow_structure` - Tests error handling for invalid workflow structure
- `test_mixed_lua_python_workflow` - Tests workflows mixing Lua and Python steps

### 2. Engine Module Tests (5 tests)
Located in `src/engine.rs` under the `tests` module:

- `test_sort_steps_no_dependencies` - Tests step sorting without dependencies
- `test_sort_steps_with_dependencies` - Tests dependency-based step sorting
- `test_sort_steps_circular_dependency` - Tests circular dependency detection
- `test_sort_steps_complex_dependencies` - Tests complex multi-level dependencies
- `test_run_workflow_integration` - Integration test for workflow execution

### 3. Lua Loader Tests (5 tests)
Located in `src/lua_loader.rs` under the `tests` module:

- `test_load_valid_lua_workflow` - Tests loading valid Lua workflow files
- `test_load_python_workflow` - Tests loading Python workflow files
- `test_load_workflow_with_dependencies` - Tests loading workflows with step dependencies
- `test_load_nonexistent_file` - Tests error handling for missing files
- `test_load_invalid_lua_syntax` - Tests error handling for invalid Lua syntax

### 4. Python Runner Tests (7 tests)
Located in `src/python_runner.rs` under the `tests` module:

- `test_run_python_step_no_inputs` - Tests Python steps without input parameters
- `test_run_python_step_with_inputs` - Tests Python steps with input data
- `test_run_python_step_syntax_error` - Tests error handling for Python syntax errors
- `test_run_python_step_no_run_function` - Tests error handling when `run` function is missing
- `test_run_python_step_runtime_error` - Tests error handling for Python runtime errors
- `test_run_python_step_complex_data_types` - Tests complex JSON data type handling
- `test_run_python_step_with_complex_inputs` - Tests complex input data processing

### 5. Lua Runner Tests (7 tests)
Located in `src/lua_runner.rs` under the `tests` module:

- `test_run_lua_step_no_inputs` - Tests Lua steps without input parameters
- `test_run_lua_step_with_inputs` - Tests Lua steps with input data
- `test_run_lua_step_nonexistent_step` - Tests error handling for missing Lua steps
- `test_json_to_lua_conversion` - Tests JSON to Lua value conversion
- `test_lua_to_json_conversion` - Tests Lua value to JSON conversion
- `test_lua_array_conversion` - Tests Lua array to JSON array conversion
- `test_lua_object_conversion` - Tests Lua table to JSON object conversion

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Specific Test
```bash
cargo test test_hybrid_workflow_execution
```

### Run Tests for Specific Module
```bash
cargo test engine::tests
cargo test python_runner::tests
cargo test lua_runner::tests
cargo test lua_loader::tests
```

## Test Features

### ✅ **Automatic Cleanup**
- Tests create temporary workflow files
- Automatically clean up test files after execution
- No manual cleanup required

### ✅ **Error Testing**
- Tests both success and failure scenarios
- Validates proper error handling
- Tests edge cases and invalid inputs

### ✅ **Integration Testing**
- Tests end-to-end workflow execution
- Tests interaction between modules
- Validates data flow between steps

### ✅ **Unit Testing** 
- Tests individual functions in isolation
- Tests data conversion utilities
- Tests dependency resolution logic

### ✅ **Data Type Testing**
- Tests JSON ↔ Python object conversion
- Tests JSON ↔ Lua value conversion
- Tests complex nested data structures

## Sample Test Output

```
running 32 tests
test engine::tests::test_sort_steps_with_dependencies ... ok
test lua_runner::tests::test_run_lua_step_no_inputs ... ok
test python_runner::tests::test_run_python_step_with_inputs ... ok
test tests::test_hybrid_workflow_execution ... ok
...
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Continuous Integration

These tests are perfect for CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Tests
  run: cargo test
  
- name: Run Tests with Coverage
  run: cargo test -- --nocapture
```

## Adding New Tests

To add new tests, simply add functions with the `#[test]` attribute:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_new_feature() {
        // Test implementation
        assert!(true);
    }
}
```

The test suite provides comprehensive coverage of all major functionality and edge cases, ensuring the reliability and correctness of the Hybrid Workflow Engine.