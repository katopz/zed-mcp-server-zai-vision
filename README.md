# ZAI Vision MCP Server for Zed

This extension integrates [z.ai Vision](https://z.ai) as a Model Context Protocol (MCP) server for Zed's Assistant, providing image analysis, video understanding, and other visual capabilities powered by GLM-4.6V.

## What is z.ai Vision?

z.ai Vision gives your AI Agent visual understanding — analyze screenshots, interpret diagrams, extract text, diagnose errors, and more.

### ✅ With Vision

- Image analysis and content understanding
- Screenshot OCR and text extraction
- Error diagnosis from screenshots
- Technical diagram interpretation
- Data visualization insights
- UI comparison and diff checking
- Video content analysis (local/remote ≤8 MB; MP4/MOV/M4V)

## Installation

This extension can be installed from the Zed extension registry.

**Prerequisites:** [Node.js >= v22.0.0](https://nodejs.org/en/download/)

## Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the ZAI Vision MCP server. If you see that the status of the tool is a red dot, make sure you toggle it so that becomes green.
3. Enable the ZAI Vision MCP Server in the active assistant profile. In the chat section, click on the `Write | Ask` button, then click on `tools`, then enable the ZAI Vision MCP Server.

## API Key Configuration

You need a z.ai API key to use this extension.

Add your API key in the extension settings:

```json
{
  "context_server": {
    "mcp-server-zai-vision": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "zai_api_key": "YOUR_ZAI_API_KEY"
      }
    }
  }
}
```

## Usage

**Best practice:** Place images in a local directory and reference them by filename or path in your conversation.

- `What does demo.png describe?`
- `Extract the text from screenshot.png`
- `Diagnose the error in error.png`
- `Explain the architecture diagram in diagram.png`
- `What insights can you get from chart.png?`
- `Compare ui_v1.png with ui_v2.png`
- `Analyze the video at demo.mp4`

## Available Tools

The ZAI Vision MCP Server provides these tools to the LLM:

- **`ui_to_artifact`** — Turn UI screenshots into code, prompts, specs, or descriptions
- **`extract_text_from_screenshot`** — OCR screenshots for code, terminals, docs, and general text
- **`diagnose_error_screenshot`** — Analyze error snapshots and propose actionable fixes
- **`understand_technical_diagram`** — Interpret architecture, flow, UML, ER, and system diagrams
- **`analyze_data_visualization`** — Read charts and dashboards to surface insights and trends
- **`ui_diff_check`** — Compare two UI shots to flag visual or implementation drift
- **`analyze_image`** — General-purpose image understanding
- **`analyze_video`** — Inspect videos to describe scenes, moments, and entities

## How It Works

The extension installs the [`@z_ai/mcp-server`](https://www.npmjs.com/package/@z_ai/mcp-server) npm package and runs it locally as a stdio MCP server, passing your API key via the `Z_AI_API_KEY` environment variable.

## Development

Clone the project and build:

```bash
cargo build
```

## License

MIT