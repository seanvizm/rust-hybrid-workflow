use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration with support for external config files and environment variables.
/// 
/// Configuration precedence (highest to lowest):
/// 1. Environment variables (HWFE_*)
/// 2. Config file (config.toml, config.json, or config.yaml)
/// 3. Default values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Workflow configuration
    pub workflows: WorkflowConfig,
    
    /// Server configuration for web UI
    pub server: ServerConfig,
    
    /// Runner-specific configurations
    pub runners: RunnerConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    /// Directory to search for workflow files
    #[serde(default = "default_workflow_dir")]
    pub directory: PathBuf,
    
    /// File extensions to consider as workflows
    #[serde(default = "default_workflow_extensions")]
    pub extensions: Vec<String>,
    
    /// Maximum number of workflows to load
    #[serde(default = "default_max_workflows")]
    pub max_workflows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host address
    #[serde(default = "default_server_host")]
    pub host: String,
    
    /// Server port
    #[serde(default = "default_server_port")]
    pub port: u16,
    
    /// Static files directory for web UI
    #[serde(default = "default_static_dir")]
    pub static_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    /// Python configuration
    pub python: PythonConfig,
    
    /// JavaScript configuration
    pub javascript: JavaScriptConfig,
    
    /// Shell configuration
    pub shell: ShellConfig,
    
    /// WASM configuration
    pub wasm: WasmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    /// Python interpreter path (default: "python3")
    #[serde(default = "default_python_interpreter")]
    pub interpreter: String,
    
    /// Enable Python runner
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaScriptConfig {
    /// Node.js interpreter path (default: "node")
    #[serde(default = "default_node_interpreter")]
    pub interpreter: String,
    
    /// Enable JavaScript runner
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    /// Shell interpreter path (default: "sh")
    #[serde(default = "default_shell_interpreter")]
    pub interpreter: String,
    
    /// Enable shell runner
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    /// WASM modules directory
    #[serde(default = "default_wasm_modules_dir")]
    pub modules_dir: PathBuf,
    
    /// Enable WASI support
    #[serde(default = "default_false")]
    pub wasi_enabled: bool,
    
    /// Enable WASM runner
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
    
    /// Enable colored output
    #[serde(default = "default_true")]
    pub colored: bool,
}

// Default value functions
fn default_workflow_dir() -> PathBuf {
    PathBuf::from("workflows")
}

fn default_workflow_extensions() -> Vec<String> {
    vec!["lua".to_string()]
}

fn default_max_workflows() -> usize {
    100
}

fn default_server_host() -> String {
    "127.0.0.1".to_string()
}

fn default_server_port() -> u16 {
    3030
}

fn default_static_dir() -> PathBuf {
    PathBuf::from("pkg")
}

fn default_python_interpreter() -> String {
    "python3".to_string()
}

fn default_node_interpreter() -> String {
    "node".to_string()
}

fn default_shell_interpreter() -> String {
    "sh".to_string()
}

fn default_wasm_modules_dir() -> PathBuf {
    PathBuf::from("wasm_modules/target/wasm32-unknown-unknown/release")
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            workflows: WorkflowConfig {
                directory: default_workflow_dir(),
                extensions: default_workflow_extensions(),
                max_workflows: default_max_workflows(),
            },
            server: ServerConfig {
                host: default_server_host(),
                port: default_server_port(),
                static_dir: default_static_dir(),
            },
            runners: RunnerConfig {
                python: PythonConfig {
                    interpreter: default_python_interpreter(),
                    enabled: default_true(),
                },
                javascript: JavaScriptConfig {
                    interpreter: default_node_interpreter(),
                    enabled: default_true(),
                },
                shell: ShellConfig {
                    interpreter: default_shell_interpreter(),
                    enabled: default_true(),
                },
                wasm: WasmConfig {
                    modules_dir: default_wasm_modules_dir(),
                    wasi_enabled: default_false(),
                    enabled: default_true(),
                },
            },
            logging: LoggingConfig {
                level: default_log_level(),
                colored: default_true(),
            },
        }
    }
}

impl AppConfig {
    /// Load configuration with the following precedence:
    /// 1. Environment variables (HWFE_*)
    /// 2. Config file (config.toml, config.json, or config.yaml)
    /// 3. Default values
    pub fn load() -> Result<Self> {
        // Start with default config
        let mut config = Self::default();
        
        // Try to load .env file (silently ignore if not found)
        #[cfg(feature = "cli")]
        {
            let _ = dotenvy::dotenv();
        }
        
        // Try to load config file (in order of preference: TOML, JSON, YAML)
        #[cfg(feature = "cli")]
        {
            if let Ok(file_config) = Self::load_from_file() {
                config = file_config;
            }
        }
        
        // Override with environment variables
        config.apply_env_overrides()?;
        
        Ok(config)
    }
    
