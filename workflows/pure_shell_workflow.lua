-- pure_shell_workflow.lua
-- Demonstrates pure shell/bash workflow

workflow = {
  name = "file_processing_pipeline",
  description = "Pure shell script workflow for file processing",

  steps = {
    -- Create test files
    create_files = {
      language = "bash",
      code = [[
run() {
    # Create temporary directory
    local temp_dir="/tmp/workflow_test_$$"
    mkdir -p "$temp_dir"
    
    # Create sample files
    echo "file1 content" > "$temp_dir/file1.txt"
    echo "file2 content" > "$temp_dir/file2.txt"
    echo "file3 content" > "$temp_dir/file3.txt"
    
    # Count files
    local file_count=$(ls "$temp_dir"/*.txt 2>/dev/null | wc -l)
    
    echo "{\"temp_dir\": \"$temp_dir\", \"files_created\": $file_count, \"status\": \"files_ready\"}"
}
]]
    },

    -- Process files
    process_files = {
      depends_on = { "create_files" },
      language = "shell",
      code = [[
run() {
    # Get the temp directory from previous step
    local input_data=$(parse_input "create_files")
    local temp_dir=$(echo "$input_data" | grep -o '"temp_dir":"[^"]*"' | cut -d'"' -f4)
    
    # Process each file
    local processed_count=0
    for file in "$temp_dir"/*.txt; do
        if [ -f "$file" ]; then
            # Convert to uppercase and add timestamp
            local processed_file="${file%.txt}_processed.txt"
            {
                echo "Processed at: $(date)"
                tr '[:lower:]' '[:upper:]' < "$file" 2>/dev/null
            } > "$processed_file"
            processed_count=$((processed_count + 1))
        fi
    done
    
    echo "{\"temp_dir\": \"$temp_dir\", \"processed_files\": $processed_count, \"status\": \"processing_complete\"}"
}
]]
    },

    -- Generate statistics
    generate_stats = {
      depends_on = { "process_files" },
      language = "bash",
      code = [[
run() {
    echo "Generating file statistics..."
    
    # Get temp directory
    local input_data=$(parse_input "process_files")
    local temp_dir=$(echo "$input_data" | grep -o '"temp_dir":"[^"]*"' | cut -d'"' -f4)
    
    # Calculate statistics
    local total_files=$(find "$temp_dir" -name "*.txt" | wc -l)
    local original_files=$(find "$temp_dir" -name "*processed.txt" -prune -o -name "*.txt" -print | wc -l)
    local processed_files=$(find "$temp_dir" -name "*processed.txt" | wc -l)
    local total_size=$(find "$temp_dir" -type f -exec wc -c {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    
    echo "Statistics:"
    echo "- Total files: $total_files"
    echo "- Original files: $original_files" 
    echo "- Processed files: $processed_files"
    echo "- Total size: $total_size bytes"
    
    echo "{\"stats\": {\"total_files\": $total_files, \"original_files\": $original_files, \"processed_files\": $processed_files, \"total_size_bytes\": $total_size}, \"temp_dir\": \"$temp_dir\"}"
}
]]
    },

    -- Cleanup
    cleanup = {
      depends_on = { "generate_stats" },
      language = "bash",
      code = [[
run() {
    echo "Cleaning up temporary files..."
    
    # Get temp directory
    local input_data=$(parse_input "generate_stats")
    local temp_dir=$(echo "$input_data" | grep -o '"temp_dir":"[^"]*"' | cut -d'"' -f4)
    
    # List files before cleanup
    echo "Files before cleanup:"
    ls -la "$temp_dir"
    
    # Remove temporary directory
    rm -rf "$temp_dir"
    
    echo "Cleanup completed"
    echo "{\"status\": \"cleanup_complete\", \"temp_dir_removed\": \"$temp_dir\"}"
}
]]
    }
  }
}