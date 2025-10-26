-- wasm_workflow.lua

workflow = {
  name = "webassembly_demo_pipeline",
  description = "Workflow demonstrating WebAssembly integration",

  steps = {
    -- Python: Generate some input data
    generate_input = {
      language = "python",
      code = [[
def run():
    print("Generating input data for WASM processing...")
    
    # Generate some test data
    data = {
        "numbers": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "fibonacci_input": 10,
        "computation_params": {
            "iterations": 100,
            "base_value": 1
        },
        "timestamp": "2024-10-26T12:00:00Z"
    }
    
    print(f"Generated data with {len(data['numbers'])} numbers")
    print(f"Fibonacci input: {data['fibonacci_input']}")
    
    return data
]]
    },

    -- WASM: Basic computation
    wasm_basic_computation = {
      depends_on = { "generate_input" },
      language = "wasm",
      module = "workflows/example_wasm_module.wasm",
      func = "run"
    },

    -- WASM: Data processing
    wasm_process_data = {
      depends_on = { "wasm_basic_computation" },
      language = "wasm", 
      module = "workflows/example_wasm_module.wasm",
      func = "process_data"
    },

    -- WASM: Complex computation
    wasm_complex_computation = {
      depends_on = { "wasm_process_data" },
      language = "wasm",
      module = "workflows/example_wasm_module.wasm", 
      func = "complex_computation"
    },

    -- JavaScript: Analyze WASM results
    analyze_wasm_results = {
      depends_on = { "generate_input", "wasm_basic_computation", "wasm_process_data", "wasm_complex_computation" },
      language = "javascript",
      code = [[
function run(inputs) {
    console.log("Analyzing WebAssembly execution results...");
    
    const inputData = inputs.generate_input;
    const basicResult = inputs.wasm_basic_computation;
    const processResult = inputs.wasm_process_data;
    const complexResult = inputs.wasm_complex_computation;
    
    // Analyze the WASM execution results
    const analysis = {
        input_summary: {
            numbers_count: inputData.numbers.length,
            fibonacci_input: inputData.fibonacci_input,
            timestamp: inputData.timestamp
        },
        wasm_execution_results: {
            basic_computation: {
                return_code: basicResult.wasm_execution.return_code,
                status: basicResult.wasm_execution.status,
                success: basicResult.wasm_execution.return_code === 0
            },
            data_processing: {
                return_code: processResult.wasm_execution.return_code,
                status: processResult.wasm_execution.status,
                success: processResult.wasm_execution.return_code === 0
            },
            complex_computation: {
                return_code: complexResult.wasm_execution.return_code,
                status: complexResult.wasm_execution.status,
                success: complexResult.wasm_execution.return_code === 0
            }
        },
        overall_analysis: {
            total_wasm_steps: 3,
            successful_steps: 0,
            failed_steps: 0,
            performance_metrics: {
                wasm_integration: "successful",
                data_flow: "multi_language",
                execution_chain: ["Python", "WASM", "WASM", "WASM", "JavaScript"]
            }
        }
    };
    
    // Count successful/failed steps
    const wasmResults = [
        basicResult.wasm_execution.return_code,
        processResult.wasm_execution.return_code,
        complexResult.wasm_execution.return_code
    ];
    
    analysis.overall_analysis.successful_steps = wasmResults.filter(code => code === 0).length;
    analysis.overall_analysis.failed_steps = wasmResults.filter(code => code !== 0).length;
    
    // Generate summary
    const successRate = (analysis.overall_analysis.successful_steps / analysis.overall_analysis.total_wasm_steps * 100).toFixed(1);
    
    console.log("\\n=== WEBASSEMBLY EXECUTION ANALYSIS ===");
    console.log(`Total WASM steps executed: ${analysis.overall_analysis.total_wasm_steps}`);
    console.log(`Successful steps: ${analysis.overall_analysis.successful_steps}`);
    console.log(`Failed steps: ${analysis.overall_analysis.failed_steps}`);
    console.log(`Success rate: ${successRate}%`);
    console.log("======================================\\n");
    
    return {
        analysis: analysis,
        summary: {
            success_rate: successRate + "%",
            wasm_integration_status: analysis.overall_analysis.successful_steps > 0 ? "SUCCESS" : "FAILED",
            languages_used: analysis.overall_analysis.performance_metrics.execution_chain
        },
        processed_at: new Date().toISOString()
    };
}
]]
    },

    -- Python: Final summary and validation
    create_final_report = {
      depends_on = { "analyze_wasm_results" },
      language = "python",
      code = [[
def run(inputs):
    print("Creating final WebAssembly integration report...")
    
    analysis = inputs["analyze_wasm_results"]["analysis"]
    summary = inputs["analyze_wasm_results"]["summary"]
    
    # Create comprehensive report
    report = {
        "workflow_info": {
            "name": "WebAssembly Integration Demo",
            "languages_demonstrated": summary["languages_used"],
            "total_steps": 5,
            "wasm_steps": analysis["overall_analysis"]["total_wasm_steps"]
        },
        "wasm_performance": {
            "success_rate": summary["success_rate"],
            "integration_status": summary["wasm_integration_status"],
            "execution_details": analysis["wasm_execution_results"]
        },
        "technical_achievements": [
            "Multi-language workflow orchestration",
            "WebAssembly module integration", 
            "Cross-language data passing",
            "Rust-compiled WASM execution",
            "JavaScript analysis of WASM results"
        ],
        "use_cases_demonstrated": [
            "High-performance computation with WASM",
            "Secure code execution in sandboxed environment",
            "Language interoperability (Python + WASM + JavaScript)",
            "Complex workflow dependency management"
        ]
    }
    
    print("\\n" + "="*60)
    print("WEBASSEMBLY WORKFLOW INTEGRATION REPORT")
    print("="*60)
    print(f"Integration Status: {report['wasm_performance']['integration_status']}")
    print(f"WASM Success Rate: {report['wasm_performance']['success_rate']}")
    print(f"Languages Used: {', '.join(report['workflow_info']['languages_demonstrated'])}")
    print(f"Total Steps: {report['workflow_info']['total_steps']}")
    print(f"WASM Steps: {report['workflow_info']['wasm_steps']}")
    print("\\nTechnical Achievements:")
    for achievement in report["technical_achievements"]:
        print(f"  ✓ {achievement}")
    print("="*60 + "\\n")
    
    return report
]]
    }
  }
}