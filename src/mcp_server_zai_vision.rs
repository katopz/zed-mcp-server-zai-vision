use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@z_ai/mcp-server";
const PACKAGE_VERSION: &str = "latest";

#[derive(Debug, Deserialize, JsonSchema)]
struct ZaiVisionMcpExtensionSettings {
    /// The API key for z.ai vision service.
    #[serde(default)]
    zai_api_key: Option<String>,
}

struct ZaiVisionMcpExtension;

impl zed::Extension for ZaiVisionMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-zai-vision", project)?;

        let settings_struct: ZaiVisionMcpExtensionSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|e| format!("{e}"))?,
            None => ZaiVisionMcpExtensionSettings { zai_api_key: None },
        };

        let server_path = format!(
            "{}/node_modules/@z_ai/mcp-server/build/index.js",
            std::env::current_dir()
                .map_err(|e| format!("{e}"))?
                .to_string_lossy()
        );

        let mut env = vec![("Z_AI_MODE".to_string(), "ZAI".to_string())];

        if let Some(api_key) = settings_struct.zai_api_key {
            if !api_key.is_empty() {
                env.push(("Z_AI_API_KEY".to_string(), api_key));
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("mcp-server-zai-vision", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(vision_settings) =
                    serde_json::from_value::<ZaiVisionMcpExtensionSettings>(settings_value)
                {
                    match vision_settings.zai_api_key {
                        Some(zai_api_key) => {
                            default_settings = default_settings
                                .replace("\"YOUR_ZAI_API_KEY\"", &format!("\"{}\"", zai_api_key));
                        }
                        None => {
                            default_settings =
                                default_settings.replace("\"YOUR_ZAI_API_KEY\"", "\"\"");
                        }
                    }
                }
            }
        }

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ZaiVisionMcpExtensionSettings))
                .map_err(|e| format!("{e}"))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ZaiVisionMcpExtension);