    /// Load configuration from file (config.toml, config.json, or config.yaml)
    #[cfg(feature = "cli")]
    fn load_from_file() -> Result<Self> {
        use config::{Config, File, FileFormat};
        
        let builder = Config::builder();
        
        // Try to load config file in order of preference
        let builder = if PathBuf::from("config.toml").exists() {
            builder.add_source(File::new("config", FileFormat::Toml))
        } else if PathBuf::from("config.json").exists() {
            builder.add_source(File::new("config", FileFormat::Json))
        } else if PathBuf::from("config.yaml").exists() || PathBuf::from("config.yml").exists() {
            builder.add_source(File::new("config", FileFormat::Yaml))
        } else {
            return Err(anyhow::anyhow!("No config file found"));
        };
        
        let settings = builder.build()
            .context("Failed to build config")?;
        
        settings.try_deserialize()
            .context("Failed to deserialize config")
    }
    
    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) -> Result<()> {
        use std::env;
        
        // Workflow configuration
        if let Ok(val) = env::var("HWFE_WORKFLOW_DIR") {
            self.workflows.directory = PathBuf::from(val);
        }
        if let Ok(val) = env::var("HWFE_WORKFLOW_EXTENSIONS") {
            self.workflows.extensions = val.split(',').map(String::from).collect();
        }
        if let Ok(val) = env::var("HWFE_WORKFLOW_MAX") {
            self.workflows.max_workflows = val.parse()
                .context("Invalid HWFE_WORKFLOW_MAX value")?;
        }
        
        // Server configuration
        if let Ok(val) = env::var("HWFE_SERVER_HOST") {
            self.server.host = val;
        }
        if let Ok(val) = env::var("HWFE_SERVER_PORT") {
            self.server.port = val.parse()
                .context("Invalid HWFE_SERVER_PORT value")?;
        }
        if let Ok(val) = env::var("HWFE_STATIC_DIR") {
            self.server.static_dir = PathBuf::from(val);
        }
        
        // Python configuration
        if let Ok(val) = env::var("HWFE_PYTHON_INTERPRETER") {
            self.runners.python.interpreter = val;
        }
        if let Ok(val) = env::var("HWFE_PYTHON_ENABLED") {
            self.runners.python.enabled = val.parse()
                .context("Invalid HWFE_PYTHON_ENABLED value")?;
        }
        
        // JavaScript configuration
        if let Ok(val) = env::var("HWFE_JS_INTERPRETER") {
            self.runners.javascript.interpreter = val;
        }
        if let Ok(val) = env::var("HWFE_JS_ENABLED") {
            self.runners.javascript.enabled = val.parse()
                .context("Invalid HWFE_JS_ENABLED value")?;
        }
        
        // Shell configuration
        if let Ok(val) = env::var("HWFE_SHELL_INTERPRETER") {
            self.runners.shell.interpreter = val;
        }
        if let Ok(val) = env::var("HWFE_SHELL_ENABLED") {
            self.runners.shell.enabled = val.parse()
                .context("Invalid HWFE_SHELL_ENABLED value")?;
        }
        
        // WASM configuration
        if let Ok(val) = env::var("HWFE_WASM_MODULES_DIR") {
            self.runners.wasm.modules_dir = PathBuf::from(val);
        }
        if let Ok(val) = env::var("HWFE_WASM_WASI_ENABLED") {
            self.runners.wasm.wasi_enabled = val.parse()
                .context("Invalid HWFE_WASM_WASI_ENABLED value")?;
        }
        if let Ok(val) = env::var("HWFE_WASM_ENABLED") {
            self.runners.wasm.enabled = val.parse()
                .context("Invalid HWFE_WASM_ENABLED value")?;
        }
        
        // Logging configuration
        if let Ok(val) = env::var("HWFE_LOG_LEVEL") {
            self.logging.level = val;
        }
        if let Ok(val) = env::var("HWFE_LOG_COLORED") {
            self.logging.colored = val.parse()
                .context("Invalid HWFE_LOG_COLORED value")?;
        }
        
        Ok(())
    }
    
    /// Save current configuration to a TOML file
    #[cfg(feature = "cli")]
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let toml_str = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;
        
        std::fs::write(path, toml_str)
            .context("Failed to write config file")?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.workflows.directory, PathBuf::from("workflows"));
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3030);
        assert!(config.runners.python.enabled);
        assert!(!config.runners.wasm.wasi_enabled);
    }
    
    #[test]
    fn test_env_override() {
        // In Edition 2024, set_var and remove_var are unsafe
        unsafe {
            std::env::set_var("HWFE_SERVER_PORT", "8080");
            std::env::set_var("HWFE_WORKFLOW_DIR", "custom_workflows");
        }
        
        let mut config = AppConfig::default();
        config.apply_env_overrides().unwrap();
        
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.workflows.directory, PathBuf::from("custom_workflows"));
        
        // Clean up
        unsafe {
            std::env::remove_var("HWFE_SERVER_PORT");
            std::env::remove_var("HWFE_WORKFLOW_DIR");
        }
    }
}
