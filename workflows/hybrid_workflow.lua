-- hybrid_workflow.lua

workflow = {
  name = "hybrid_data_pipeline",
  description = "Workflow with embedded Python code",

  steps = {
    fetch_data = {
      language = "python",
      code = [[
def run():
    print("Fetching data...")
    return {"data": [1, 2, 3, 4]}
]]
    },

    process_data = {
      depends_on = { "fetch_data" },
      language = "python",
      code = [[
def run(inputs):
    raw = inputs["fetch_data"]["data"]
    processed = [x * 2 for x in raw]
    return {"processed": processed}
]]
    },

    store_data = {
      depends_on = { "process_data" },
      language = "python",
      code = [[
def run(inputs):
    final = inputs["process_data"]["processed"]
    print("Storing:", final)
    return {"status": "done"}
]]
    }
  }
}
