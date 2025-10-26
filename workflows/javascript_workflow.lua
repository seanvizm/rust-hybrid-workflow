-- javascript_workflow.lua

workflow = {
  name = "javascript_data_pipeline",
  description = "Workflow demonstrating JavaScript/Node.js integration",

  steps = {
    generate_data = {
      language = "javascript",
      code = [[
function run() {
    console.log("Generating sample data with JavaScript...");
    
    // Generate some sample data
    const data = [];
    for (let i = 1; i <= 10; i++) {
        data.push({
            id: i,
            value: Math.random() * 100,
            timestamp: new Date().toISOString()
        });
    }
    
    return {
        dataset: data,
        count: data.length,
        generated_at: new Date().toISOString()
    };
}
]]
    },

    process_with_js = {
      depends_on = { "generate_data" },
      language = "javascript",
      code = [[
function run(inputs) {
    console.log("Processing data with JavaScript...");
    
    const dataset = inputs.generate_data.dataset;
    
    // Calculate statistics
    const values = dataset.map(item => item.value);
    const sum = values.reduce((a, b) => a + b, 0);
    const average = sum / values.length;
    const max = Math.max(...values);
    const min = Math.min(...values);
    
    // Process and filter data
    const processedData = dataset
        .filter(item => item.value > average)
        .map(item => ({
            ...item,
            normalized: item.value / max,
            above_average: true
        }));
    
    return {
        statistics: {
            count: dataset.length,
            sum: sum,
            average: average,
            max: max,
            min: min
        },
        processed_items: processedData,
        above_average_count: processedData.length
    };
}
]]
    },

    format_results = {
      depends_on = { "process_with_js" },
      language = "javascript",
      code = [[
function run(inputs) {
    console.log("Formatting final results with JavaScript...");
    
    const stats = inputs.process_with_js.statistics;
    const processed = inputs.process_with_js.processed_items;
    
    // Create a formatted report
    const report = {
        title: "JavaScript Data Processing Report",
        summary: {
            total_records: stats.count,
            records_above_average: processed.length,
            percentage_above_average: ((processed.length / stats.count) * 100).toFixed(2) + "%"
        },
        statistics: {
            average: Math.round(stats.average * 100) / 100,
            max: Math.round(stats.max * 100) / 100,
            min: Math.round(stats.min * 100) / 100
        },
        top_items: processed
            .sort((a, b) => b.value - a.value)
            .slice(0, 3)
            .map(item => ({
                id: item.id,
                value: Math.round(item.value * 100) / 100,
                normalized: Math.round(item.normalized * 1000) / 1000
            }))
    };
    
    console.log("\\n=== JAVASCRIPT PROCESSING REPORT ===");
    console.log(JSON.stringify(report, null, 2));
    console.log("=====================================\\n");
    
    return {
        report: report,
        status: "completed",
        processed_at: new Date().toISOString()
    };
}
]]
    }
  }
}