use crate::error::Result;
use crate::llm::LLMClient;
use crate::metadata::TrackMetadata;
use crate::suggestions::{MetadataSuggestion, SuggestionsReport};

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

    /// Analyze track and generate structured suggestions
    pub async fn analyze_with_suggestions(&self, metadata: &TrackMetadata) -> Result<SuggestionsReport> {
        println!("🔍 Analyzing track with {}...", self.llm.provider_name());
        
        // Build a more structured prompt for suggestions
        let observation = self.observe_for_suggestions(metadata);
        let llm_response = self.think(&observation).await?;
        
        // Parse LLM response to extract suggestions
        let suggestions = self.parse_suggestions(&llm_response, metadata);
        
        let report = SuggestionsReport::new(
            metadata.file_path.clone(),
            metadata.clone(),
            suggestions,
            llm_response,
        );
        
        Ok(report)
    }

    /// Build a structured prompt that asks for specific suggestions
    fn observe_for_suggestions(&self, metadata: &TrackMetadata) -> String {
        let system_prompt = r#"You are a music metadata expert. Analyze the MP3 metadata and provide structured suggestions.

For each field that needs correction, respond in this EXACT format:

SUGGESTION: [field_name]
CURRENT: [current value or "None"]
SUGGESTED: [your suggested value]
CONFIDENCE: [High/Medium/Low]
REASON: [brief explanation]
---

Available fields: artist, title, album, year, genre, album_artist, track_number

Only suggest changes for fields that are missing, incorrect, or could be improved.
If metadata is complete and accurate, respond with: "NO_SUGGESTIONS_NEEDED"

After all suggestions, provide a brief OVERALL_ASSESSMENT."#;

        format!("{}\n\n{}", system_prompt, metadata.to_prompt_format())
    }

    /// Parse LLM response into structured suggestions
    fn parse_suggestions(&self, response: &str, _metadata: &TrackMetadata) -> Vec<MetadataSuggestion> {
        let mut suggestions = Vec::new();
        
        if response.contains("NO_SUGGESTIONS_NEEDED") {
            return suggestions;
        }
        
        // Split by suggestion blocks
        let blocks: Vec<&str> = response.split("---").collect();
        
        for block in blocks {
            if block.trim().is_empty() || !block.contains("SUGGESTION:") {
                continue;
            }
            
            let lines: Vec<&str> = block.lines().collect();
            let mut field = String::new();
            let mut current = None;
            let mut suggested = String::new();
            let mut confidence = String::from("Medium");
            let mut reason = String::new();
            
            for line in lines {
                let line = line.trim();
                if line.starts_with("SUGGESTION:") {
                    field = line.replace("SUGGESTION:", "").trim().to_string();
                } else if line.starts_with("CURRENT:") {
                    let val = line.replace("CURRENT:", "").trim().to_string();
                    if val != "None" && !val.is_empty() {
                        current = Some(val);
                    }
                } else if line.starts_with("SUGGESTED:") {
                    suggested = line.replace("SUGGESTED:", "").trim().to_string();
                } else if line.starts_with("CONFIDENCE:") {
                    confidence = line.replace("CONFIDENCE:", "").trim().to_string();
                } else if line.starts_with("REASON:") {
                    reason = line.replace("REASON:", "").trim().to_string();
                }
            }
            
            if !field.is_empty() && !suggested.is_empty() {
                suggestions.push(MetadataSuggestion {
                    field,
                    current_value: current,
                    suggested_value: suggested,
                    confidence,
                    reason,
                });
            }
        }
        
        suggestions
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
