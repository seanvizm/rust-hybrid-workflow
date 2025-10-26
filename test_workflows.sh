#!/bin/bash

# Workflow File Tester for Rust Hybrid Workflow Engine
# This script tests various workflow files and validates their execution

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "INFO")
            echo -e "${BLUE}[INFO]${NC} $message"
            ;;
        "SUCCESS")
            echo -e "${GREEN}[SUCCESS]${NC} $message"
            ;;
        "ERROR")
            echo -e "${RED}[ERROR]${NC} $message"
            ;;
        "WARNING")
            echo -e "${YELLOW}[WARNING]${NC} $message"
            ;;
    esac
}

# Function to test a single workflow file
test_workflow() {
    local workflow_file=$1
    local expected_result=${2:-0}  # Default to expecting success (exit code 0)
    local test_name=${3:-$(basename "$workflow_file")}
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    print_status "INFO" "Testing workflow: $test_name"
    echo "----------------------------------------"
    
    # Check if file exists
    if [ ! -f "$workflow_file" ]; then
        print_status "ERROR" "Workflow file not found: $workflow_file"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo ""
        return 1
    fi
    
    # Create a temporary main.rs for this specific test
    local temp_main="src/main_test_temp.rs"
    cat > "$temp_main" << EOF
mod engine;
mod lua_loader;
mod lua_runner;
mod python_runner;

fn main() -> anyhow::Result<()> {
    engine::run_workflow("$workflow_file")?;
    Ok(())
}
EOF
    
    # Backup original main.rs
    cp src/main.rs src/main_backup.rs
    cp "$temp_main" src/main.rs
    
    # Run the test
    local start_time=$(date +%s)
    if cargo run --quiet 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $expected_result -eq 0 ]; then
            print_status "SUCCESS" "✓ $test_name passed (${duration}s)"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            print_status "ERROR" "✗ $test_name was expected to fail but passed"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $expected_result -ne 0 ]; then
            print_status "SUCCESS" "✓ $test_name failed as expected (${duration}s)"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            print_status "ERROR" "✗ $test_name failed unexpectedly (${duration}s)"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    fi
    
    # Restore original main.rs
    mv src/main_backup.rs src/main.rs
    rm -f "$temp_main"
    
    echo ""
}

# Function to validate workflow file syntax
validate_workflow_syntax() {
    local workflow_file=$1
    local test_name="Syntax validation for $(basename "$workflow_file")"
    
    print_status "INFO" "Validating syntax: $workflow_file"
    
    # Check if it's a Lua file
    if [[ "$workflow_file" == *.lua ]]; then
        # Basic Lua syntax check (if lua is available)
        if command -v lua >/dev/null 2>&1; then
            if lua -e "dofile('$workflow_file')" >/dev/null 2>&1; then
                print_status "SUCCESS" "✓ Lua syntax is valid"
            else
                print_status "ERROR" "✗ Lua syntax error detected"
                return 1
            fi
        else
            print_status "WARNING" "Lua interpreter not found, skipping syntax check"
        fi
        
        # Check for required workflow structure
        if grep -q "workflow\s*=" "$workflow_file" && grep -q "steps\s*=" "$workflow_file"; then
            print_status "SUCCESS" "✓ Required workflow structure found"
        else
            print_status "ERROR" "✗ Missing required workflow structure (workflow.steps)"
            return 1
        fi
    fi
}

# Function to create a test workflow file
create_test_workflow() {
    local filename=$1
    local content=$2
    
    echo "$content" > "$filename"
    print_status "INFO" "Created test workflow: $filename"
}

# Function to cleanup test files
cleanup_test_files() {
    print_status "INFO" "Cleaning up test files..."
    rm -f workflows/test_*.lua
    rm -f src/main_test_temp.rs
    rm -f src/main_backup.rs
}

