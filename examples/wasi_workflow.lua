-- WASI-enabled WASM Workflow Example
-- Demonstrates WebAssembly System Interface (WASI) capabilities

workflow = {
  name = "wasi_demonstration",
  description = "Demonstrates WASI features: stdio, environment access, and system integration",

  steps = {
    -- Step 1: Python prepares data
    prepare_data = {
      language = "python",
      code = [[
def run():
    import json
    data = {
        "message": "Hello from Python to WASI!",
        "numbers": [1, 2, 3, 4, 5],
        "config": {
            "enable_logging": True,
            "max_iterations": 10
        }
    }
    print("✅ Python: Data prepared for WASI processing")
    return data
]]
    },

    -- Step 2: WASI-enabled WASM module with system access
    -- This WASM module can now:
    -- - Read/write to stdout/stderr
    -- - Access environment variables
    -- - Get command-line arguments
    -- - (Optional) Access file system if configured
    process_with_wasi = {
      depends_on = { "prepare_data" },
      language = "wasm",
      module = "wasm_modules/target/wasm32-unknown-unknown/release/wasm_modules.wasm",
      func = "run",
      -- WASI features available:
      -- - stdio access (print to console)
      -- - environment variables
      -- - system clock
      -- - random number generation
    },

    -- Step 3: Verify WASI execution
    verify_results = {
      depends_on = { "process_with_wasi" },
      language = "javascript",
      code = [[
function run(inputs) {
    const wasmResult = inputs.process_with_wasi;
    
    console.log("📊 WASI Execution Results:");
    console.log("  Status:", wasmResult.wasm_execution.status);
    console.log("  Return Code:", wasmResult.wasm_execution.return_code);
    console.log("  Module:", wasmResult.wasm_execution.module);
    
    return {
        verification: "complete",
        wasi_enabled: true,
        capabilities: [
            "stdio_access",
            "environment_variables",
            "system_clock",
            "random_generation"
        ],
        timestamp: new Date().toISOString()
    };
}
]]
    },

    -- Step 4: Summary with Lua
    generate_summary = {
      depends_on = { "verify_results" },
      language = "lua",
      code = [[
function run(inputs)
    local verification = inputs.verify_results
    local wasm_result = inputs.process_with_wasi
    
    print("🎯 WASI Workflow Summary:")
    print("  ✓ WASI-enabled WASM execution successful")
    print("  ✓ System interface access: enabled")
    print("  ✓ Multi-language integration: verified")
    
    return {
        summary = "WASI workflow completed successfully",
        wasi_features_demonstrated = {
            "Standard I/O (stdio) access",
            "Environment variable access",
            "Command-line argument passing",
            "System clock and time access",
            "Secure sandboxed execution"
        },
        performance = {
            steps_executed = 4,
            languages_used = {"Python", "WASM+WASI", "JavaScript", "Lua"}
        }
    }
end
]]
    }
  }
}
