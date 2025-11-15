# Parallel Execution

The Hybrid Workflow Engine supports parallel execution of independent workflow steps, significantly improving performance for workflows with multiple concurrent tasks.

## Overview

Parallel execution allows independent workflow steps to run concurrently instead of sequentially. The engine automatically analyzes step dependencies and groups them into execution levels, where all steps in a level can run in parallel.

### Benefits

- **Faster Execution**: Independent steps run concurrently, reducing total workflow time
- **Efficient Resource Usage**: Better CPU utilization when running multiple steps
- **Automatic Dependency Management**: Engine handles dependency resolution and scheduling
- **Safe by Default**: Sequential mode prevents race conditions in critical workflows

## Configuration

### Config File

Add an `[execution]` section to your `config.toml`:

```toml
[execution]
# Execution mode: "sequential" (default) or "parallel"
mode = "parallel"

# Maximum number of steps to execute in parallel
# 0 = auto-detect CPU count (default: 4)
max_parallel_steps = 4

# Enable step-level parallelism (future feature)
enable_step_parallelism = false
```

### Environment Variables

Override configuration with environment variables:

```bash
# Set execution mode
export HWFE_EXECUTION_MODE=parallel

# Set max parallel steps
export HWFE_MAX_PARALLEL_STEPS=8

# Enable step parallelism
export HWFE_ENABLE_PARALLELISM=true
```

### Command-Line Usage

```bash
# Use parallel mode with environment variable
HWFE_EXECUTION_MODE=parallel cargo run --features cli workflows/my_workflow.lua

# Use config file
cargo run --features cli workflows/my_workflow.lua
```

## How It Works

### Dependency Level Grouping

The parallel execution engine analyzes your workflow and groups steps by dependency level:

```
Level 1: [init]                    ← Runs first (no dependencies)
Level 2: [task_a, task_b, task_c]  ← Run in parallel (all depend on init)
Level 3: [merge]                   ← Runs after level 2 (depends on all tasks)
Level 4: [finalize]                ← Runs last (depends on merge)
```

### Execution Process

1. **Dependency Analysis**: Engine calculates dependency levels for all steps
2. **Level-by-Level Execution**: Each level executes before moving to the next
3. **Concurrent Execution**: Steps within a level run in parallel (up to max_parallel_steps)
4. **Result Propagation**: Outputs from completed steps become inputs for dependent steps

### Circular Dependency Detection

The engine detects and reports circular dependencies during the grouping phase:

```
Error: Circular dependency detected involving step 'step_a'
```

## Example Workflows

### Parallel Demo Workflow

See `workflows/parallel_demo.lua` for a complete example:

```lua
workflow = {
  name = "parallel_demo",
  description = "Demonstrates parallel execution",

  steps = {
    init = {
      language = "lua",
      code = [[
function run()
  print("🚀 Initializing...")
  return "initialized"
end
      ]]
    },
    
    -- These 4 tasks run in parallel (all depend only on init)
    task_a = {
      depends_on = {"init"},
      language = "shell",
      code = "echo 'Task A' && sleep 1 && echo 'result_a'"
    },
    
    task_b = {
      depends_on = {"init"},
      language = "python",
      code = [[
def run(inputs=None):
    print("Task B")
    return "result_b"
      ]]
    },
    
    task_c = {
      depends_on = {"init"},
      language = "lua",
      code = [[
function run()
  print("Task C")
  return "result_c"
end
      ]]
    },
    
    task_d = {
      depends_on = {"init"},
      language = "shell",
      code = "echo 'Task D' && sleep 1 && echo 'result_d'"
    },
    
    -- Merge runs after all tasks complete
    merge = {
      depends_on = {"task_a", "task_b", "task_c", "task_d"},
      language = "lua",
      code = [[
function run(inputs)
  print("Merging results...")
  for k, v in pairs(inputs) do
    print("  " .. k .. ": " .. tostring(v))
  end
  return "merged"
end
      ]]
    }
  }
}
```

