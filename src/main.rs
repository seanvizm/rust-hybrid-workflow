mod engine;
mod lua_loader;
mod lua_runner;
mod python_runner;

fn main() -> anyhow::Result<()> {
    println!("=== Running hybrid workflow (Python + Lua) ===");
    engine::run_workflow("workflows/hybrid_workflow.lua")?;
    
    println!("\n=== Running pure Lua workflow ===");
    engine::run_workflow("workflows/workflow.lua")?;
    
    Ok(())
}
