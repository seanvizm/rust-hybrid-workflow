-- js_python_shell_workflow.lua

workflow = {
  name = "javascript_integration_pipeline",
  description = "Workflow demonstrating JavaScript integration with Python and Shell",

  steps = {
    -- Python: Generate initial data
    generate_data = {
      language = "python",
      code = [[
def run():
    print("Generating data with Python...")
    import random
    
    data = {
        "numbers": [random.randint(1, 100) for _ in range(20)],
        "timestamp": "2024-10-26T10:00:00Z",
        "source": "python_generator"
    }
    
    return data
]]
    },

    -- JavaScript: Process and analyze the data
    analyze_data = {
      depends_on = { "generate_data" },
      language = "javascript",
      code = [[
function run(inputs) {
    console.log("Analyzing data with JavaScript...");
    
    const numbers = inputs.generate_data.numbers;
    
    // Perform various JavaScript analytics
    const stats = {
        count: numbers.length,
        sum: numbers.reduce((a, b) => a + b, 0),
        average: numbers.reduce((a, b) => a + b, 0) / numbers.length,
        max: Math.max(...numbers),
        min: Math.min(...numbers),
        median: [...numbers].sort((a, b) => a - b)[Math.floor(numbers.length / 2)]
    };
    
    // Advanced processing
    const processed = {
        above_average: numbers.filter(n => n > stats.average),
        below_average: numbers.filter(n => n <= stats.average),
        quartiles: {
            q1: [...numbers].sort((a, b) => a - b)[Math.floor(numbers.length * 0.25)],
            q2: stats.median,
            q3: [...numbers].sort((a, b) => a - b)[Math.floor(numbers.length * 0.75)]
        }
    };
    
    // Create analysis report
    const analysis = {
        original_data: {
            source: inputs.generate_data.source,
            timestamp: inputs.generate_data.timestamp,
            count: numbers.length
        },
        statistics: stats,
        distributions: {
            above_average_count: processed.above_average.length,
            below_average_count: processed.below_average.length,
            above_average_percentage: (processed.above_average.length / numbers.length * 100).toFixed(2) + "%"
        },
        quartile_analysis: processed.quartiles,
        processed_at: new Date().toISOString()
    };
    
    console.log("JavaScript Analysis Complete:");
    console.log("- Average:", stats.average.toFixed(2));
    console.log("- Above average:", processed.above_average.length, "numbers");
    console.log("- Range:", stats.min, "to", stats.max);
    
    return analysis;
}
]]
    },

    -- Python: Final processing and machine learning insights
    ml_insights = {
      depends_on = { "analyze_data" },
      language = "python",
      code = [[
def run(inputs):
    print("Generating ML insights with Python...")
    
    analysis = inputs["analyze_data"]
    stats = analysis["statistics"]
    
    # Simulate some ML-like analysis
    insights = {
        "data_quality": {
            "score": min(100, max(0, 100 - abs(stats["average"] - 50) * 2)),
            "variance": stats["max"] - stats["min"],
            "distribution": "normal" if abs(stats["average"] - stats["median"]) < 5 else "skewed"
        },
        "recommendations": [],
        "confidence": 0.85
    }
    
    # Generate recommendations based on analysis
    if stats["average"] > 75:
        insights["recommendations"].append("Data shows high values - consider upper bound validation")
    elif stats["average"] < 25:
        insights["recommendations"].append("Data shows low values - investigate data source")
    else:
        insights["recommendations"].append("Data distribution appears normal")
    
    if stats["max"] - stats["min"] > 80:
        insights["recommendations"].append("High variance detected - consider outlier analysis")
    
    # Combine with JavaScript analysis
    final_result = {
        "ml_insights": insights,
        "javascript_analysis": {
            "processed_count": analysis["original_data"]["count"],
            "above_average_pct": analysis["distributions"]["above_average_percentage"],
            "quartile_range": analysis["quartile_analysis"]
        },
        "processing_chain": ["Python (generation)", "JavaScript (analysis)", "Python (ML insights)"],
        "completed_at": "2024-10-26T" + str(__import__('time').time())
    }
    
    return final_result
]]
    },

    -- Shell: Generate final report and cleanup
    generate_report = {
      depends_on = { "ml_insights" },
      language = "shell",
      code = [[
#!/bin/bash

function run() {
    echo "Generating final report with Shell..."
    
    # Get timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    report_id="JS-WORKFLOW-$(date +%s)"
    
    echo "=== JAVASCRIPT INTEGRATION WORKFLOW REPORT ==="
    echo "Report ID: $report_id"
    echo "Generated: $timestamp"
    echo "Languages Used: Python -> JavaScript -> Python -> Shell"
    echo "=============================================="
    
    # Simulate some file operations
    temp_dir="/tmp/js_workflow_$$"
    mkdir -p "$temp_dir"
    
    # Create a summary file
    echo "Multi-language workflow completed successfully" > "$temp_dir/summary.txt"
    echo "JavaScript integration: PASSED" >> "$temp_dir/summary.txt"
    echo "Data processing: COMPLETED" >> "$temp_dir/summary.txt"
    
    # Read the file and clean up
    summary_content=$(cat "$temp_dir/summary.txt")
    rm -rf "$temp_dir"
    
    # Return structured result
    echo '{
        "report": {
            "id": "'$report_id'",
            "timestamp": "'$timestamp'",
            "status": "completed",
            "summary": "'$summary_content'",
            "languages_used": ["Python", "JavaScript", "Python", "Shell"],
            "integration_test": "PASSED"
        },
        "system": {
            "hostname": "'$(hostname)'",
            "user": "'$(whoami)'",
            "shell": "'$SHELL'"
        }
    }'
}
]]
    }
  }
}