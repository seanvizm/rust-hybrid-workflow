-- multi_language_workflow.lua

workflow = {
  name = "multi_language_pipeline",
  description = "Comprehensive workflow using Python, JavaScript, Lua, and Shell",

  steps = {
    -- Python: Data fetching and initial processing
    fetch_api_data = {
      language = "python",
      code = [[
def run():
    print("Fetching data with Python...")
    import json
    import random
    
    # Simulate API data fetch
    api_data = {
        "users": [
            {"id": i, "name": f"User{i}", "score": random.randint(50, 100)}
            for i in range(1, 21)
        ],
        "metadata": {
            "source": "mock_api",
            "timestamp": "2024-10-26T10:00:00Z",
            "total_users": 20
        }
    }
    
    return api_data
]]
    },

    -- JavaScript: Advanced data analysis
    analyze_with_js = {
      depends_on = { "fetch_api_data" },
      language = "javascript",
      code = [[
function run(inputs) {
    console.log("Analyzing data with JavaScript...");
    
    const users = inputs.fetch_api_data.users;
    const metadata = inputs.fetch_api_data.metadata;
    
    // Advanced analytics using JavaScript's array methods
    const scores = users.map(user => user.score);
    const sortedUsers = [...users].sort((a, b) => b.score - a.score);
    
    const analytics = {
        total_users: users.length,
        score_stats: {
            average: scores.reduce((a, b) => a + b, 0) / scores.length,
            median: scores.sort((a, b) => a - b)[Math.floor(scores.length / 2)],
            max: Math.max(...scores),
            min: Math.min(...scores)
        },
        top_performers: sortedUsers.slice(0, 5).map(user => ({
            id: user.id,
            name: user.name,
            score: user.score,
            percentile: ((sortedUsers.length - sortedUsers.indexOf(user)) / sortedUsers.length * 100).toFixed(1)
        })),
        score_distribution: {
            excellent: users.filter(u => u.score >= 90).length,
            good: users.filter(u => u.score >= 80 && u.score < 90).length,
            average: users.filter(u => u.score >= 70 && u.score < 80).length,
            below_average: users.filter(u => u.score < 70).length
        }
    };
    
    return {
        original_metadata: metadata,
        analytics: analytics,
        processed_at: new Date().toISOString()
    };
}
]]
    },

    -- Lua: Configuration and business logic
    apply_business_rules = {
      depends_on = { "analyze_with_js" },
      language = "lua",
      code = [[
function run(inputs)
    print("Applying business rules with Lua...")
    
    local analytics = inputs.analyze_with_js.analytics
    local top_performers = analytics.top_performers
    
    -- Business rules in Lua
    local rewards = {}
    local total_bonus = 0
    
    for i, performer in ipairs(top_performers) do
        local bonus = 0
        if performer.score >= 95 then
            bonus = 1000
        elseif performer.score >= 90 then
            bonus = 500
        elseif performer.score >= 85 then
            bonus = 250
        end
        
        table.insert(rewards, {
            user_id = performer.id,
            name = performer.name,
            score = performer.score,
            bonus = bonus,
            rank = i
        })
        
        total_bonus = total_bonus + bonus
    end
    
    -- Generate recommendations
    local recommendations = {}
    if analytics.score_stats.average < 75 then
        table.insert(recommendations, "Consider additional training programs")
    end
    if analytics.score_distribution.below_average > analytics.total_users * 0.3 then
        table.insert(recommendations, "Implement mentorship program")
    end
    if analytics.score_distribution.excellent < 3 then
        table.insert(recommendations, "Create stretch goals for high performers")
    end
    
    return {
        reward_program = {
            rewards = rewards,
            total_bonus_allocated = total_bonus,
            eligible_users = #rewards
        },
        recommendations = recommendations,
        business_metrics = {
            performance_score = analytics.score_stats.average,
            excellence_rate = (analytics.score_distribution.excellent / analytics.total_users) * 100
        }
    }
end
]]
    },

    -- Shell: System operations and reporting
    generate_reports = {
      depends_on = { "apply_business_rules" },
      language = "shell",
      code = [[
#!/bin/bash

function run() {
    echo "Generating system reports with Shell..."
    
    # Parse input data (simplified for demo)
    local rewards_data=$(parse_input "apply_business_rules")
    
    # Create timestamp
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local report_id="RPT-$(date +%s)"
    
    # Simulate report generation
    echo "=== PERFORMANCE REPORT ==="
    echo "Report ID: $report_id"
    echo "Generated: $timestamp"
    echo "=========================="
    
    # Simulate file operations
    local temp_report="/tmp/performance_report_$report_id.txt"
    echo "Performance Analysis Report" > "$temp_report"
    echo "Generated: $timestamp" >> "$temp_report"
    echo "Status: Report generated successfully" >> "$temp_report"
    
    # Clean up
    rm -f "$temp_report" 2>/dev/null
    
    # Return structured data
    echo '{
        "report_info": {
            "report_id": "'$report_id'",
            "generated_at": "'$timestamp'",
            "status": "completed",
            "format": "multi_format",
            "file_operations": "completed"
        },
        "system_info": {
            "hostname": "'$(hostname)'",
            "user": "'$(whoami)'",
            "pwd": "'$(pwd)'"
        }
    }'
}
]]
    },

    -- Python: Final consolidation and summary
    create_final_summary = {
      depends_on = { "fetch_api_data", "analyze_with_js", "apply_business_rules", "generate_reports" },
      language = "python",
      code = [[
def run(inputs):
    print("Creating final summary with Python...")
    
    # Collect data from all previous steps
    original_data = inputs["fetch_api_data"]
    js_analytics = inputs["analyze_with_js"]["analytics"]
    lua_business = inputs["apply_business_rules"]
    shell_reports = inputs["generate_reports"]
    
    # Create comprehensive summary
    summary = {
        "workflow_summary": {
            "name": "Multi-Language Data Pipeline",
            "steps_completed": 5,
            "languages_used": ["Python", "JavaScript", "Lua", "Shell"],
            "total_processing_time": "simulated"
        },
        "data_flow": {
            "initial_users": original_data["metadata"]["total_users"],
            "average_score": round(js_analytics["score_stats"]["average"], 2),
            "top_performers": len(js_analytics["top_performers"]),
            "total_bonus_allocated": lua_business["reward_program"]["total_bonus_allocated"],
            "recommendations_count": len(lua_business["recommendations"])
        },
        "system_info": {
            "report_id": shell_reports["report_info"]["report_id"],
            "hostname": shell_reports["system_info"]["hostname"]
        },
        "language_contributions": {
            "python": "Data fetching and final summary",
            "javascript": "Advanced analytics and data processing", 
            "lua": "Business rules and reward calculations",
            "shell": "System operations and report generation"
        }
    }
    
    print("\n" + "="*50)
    print("MULTI-LANGUAGE WORKFLOW COMPLETED")
    print("="*50)
    import json
    print(json.dumps(summary, indent=2))
    print("="*50 + "\n")
    
    return summary
]]
    }
  }
}