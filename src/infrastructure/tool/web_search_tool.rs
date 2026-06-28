use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::domain::error::tool_error::ToolError;
use crate::domain::port::tool::Tool;

const TAVILY_SEARCH_URL: &str = "https://api.tavily.com/search";

#[derive(Debug, Serialize, Deserialize)]
struct WebSearchArguments {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_depth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TavilySearchResponse {
    query: String,
    results: Vec<TavilySearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TavilySearchResult {
    title: String,
    url: String,
    content: String,
}

#[derive(Debug, Clone)]
pub struct WebSearchTool {
    api_key: String,
    client: Client,
}

impl WebSearchTool {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Find web pages with Tavily. Returns titles, URLs, and content excerpts for choosing sources."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "What to look up on the web. Add names, dates, or site hints when they matter."
                },
                "max_results": {
                    "type": "integer",
                    "description": "Result limit. Default: 5. Maximum: 20.",
                    "minimum": 1,
                    "maximum": 20
                },
                "search_depth": {
                    "type": "string",
                    "enum": ["basic", "advanced"],
                    "description": "basic is faster; advanced spends more work on harder research."
                },
                "topic": {
                    "type": "string",
                    "enum": ["general", "news", "finance"],
                    "description": "Result category. Default: general."
                },
                "time_range": {
                    "type": "string",
                    "enum": ["day", "week", "month", "year"],
                    "description": "Prefer pages published or updated in this recent window."
                },
            },
            "required": ["query"],
            "additionalProperties": false
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, ToolError> {
        let request_body = serde_json::from_value::<WebSearchArguments>(arguments)
            .map_err(|err| ToolError::InvalidArguments(err.to_string()))?;

        let response = self
            .client
            .post(TAVILY_SEARCH_URL)
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|err| ToolError::ExecutionFailed(err.to_string()))?;

        if !response.status().is_success() {
            return Err(ToolError::ExecutionFailed(format!(
                "Tavily API returned status code {}",
                response.status()
            )));
        }

        let response_body = response
            .json::<TavilySearchResponse>()
            .await
            .map_err(|err| ToolError::ExecutionFailed(err.to_string()))?;

        let results = serde_json::to_value(&response_body)
            .map_err(|err| ToolError::ExecutionFailed(err.to_string()))?;

        Ok(results)
    }
}
