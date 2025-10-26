-- comprehensive_workflow.lua
-- Demonstrates all supported languages: Lua, Python, and Shell/Bash

workflow = {
  name = "comprehensive_multi_language_pipeline",
  description = "Comprehensive workflow demonstrating Lua, Python, and Shell integration",

  steps = {
    -- Step 1: Initialize configuration with Lua
    lua_config = {
      language = "lua",
      code = [[
function run()
    print("Step 1: Initializing configuration with Lua...")
    return {
        config = {
            batch_size = 100,
            output_format = "json",
            enable_logging = true
        },
        metadata = {
            created_by = "lua",
            timestamp = os.date("%Y-%m-%d %H:%M:%S")
        }
    }
end
]]
    },

    -- Step 2: Data generation with Python
    python_data_gen = {
      depends_on = { "lua_config" },
      language = "python",
      code = [[
def run(inputs):
    import random
    import json
    
    config = inputs["lua_config"]["config"]
    batch_size = config["batch_size"]
    
    print(f"Step 2: Generating {batch_size} data points with Python...")
    
    # Generate sample data
    data = []
    for i in range(batch_size):
        data.append({
            "id": i + 1,
            "value": random.randint(1, 1000),
            "category": random.choice(["A", "B", "C"])
        })
    
    # Calculate statistics
    values = [item["value"] for item in data]
    stats = {
        "count": len(data),
        "min": min(values),
        "max": max(values),
        "avg": sum(values) / len(values)
    }
    
    return {
        "data": data[:10],  # Return first 10 items for brevity
        "stats": stats,
        "total_generated": len(data),
        "processing_status": "data_generated"
    }
]]
    },

    -- Step 3: System information gathering with Shell
    shell_system_info = {
      depends_on = { "python_data_gen" },
      language = "bash",
      code = [[
run() {
    echo "Step 3: Gathering system information with Bash..."
    
    # Get system information
    local os_info=$(uname -a)
    local disk_usage=$(df -h / | awk 'NR==2 {print $5}')
    local memory_info=$(free -h 2>/dev/null || echo "N/A (not Linux)")
    local current_user=$(whoami)
    local working_dir=$(pwd)
    
    # Get data stats from previous step
    local python_data=$(parse_input "python_data_gen")
    local data_count=$(echo "$python_data" | grep -o '"total_generated":[0-9]*' | cut -d':' -f2)
    
    echo "Processed $data_count records from Python step"
    
    # Create comprehensive system report
    cat << EOF
{
  "system_info": {
    "os": "$os_info",
    "disk_usage": "$disk_usage",
    "memory": "$memory_info",
    "user": "$current_user",
    "working_directory": "$working_dir",
    "timestamp": "$(date '+%Y-%m-%d %H:%M:%S')"
  },
  "data_processing": {
    "records_processed": $data_count,
    "source_step": "python_data_gen"
  },
  "shell_status": "system_info_collected"
}
EOF
}
]]
    },

    -- Step 4: File operations with Shell
    shell_file_ops = {
      depends_on = { "shell_system_info" },
      language = "shell",
      code = [[
run() {
    echo "Step 4: Performing file operations with Shell..."
    
    # Create temporary workspace
    local temp_dir="/tmp/workflow_$$"
    mkdir -p "$temp_dir"
    
    # Get system info from previous step
    local system_data=$(parse_input "shell_system_info")
    
    # Create some files based on system info
    echo "Workflow execution report" > "$temp_dir/report.txt"
    echo "Generated at: $(date)" >> "$temp_dir/report.txt"
    echo "System: $(uname -s)" >> "$temp_dir/report.txt"
    echo "" >> "$temp_dir/report.txt"
    echo "Previous step data:" >> "$temp_dir/report.txt"
    echo "$system_data" | head -5 >> "$temp_dir/report.txt"  # First few lines
    
    # Create a data file
    echo "id,value,category" > "$temp_dir/sample_data.csv"
    echo "1,100,A" >> "$temp_dir/sample_data.csv"
    echo "2,200,B" >> "$temp_dir/sample_data.csv"
    echo "3,300,C" >> "$temp_dir/sample_data.csv"
    
    # Get file statistics
    local file_count=$(ls -1 "$temp_dir" | wc -l)
    local total_size=$(find "$temp_dir" -type f -exec wc -c {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    
    echo "Created $file_count files in $temp_dir"
    echo "Total size: $total_size bytes"
    
    # Output results
    echo "{\"temp_workspace\": \"$temp_dir\", \"files_created\": $file_count, \"total_size_bytes\": $total_size, \"files\": [\"report.txt\", \"sample_data.csv\"], \"status\": \"files_ready\"}"
}
]]
    },

    -- Step 5: Data analysis with Python
    python_analysis = {
      depends_on = { "python_data_gen", "shell_file_ops" },
      language = "python",
      code = [[
def run(inputs):
    import json
    import os
    
    print("Step 5: Performing data analysis with Python...")
    
    # Get data from python step
    data_info = inputs["python_data_gen"]["stats"]
    
    # Get file info from shell step
    file_info = inputs["shell_file_ops"]
    temp_dir = file_info["temp_workspace"]
    
    print(f"Analyzing data stats: {data_info}")
    print(f"Files created in: {temp_dir}")
    
    # Enhanced analysis
    analysis = {
        "data_quality": {
            "range_spread": data_info["max"] - data_info["min"],
            "avg_value": round(data_info["avg"], 2),
            "data_points": data_info["count"]
        },
        "file_operations": {
            "workspace": temp_dir,
            "files_created": file_info["files_created"],
            "total_size": file_info["total_size_bytes"]
        },
        "recommendations": []
    }
    
    # Add recommendations based on analysis
    if data_info["avg"] > 500:
        analysis["recommendations"].append("High average values detected - consider scaling")
    
    if file_info["files_created"] > 0:
        analysis["recommendations"].append("Files successfully created for further processing")
    
    return {
        "analysis": analysis,
        "processing_complete": True,
        "next_actions": ["cleanup", "reporting"],
        "python_status": "analysis_complete"
    }
]]
    },

    -- Step 6: Final cleanup and reporting with Lua
    lua_finalize = {
      depends_on = { "python_analysis", "shell_file_ops" },
      language = "lua",
      code = [[
function run(inputs)
    print("Step 6: Finalizing workflow with Lua...")
    
    local analysis = inputs["python_analysis"]["analysis"]
    local file_ops = inputs["shell_file_ops"]
    
    print("Analysis results:")
    print("  - Data quality range spread:", analysis["data_quality"]["range_spread"])
    print("  - Average value:", analysis["data_quality"]["avg_value"])
    print("  - Files created:", analysis["file_operations"]["files_created"])
    
    -- Create final summary
    local summary = {
        workflow_name = "comprehensive_multi_language_pipeline",
        steps_completed = 6,
        languages_used = {"lua", "python", "shell", "bash"},
        processing_stats = {
            data_points_processed = analysis["data_quality"]["data_points"],
            files_created = analysis["file_operations"]["files_created"],
            workspace_used = analysis["file_operations"]["workspace"]
        },
        recommendations = analysis["recommendations"],
        final_status = "SUCCESS",
        completed_at = os.date("%Y-%m-%d %H:%M:%S")
    }
    
    print("✅ Workflow completed successfully!")
    print("Languages demonstrated: Lua, Python, Shell/Bash")
    
    return summary
end
]]
    },

    -- Step 7: Optional cleanup with Shell
    shell_cleanup = {
      depends_on = { "lua_finalize" },
      language = "bash",
      code = [[
run() {
    echo "Step 7: Performing cleanup with Bash..."
    
    # Get workspace from finalize step
    local finalize_data=$(parse_input "lua_finalize")
    local temp_dir=$(echo "$finalize_data" | grep -o '"workspace_used":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$temp_dir" ] && [ -d "$temp_dir" ]; then
        echo "Cleaning up temporary workspace: $temp_dir"
        
        # List files before cleanup
        echo "Files to be removed:"
        ls -la "$temp_dir"
        
        # Remove temporary directory
        rm -rf "$temp_dir"
        echo "✅ Cleanup completed"
    else
        echo "No temporary workspace to clean up"
    fi
    
    echo '{"cleanup_status": "completed", "workspace_removed": true, "final_step": true}'
}
]]
    }
  }
}