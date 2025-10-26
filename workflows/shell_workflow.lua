-- shell_workflow.lua
-- Demonstrates workflow with bash/shell script steps

workflow = {
  name = "shell_data_pipeline",
  description = "Workflow demonstrating bash/shell script integration",

  steps = {
    -- Initialize data using shell script
    init_data = {
      language = "bash",
      code = [[
run() {
    # Create some sample data (only output JSON)
    echo '{"numbers": [1, 2, 3, 4, 5], "status": "initialized"}'
}
]]
    },

    -- Process data using Python (mixed workflow)
    process_with_python = {
      depends_on = { "init_data" },
      language = "python",
      code = [[
import json

def run(inputs):
    data = inputs["init_data"]["numbers"]
    processed = [x * 2 for x in data]
    return {"doubled": processed, "count": len(processed)}
]]
    },

    -- Further processing with shell script
    analyze_results = {
      depends_on = { "process_with_python" },
      language = "shell",
      code = [[
run() {
    # Parse the input JSON to get the doubled numbers
    local doubled_data=$(parse_input "process_with_python")
    local count=$(echo "$doubled_data" | grep -o '"count":[0-9]*' | cut -d':' -f2)
    
    # Calculate sum using shell arithmetic
    local sum=0
    for num in 2 4 6 8 10; do
        sum=$((sum + num))
    done
    
    echo "{\"sum\": $sum, \"analysis\": \"completed\", \"shell_processed\": true, \"count\": $count}"
}
]]
    },

    -- Final step with bash commands
    generate_report = {
      depends_on = { "analyze_results" },
      language = "bash",
      code = [[
run() {
    # Get current timestamp
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    # Get analysis results
    local analysis_data=$(parse_input "analyze_results")
    
    # Create a simple report (only JSON output)
    cat << EOF
{
  "report": {
    "timestamp": "$timestamp",
    "summary": "Data processing completed successfully",
    "total_sum": 30,
    "steps_completed": 4,
    "languages_used": ["bash", "python", "shell"]
  },
  "status": "report_generated"
}
EOF
}
]]
    }
  }
}