#!/bin/bash

# Quick Workflow Tester - Simplified for new config system
# Usage: ./quick_test.sh [workflow_file]
#
# Examples:
#   ./quick_test.sh                    # Test all workflows
#   ./quick_test.sh workflow.lua       # Test specific workflow
#   ./quick_test.sh workflows/test.lua # Test with full path
#
# NOTE: For comprehensive testing with 49+ unit/integration tests, use:
#   cargo test
#
# This script is beneficial for:
#   - Quick manual workflow validation
#   - Testing a single workflow without full test suite
#   - CI/CD smoke tests for workflow execution
#   - Debugging workflow issues in isolation

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_msg() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to test a workflow using the compiled binary
test_single_workflow() {
    local workflow_file=$1
    local test_name=$(basename "$workflow_file")
    
    print_msg "Testing: $test_name"
    
    if [ ! -f "$workflow_file" ]; then
        print_error "File not found: $workflow_file"
        return 1
    fi
    
    # Run test using the binary directly
    local start_time=$(date +%s)
    if cargo run --bin hybrid-workflow-engine --quiet -- "$workflow_file" > /tmp/quick_test_output.log 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_success "✓ $test_name completed successfully (${duration}s)"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_error "✗ $test_name failed (${duration}s)"
        echo "Output:"
        cat /tmp/quick_test_output.log
        return 1
    fi
}

# Main execution
echo "🚀 Quick Workflow Tester"
echo "========================"
echo ""
print_warning "For comprehensive testing, use: cargo test"
print_msg "This script is for quick manual workflow validation"
echo ""

# Build project first
print_msg "Building project..."
if cargo build --bin hybrid-workflow-engine --quiet 2>&1; then
    print_success "Build completed"
else
    print_error "Build failed"
    exit 1
fi

echo ""

if [ $# -eq 1 ]; then
    # Test specific file
    workflow_path="$1"
    
    # If just filename, prepend workflows/
    if [[ "$workflow_path" != */* ]]; then
        workflow_path="workflows/$workflow_path"
    fi
    
    test_single_workflow "$workflow_path"
    exit_code=$?
    
    # Cleanup
    rm -f /tmp/quick_test_output.log
    exit $exit_code
else
    # Test all workflow files
    success_count=0
    total_count=0
    
    print_msg "Testing all workflows in workflows/ directory..."
    echo ""
    
    for workflow in workflows/*.lua; do
        if [ -f "$workflow" ]; then
            total_count=$((total_count + 1))
            if test_single_workflow "$workflow"; then
                success_count=$((success_count + 1))
            fi
            echo ""
        fi
    done
    
    echo "================================"
    echo "Summary: $success_count/$total_count workflows passed"
    echo "================================"
    
    # Cleanup
    rm -f /tmp/quick_test_output.log
    
    if [ $success_count -eq $total_count ]; then
        print_success "All workflows passed! 🎉"
        echo ""
        print_msg "Tip: Run 'cargo test' for comprehensive unit and integration tests"
        exit 0
    else
        print_error "Some workflows failed"
        echo ""
        print_msg "Tip: Run 'cargo test' for detailed test results"
        exit 1
    fi
fi