# Main test function
run_tests() {
    print_status "INFO" "Starting Workflow File Tests"
    print_status "INFO" "============================"
    echo ""
    
    # Build the project first
    print_status "INFO" "Building project..."
    if ! cargo build --quiet; then
        print_status "ERROR" "Project build failed. Cannot proceed with tests."
        exit 1
    fi
    print_status "SUCCESS" "Project built successfully"
    echo ""
    
    # Test existing workflow files
    print_status "INFO" "Testing existing workflow files..."
    echo ""
    
    # Test hybrid workflow (Python + Lua structure)
    if [ -f "workflows/hybrid_workflow.lua" ]; then
        validate_workflow_syntax "workflows/hybrid_workflow.lua"
        test_workflow "workflows/hybrid_workflow.lua" 0 "Hybrid Python Workflow"
    fi
    
    # Test pure Lua workflow
    if [ -f "workflows/workflow.lua" ]; then
        validate_workflow_syntax "workflows/workflow.lua"
        test_workflow "workflows/workflow.lua" 0 "Pure Lua Workflow"
    fi
    
    # Create and test additional scenarios
    print_status "INFO" "Creating and testing additional scenarios..."
    echo ""
    
    # Test 1: Simple single-step Lua workflow
    create_test_workflow "workflows/test_simple.lua" '
workflow = {
  name = "simple_test",
  description = "Simple single step test",
  steps = {
    hello = {
      run = function()
        print("Hello from Lua!")
        return { message = "Hello World" }
      end
    }
  }
}'
    test_workflow "workflows/test_simple.lua" 0 "Simple Single-Step Lua"
    
    # Test 2: Invalid workflow (missing steps)
    create_test_workflow "workflows/test_invalid.lua" '
workflow = {
  name = "invalid_test",
  description = "Invalid workflow missing steps"
}'
    test_workflow "workflows/test_invalid.lua" 1 "Invalid Workflow (should fail)"
    
    # Test 3: Python workflow with dependencies
    create_test_workflow "workflows/test_python_deps.lua" '
workflow = {
  name = "python_dependency_test",
  description = "Python workflow with dependencies",
  steps = {
    step1 = {
      language = "python",
      code = [[
def run():
    return {"value": 42}
]]
    },
    step2 = {
      depends_on = {"step1"},
      language = "python", 
      code = [[
def run(inputs):
    val = inputs["step1"]["value"]
    return {"doubled": val * 2}
]]
    }
  }
}'
    test_workflow "workflows/test_python_deps.lua" 0 "Python Dependencies Test"
    
    # Test 4: Mixed Lua and Python (if engine supports it)
    create_test_workflow "workflows/test_mixed.lua" '
workflow = {
  name = "mixed_test",
  description = "Mixed Lua and Python workflow",
  steps = {
    lua_step = {
      run = function()
        return { from_lua = "Hello from Lua" }
      end
    },
    python_step = {
      depends_on = {"lua_step"},
      language = "python",
      code = [[
def run(inputs):
    lua_msg = inputs["lua_step"]["from_lua"]
    return {"combined": lua_msg + " -> processed by Python"}
]]
    }
  }
}'
    test_workflow "workflows/test_mixed.lua" 0 "Mixed Lua-Python Workflow"
}

# Function to display test results
show_results() {
    echo ""
    print_status "INFO" "Test Results Summary"
    print_status "INFO" "===================="
    echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
    echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
    echo ""
    
    if [ $FAILED_TESTS -eq 0 ]; then
        print_status "SUCCESS" "All tests passed! 🎉"
        return 0
    else
        print_status "ERROR" "$FAILED_TESTS test(s) failed"
        return 1
    fi
}

# Trap to ensure cleanup on exit
trap cleanup_test_files EXIT

# Check if specific workflow file was provided
if [ $# -eq 1 ]; then
    workflow_file=$1
    print_status "INFO" "Testing specific workflow file: $workflow_file"
    echo ""
    
    if [ ! -f "$workflow_file" ]; then
        print_status "ERROR" "File not found: $workflow_file"
        exit 1
    fi
    
    # Build project
    if ! cargo build --quiet; then
        print_status "ERROR" "Project build failed"
        exit 1
    fi
    
    validate_workflow_syntax "$workflow_file"
    test_workflow "$workflow_file" 0 "Custom Workflow Test"
    show_results
else
    # Run all tests
    run_tests
    show_results
fi