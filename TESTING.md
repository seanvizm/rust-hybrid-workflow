# Workflow Testing System

This directory contains testing scripts for validating workflow files in the Rust Hybrid Workflow Engine.

## Test Scripts

### 1. `test_workflows.sh` - Comprehensive Test Suite

The main testing script that provides comprehensive validation and testing of workflow files.

**Features:**
- ✅ Builds the project before testing
- ✅ Validates Lua syntax (if Lua interpreter available)
- ✅ Checks workflow structure requirements
- ✅ Tests existing workflow files
- ✅ Creates and tests additional test scenarios
- ✅ Supports both success and failure testing
- ✅ Provides detailed colored output
- ✅ Cleans up temporary test files
- ✅ Timing information for each test

**Usage:**
```bash
# Run all tests
./test_workflows.sh

# Test specific workflow file
./test_workflows.sh workflows/my_workflow.lua
```

**Test Scenarios Included:**
- Existing workflow files validation
- Simple single-step Lua workflow
- Invalid workflow structure (expects failure)
- Python workflow with dependencies
- Mixed Lua-Python workflow

### 2. `quick_test.sh` - Simple Quick Tester

A simpler, faster script for quick workflow validation during development.

**Features:**
- ✅ Quick build and test
- ✅ Minimal output for fast feedback
- ✅ Tests specific file or all workflow files
- ✅ Simple pass/fail reporting

**Usage:**
```bash
# Test all workflow files
./quick_test.sh

# Test specific workflow file
./quick_test.sh workflows/hybrid_workflow.lua
```

## Test Output

### Success Output
```
[SUCCESS] ✓ Hybrid Python Workflow passed (2s)
```

### Error Output
```
[ERROR] ✗ Invalid Workflow failed unexpectedly (1s)
```

### Summary
```
Total Tests: 6
Passed: 6
Failed: 0
[SUCCESS] All tests passed! 🎉
```

## Workflow File Requirements

For a workflow file to pass validation, it must:

1. **Have valid Lua syntax** (if Lua interpreter is available)
2. **Contain required structure:**
   ```lua
   workflow = {
     name = "workflow_name",
     description = "description",
     steps = {
       -- step definitions
     }
   }
   ```
3. **Have valid step definitions** with either:
   - Lua function: `run = function() ... end`
   - Python code: `language = "python", code = "..."`

## Test Scenarios

### 1. Pure Lua Workflow
```lua
workflow = {
  steps = {
    step1 = {
      run = function()
        return { result = "success" }
      end
    }
  }
}
```

### 2. Python Workflow
```lua
workflow = {
  steps = {
    step1 = {
      language = "python",
      code = [[
def run():
    return {"result": "success"}
]]
    }
  }
}
```

### 3. Mixed Workflow
```lua
workflow = {
  steps = {
    lua_step = {
      run = function()
        return { from_lua = "data" }
      end
    },
    python_step = {
      depends_on = {"lua_step"},
      language = "python",
      code = [[
def run(inputs):
    return {"processed": inputs["lua_step"]["from_lua"]}
]]
    }
  }
}
```

## Continuous Integration

These scripts can be integrated into CI/CD pipelines:

```bash
# In your CI script
./test_workflows.sh
exit_code=$?

if [ $exit_code -eq 0 ]; then
    echo "All workflow tests passed!"
else
    echo "Workflow tests failed!"
    exit 1
fi
```

## Troubleshooting

### Common Issues

1. **Build Failures**: Ensure all dependencies are installed and the project compiles
2. **Lua Syntax Warnings**: Install Lua interpreter for syntax validation
3. **Permission Denied**: Make sure scripts are executable (`chmod +x *.sh`)

### Debug Mode

For more verbose output, you can modify the scripts to remove `--quiet` flags:

```bash
# In the script, change:
cargo run --quiet
# To:
cargo run
```

## Adding New Tests

To add new test scenarios, modify `test_workflows.sh` and add new `create_test_workflow` calls:

```bash
create_test_workflow "workflows/test_my_scenario.lua" '
workflow = {
  name = "my_test",
  steps = {
    # your test steps
  }
}'
test_workflow "workflows/test_my_scenario.lua" 0 "My Test Scenario"
```