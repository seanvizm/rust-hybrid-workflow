use mlua::{Lua, Table};

#[derive(Clone)]
pub struct Step {
    pub name: String,
    pub language: String,
    pub code: String,
    pub depends_on: Vec<String>,
}

pub fn load_workflow(path: &str) -> anyhow::Result<Vec<Step>> {
    let lua = Lua::new();
    let script = std::fs::read_to_string(path)?;
    lua.load(&script).exec()?;

    let globals = lua.globals();
    let workflow: Table = globals.get("workflow")?;
    let steps: Table = workflow.get("steps")?;

    let mut result = vec![];

    for pair in steps.pairs::<String, Table>() {
        let (name, step) = pair?;
        
        // Default to "lua" if language is not specified
        let language: String = step.get("language").unwrap_or_else(|_| "lua".to_string());
        
        // For Lua steps, extract the function and convert to code string
        let code: String = if language == "lua" {
            // For pure Lua workflows, we need to handle the run function differently
            // For now, we'll create a placeholder - this will need more sophisticated handling
            format!("-- Lua function for step: {}", name)
        } else {
            step.get("code")?
        };
        
        let depends_on: Option<Vec<String>> = step.get("depends_on").ok();

        result.push(Step {
            name,
            language,
            code,
            depends_on: depends_on.unwrap_or_default(),
        });
    }

    Ok(result)
}
