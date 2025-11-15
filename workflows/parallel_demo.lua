-- Parallel Execution Demo Workflow
-- This workflow demonstrates parallel execution capabilities
-- with independent steps that can run concurrently

workflow = {
  name = "parallel_demo",
  description = "Demonstrates parallel execution with 4 independent tasks",

  steps = {
    init = {
      language = "lua",
      code = [[
function run()
  print("🚀 Initializing parallel workflow...")
  return "initialized"
end
      ]]
    },
    
    task_a = {
      depends_on = {"init"},
      language = "shell",
      code = [[
echo "Task A: Starting data processing..."
sleep 1
echo "Task A: Processing complete"
echo "result_a"
      ]]
    },
    
    task_b = {
      depends_on = {"init"},
      language = "python",
      code = [[
def run(inputs=None):
    print("Task B: Starting calculations...")
    # Simulate work without external modules
    total = sum(range(1000000))
    print("Task B: Calculations complete")
    return "result_b"
      ]]
    },
    
    task_c = {
      depends_on = {"init"},
      language = "lua",
      code = [[
function run()
  print("Task C: Starting validation...")
  -- Simulate work
  local start = os.clock()
  while os.clock() - start < 1 do end
  print("Task C: Validation complete")
  return "result_c"
end
      ]]
    },
    
    task_d = {
      depends_on = {"init"},
      language = "shell",
      code = [[
echo "Task D: Starting transformation..."
sleep 1
echo "Task D: Transformation complete"
echo "result_d"
      ]]
    },
    
    merge_results = {
      depends_on = {"task_a", "task_b", "task_c", "task_d"},
      language = "lua",
      code = [[
function run(inputs)
  print("📊 Merging results from all tasks...")
  print("Received inputs: ")
  for k, v in pairs(inputs) do
    print("  " .. k .. ": " .. tostring(v))
  end
  return "all_tasks_completed"
end
      ]]
    },
    
    finalize = {
      depends_on = {"merge_results"},
      language = "shell",
      code = [[
echo "✅ Workflow finalized successfully"
echo "All parallel tasks completed"
      ]]
    }
  }
}
