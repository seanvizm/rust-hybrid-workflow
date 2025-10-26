-- workflow.lua

workflow = {
  name = "data_pipeline",
  description = "A sample workflow to fetch, process, and store data",

  steps = {
    fetch_data = {
      run = function()
        print("Fetching data...")
        return { data = {1, 2, 3, 4, 5} }
      end
    },

    process_data = {
      depends_on = { "fetch_data" },
      run = function(inputs)
        local raw = inputs["fetch_data"].data
        local processed = {}
        for i, v in ipairs(raw) do
          processed[i] = v * 2
        end
        return { processed = processed }
      end
    },

    store_data = {
      depends_on = { "process_data" },
      run = function(inputs)
        local final = inputs["process_data"].processed
        print("Storing data:", table.concat(final, ", "))
        return { status = "success" }
      end
    }
  }
}