### Performance Comparison

For the parallel demo workflow above (4 independent 1-second tasks):

- **Sequential Mode**: ~4.4 seconds
- **Parallel Mode**: ~2.3 seconds
- **Speedup**: 1.93x (almost 2x faster)

## Best Practices

### When to Use Parallel Mode

✅ **Good use cases:**
- Data processing pipelines with independent tasks
- Multi-stage validation workflows
- Fan-out/fan-in patterns (one task spawns many, then merges)
- I/O-bound workflows (file processing, API calls)

❌ **Avoid for:**
- Workflows with strict ordering requirements
- Steps that modify shared state
- Very fast workflows (overhead may outweigh benefits)
- When debugging (sequential is easier to trace)

### Designing for Parallelism

1. **Minimize Dependencies**: Reduce unnecessary dependencies between steps
2. **Group Related Work**: Steps at the same dependency level run together
3. **Balance Work**: Try to distribute work evenly across parallel steps
4. **Handle Failures**: Parallel execution stops if any step fails

### Resource Limits

The `max_parallel_steps` setting controls concurrency:

```toml
[execution]
# Conservative (good for limited resources)
max_parallel_steps = 2

# Moderate (balanced)
max_parallel_steps = 4

# Aggressive (high-performance systems)
max_parallel_steps = 8

# Auto-detect (uses CPU count)
max_parallel_steps = 0
```

## Technical Details

### Implementation

- **Runtime**: Tokio async runtime for concurrent execution
- **Synchronization**: Arc<RwLock<>> for shared state access
- **Concurrency Control**: Semaphore limits concurrent tasks
- **Error Handling**: Any step failure terminates the entire level

### Thread Safety

All language runners are thread-safe:
- **Lua**: Each step gets its own Lua VM instance
- **Python**: Protected by Python GIL and PyO3
- **Shell**: Separate process per execution
- **JavaScript**: Separate Boa context per step
- **WASM**: Isolated WASM instances

### Performance Characteristics

- **Overhead**: ~50-100ms for dependency analysis and setup
- **Scalability**: Linear speedup up to CPU core count
- **Memory**: Each concurrent step allocates its own runtime
- **I/O**: Particularly effective for I/O-bound workloads

## Troubleshooting

### Parallel Mode Not Activating

Check your configuration:

```bash
# Verify config is loaded
cargo run --features cli workflows/test.lua 2>&1 | grep "Execution mode"
# Should show: Execution mode: parallel

# Test with environment variable
HWFE_EXECUTION_MODE=parallel cargo run --features cli workflows/test.lua
```

### Unexpected Execution Order

Remember that within a level, execution order is non-deterministic:

```
Level 2: [task_a, task_b, task_c]
# These may complete in ANY order
```

If order matters, add dependencies:

```lua
task_b = {
  depends_on = {"init", "task_a"},  -- Now runs after task_a
  ...
}
```

### Performance Not Improving

Common issues:
1. **CPU-bound work on single-core machine**: Parallel mode needs multiple cores
2. **Very short tasks**: Overhead may exceed task duration
3. **Sequential dependencies**: Check dependency graph
4. **max_parallel_steps too low**: Increase the limit

### Debugging Parallel Execution

Use sequential mode for easier debugging:

```bash
# Temporarily switch to sequential
HWFE_EXECUTION_MODE=sequential cargo run --features cli workflows/test.lua
```

## Future Enhancements

Planned improvements for parallel execution:

- **Step-level parallelism**: Allow individual steps to spawn parallel operations
- **Dynamic scheduling**: Adaptive concurrency based on system load
- **Distributed execution**: Run steps across multiple machines
- **Priority scheduling**: Execute critical steps first
- **Resource quotas**: CPU/memory limits per step

## See Also

- [Configuration Management](CONFIGURATION.md) - Full configuration reference
- [Testing Guide](TESTING.md) - Testing parallel workflows
- [README](../README.md) - General project overview
