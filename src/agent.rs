use crate::error::Result;
use crate::llm::LLMClient;
use crate::metadata::TrackMetadata;

pub struct MusicAgent {
    llm: Box<dyn LLMClient>,
}

impl MusicAgent {
    pub fn new(llm: Box<dyn LLMClient>) -> Self {
        Self { llm }
    }

    /// Main agent workflow: Observe → Think → Report
    pub async fn analyze_track(&self, metadata: &TrackMetadata) -> Result<AnalysisReport> {
        println!("🔍 Analyzing track with {}...", self.llm.provider_name());

        // Step 1: Observe - Build context from metadata
        let observation = self.observe(metadata);

        // Step 2: Think - Send to LLM for analysis
        let llm_response = self.think(&observation).await?;

        // Step 3: Report - Structure the results
        let report = AnalysisReport {
            metadata: metadata.clone(),
            analysis: llm_response,
            has_issues: metadata.has_missing_critical_fields(),
        };

        Ok(report)
    }

    /// Observe: Prepare metadata for LLM analysis
    fn observe(&self, metadata: &TrackMetadata) -> String {
        let system_prompt = r#"You are a music metadata expert. Analyze the provided MP3 file metadata and provide:

1. **Assessment**: Evaluate the quality and completeness of the metadata
2. **Issues**: Identify any missing, incorrect, or suspicious data
3. **Suggestions**: Recommend specific corrections or improvements
4. **Confidence**: Rate your confidence in the current metadata (Low/Medium/High)

Be concise but thorough. Focus on actionable insights."#;

        format!("{}\n\n{}", system_prompt, metadata.to_prompt_format())
    }

    /// Think: Send observation to LLM for reasoning
    async fn think(&self, observation: &str) -> Result<String> {
        self.llm.generate(observation).await
    }
}

/// Structured analysis report from the agent
#[derive(Debug)]
pub struct AnalysisReport {
    pub metadata: TrackMetadata,
    pub analysis: String,
    pub has_issues: bool,
}

impl AnalysisReport {
    pub fn display(&self) {
        println!("\n{}", "=".repeat(62));
        println!("📊 ANALYSIS REPORT");
        println!("{}\n", "=".repeat(62));

        println!("{}\n", self.metadata);

        println!("🤖 AI Analysis:");
        println!("{}", "-".repeat(62));
        println!("{}", self.analysis);
        println!("{}\n", "-".repeat(62));

        if self.has_issues {
            println!("⚠️  Issues detected - review suggestions above");
        } else {
            println!("✅ Metadata appears complete");
        }
    }
}
