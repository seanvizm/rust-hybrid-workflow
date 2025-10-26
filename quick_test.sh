#!/bin/bash

# Quick Workflow Tester - Simple version for quick testing
# Usage: ./quick_test.sh [workflow_file]

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
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

# Function to resolve workflow path
resolve_workflow_path() {
    local path=$1
    
    # If path already starts with workflows/, use as-is
    if [[ "$path" == workflows/* ]]; then
        echo "$path"
        return
    fi
    
    # If it's just a filename, prepend workflows/
    if [[ "$path" != */* ]]; then
        local workflow_path="workflows/$path"
        if [ -f "$workflow_path" ]; then
            echo "$workflow_path"
            return
        fi
    fi
    
    # Return original path
    echo "$path"
}

# Function to test a workflow
test_single_workflow() {
    local workflow_input=$1
    local workflow_file=$(resolve_workflow_path "$workflow_input")
    local test_name=${2:-$(basename "$workflow_file")}
    
    print_msg "Testing: $test_name"
    
    if [ ! -f "$workflow_file" ]; then
        print_error "File not found: $workflow_file"
        return 1
    fi
    
    # Create temporary main.rs
    cat > src/main_temp.rs << EOF
mod core;
mod runners;

use core::run_workflow;

fn main() -> anyhow::Result<()> {
    println!("Testing workflow: $workflow_file");
    println!("=================================");
    run_workflow("$workflow_file")?;
    Ok(())
}
EOF
    
    # Backup and replace main.rs
    cp src/main.rs src/main.backup
    mv src/main_temp.rs src/main.rs
    
    # Run test
    local start_time=$(date +%s)
    if cargo run --quiet; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_success "✓ $test_name completed successfully (${duration}s)"
        
        # Restore main.rs
        mv src/main.backup src/main.rs
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_error "✗ $test_name failed (${duration}s)"
        
        # Restore main.rs
        mv src/main.backup src/main.rs
        return 1
    fi
}

# Main execution
echo "🚀 Quick Workflow Tester"
echo "========================"

# Build project first
print_msg "Building project..."
if cargo build --quiet; then
    print_success "Build completed"
else
    print_error "Build failed"
    exit 1
fi

echo ""

if [ $# -eq 1 ]; then
    # Test specific file
    test_single_workflow "$1"
else
    # Test all known workflow files
    success_count=0
    total_count=0
    
    for workflow in workflows/*.lua; do
        if [ -f "$workflow" ]; then
            total_count=$((total_count + 1))
            if test_single_workflow "$workflow"; then
                success_count=$((success_count + 1))
            fi
            echo ""
        fi
    done
    
    echo "Summary: $success_count/$total_count workflows passed"
    
    if [ $success_count -eq $total_count ]; then
        print_success "All workflows passed! 🎉"
        exit 0
    else
        print_error "Some workflows failed"
        exit 1
    fi
fi