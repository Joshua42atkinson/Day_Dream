// ═══════════════════════════════════════════════════════════════════════════════
// DAY_DREAM — YouTube Video Generation Pipeline
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        narrative/mod.rs
// PURPOSE:     AI narration generation for YouTube video content
//
// ARCHITECTURE:
//   • Genre-aware narration generation for educational content
//   • 30-second per slide pacing control
//   • Tone and depth adaptation based on content complexity
//   • Integration with LongCat for artistic image generation
//
// DEPENDENCIES:
//   - trinity_protocol — Genre enum, VAAM components
//   - serde — Serialization
//   - tracing — Logging
//
// CHANGES:
//   2026-05-26  Migrated from TRINITY narrative.rs for YouTube pipeline
//   2026-03-16  Cascade  Created for 30-second core loop (original)
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use trinity_protocol::Genre;

// TODO: Define Day_Dream-specific content stages instead of HeroStage
// For YouTube pipeline, we need content stages like: Introduction, Explanation, Example, Conclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentStage {
    Introduction,
    Explanation,
    Example,
    Analysis,
    Conclusion,
}

// TODO: Define Day_Dream-specific phases instead of Phase
// For YouTube pipeline, we need phases like: Planning, Scripting, Generation, Assembly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelinePhase {
    Planning,
    Scripting,
    Generation,
    Assembly,
    Publishing,
}

/// Narrative context for YouTube video generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeContext {
    /// Content genre (educational, storytelling, tutorial, etc.)
    pub genre: Genre,
    /// Current content stage (Introduction, Explanation, Example, etc.)
    pub content_stage: ContentStage,
    /// Current pipeline phase (Planning, Scripting, Generation, etc.)
    pub pipeline_phase: PipelinePhase,
    /// Video topic/theme
    pub topic: String,
    /// Target audience description
    pub target_audience: String,
    /// Content complexity level (0.0 = simple, 1.0 = advanced)
    pub complexity: f32,
    /// Desired video duration per slide (seconds)
    pub slide_duration: u32,
    /// Total number of slides planned
    pub total_slides: u32,
    /// Current slide number
    pub current_slide: u32,
    /// Content style/tone preference
    pub style_preference: Option<String>,
    /// Key learning objectives
    pub learning_objectives: Vec<String>,
    /// Cognitive load target (from VAAM)
    pub cognitive_load_target: f32,
}

impl Default for NarrativeContext {
    fn default() -> Self {
        Self {
            genre: Genre::Cyberpunk,
            content_stage: ContentStage::Introduction,
            pipeline_phase: PipelinePhase::Planning,
            topic: "Introduction to AI".to_string(),
            target_audience: "General audience interested in technology".to_string(),
            complexity: 0.5,
            slide_duration: 30,
            total_slides: 5,
            current_slide: 1,
            style_preference: None,
            learning_objectives: vec![],
            cognitive_load_target: 0.5,
        }
    }
}

/// Generated narration script for YouTube video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrationScript {
    /// Unique identifier
    pub id: String,
    /// Timestamp
    pub timestamp: String,
    /// Script type (intro, content, conclusion, etc.)
    pub script_type: String,
    /// Narration content (30-second script)
    pub content: String,
    /// Genre
    pub genre: Genre,
    /// Slide number
    pub slide_number: u32,
    /// Total slides in video
    pub total_slides: u32,
    /// Estimated duration (seconds)
    pub duration: u32,
    /// Suggested image prompt for LongCat
    pub image_prompt: String,
}

/// Complexity-based tone directive for YouTube narration.
/// Adapts narration style based on content complexity and target audience.
pub fn complexity_tone_guide(complexity: f32) -> &'static str {
    match (complexity * 100.0) as u32 {
        0..=20 => {
            "TONE: INTRODUCTORY LEVEL. Use simple language, clear explanations, \
             and relatable examples. Avoid jargon. Focus on building foundational understanding. \
             Speak slowly and clearly. Use analogies and metaphors."
        }
        21..=50 => {
            "TONE: INTERMEDIATE LEVEL. Balance technical accuracy with accessibility. \
             Introduce key terminology but explain it clearly. Use practical examples. \
             Maintain engaging pace with clear transitions between concepts."
        }
        51..=75 => {
            "TONE: ADVANCED LEVEL. Assume audience familiarity with basics. \
             Dive deeper into technical details and nuances. Use industry terminology appropriately. \
             Challenge viewers with thought-provoking insights and connections."
        }
        _ => {
            "TONE: EXPERT LEVEL. Full technical depth with minimal simplification. \
             Address advanced concepts and cutting-edge developments. \
             Encourage critical thinking and analysis. Suitable for professional audiences."
        }
    }
}

/// Content depth directive for YouTube narration.
/// Controls how detailed the explanation should be based on content stage and complexity.
pub fn content_depth_directive(
    content_stage: ContentStage,
    complexity: f32,
    slide_number: u32,
    total_slides: u32,
) -> &'static str {
    let is_intro = slide_number == 1;
    let is_conclusion = slide_number == total_slides;
    let mid_content = !is_intro && !is_conclusion;

    if is_conclusion {
        // Conclusion: Summarize and call to action
        "DEPTH: CONCLUSION. Summarize key points from the video. \
         Reinforce main takeaways. Provide clear call-to-action (subscribe, like, comment). \
         End with forward-looking statement or teaser for next content."
    } else if is_intro {
        // Introduction: Hook and overview
        "DEPTH: INTRODUCTION. Create compelling hook in first 5 seconds. \
         Preview what viewers will learn. Establish credibility and relevance. \
         Keep it concise and engaging. Preview the structure of the content."
    } else if mid_content && complexity > 0.6 {
        // Mid-content with high complexity: Deep dive
        "DEPTH: DEEP DIVE. Provide detailed explanations with examples. \
         Use visual descriptions to support complex concepts. \
         Include step-by-step breakdowns when appropriate. \
         Balance depth with clarity to maintain viewer engagement."
    } else {
        // Standard mid-content: Balanced explanation
        "DEPTH: BALANCED EXPLANATION. Clear, concise explanations. \
         Use examples and analogies to illustrate concepts. \
         Maintain 30-second pacing per slide. \
         Include transitions between ideas for smooth flow."
    }
}


/// Content stage-aware music mood prompt for video background music.
/// Maps each content stage to appropriate background music style.
pub fn video_mood_prompt(content_stage: ContentStage, genre: Genre) -> (&'static str, &'static str) {
    // Returns (style, prompt)
    match content_stage {
        ContentStage::Introduction => (
            "upbeat",
            match genre {
                Genre::Cyberpunk => "futuristic electronic intro, synthesizer buildup, energetic anticipation, digital atmosphere, modern and engaging",
                Genre::Fantasy => "mystical orchestral intro, gentle strings building, magical atmosphere, wonder and discovery, enchanting beginning",
                Genre::SciFi => "space ambient intro, subtle electronic pulses, cosmic atmosphere, scientific curiosity, exploration theme",
                _ => "engaging instrumental intro, building energy, positive atmosphere, welcoming and professional, clear introduction",
            }
        ),
        ContentStage::Explanation => (
            "focused",
            match genre {
                Genre::Cyberpunk => "steady electronic rhythm, focused concentration, technological atmosphere, clear and precise, modern educational",
                Genre::Fantasy => "gentle orchestral background, storytelling atmosphere, warm and inviting, narrative support, magical learning",
                Genre::SciFi => "ambient electronic background, scientific atmosphere, calm and focused, space exploration, educational ambient",
                _ => "calm instrumental background, focused learning atmosphere, clear and supportive, professional education, concentration music",
            }
        ),
        ContentStage::Example => (
            "dynamic",
            match genre {
                Genre::Cyberpunk => "rhythmic electronic example, dynamic demonstration, technological showcase, engaging and clear, modern example",
                Genre::Fantasy => "storytelling orchestral, narrative example, magical demonstration, engaging tale, fantasy illustration",
                Genre::SciFi => "futuristic example, scientific demonstration, space technology showcase, clear and engaging, sci-fi example",
                _ => "dynamic instrumental, clear example presentation, engaging demonstration, practical illustration, educational example",
            }
        ),
        ContentStage::Analysis => (
            "thoughtful",
            match genre {
                Genre::Cyberpunk => "analytical electronic, deep thinking atmosphere, technological analysis, complex patterns, modern investigation",
                Genre::Fantasy => "mysterious orchestral, deep analysis atmosphere, magical investigation, complex patterns, fantasy analysis",
                Genre::SciFi => "scientific ambient, deep space analysis, cosmic investigation, complex patterns, sci-fi analysis",
                _ => "thoughtful instrumental, deep analysis atmosphere, complex examination, educational investigation, analytical music",
            }
        ),
        ContentStage::Conclusion => (
            "inspiring",
            match genre {
                Genre::Cyberpunk => "triumphant electronic finish, achievement unlocked, technological victory, inspiring conclusion, modern success",
                Genre::Fantasy => "epic orchestral finish, heroic achievement, magical victory, inspiring conclusion, fantasy success",
                Genre::SciFi => "cosmic triumph finish, scientific achievement, space victory, inspiring conclusion, sci-fi success",
                _ => "inspiring instrumental finish, achievement unlocked, educational success, positive conclusion, professional completion",
            }
        ),
    }
}
            "visionary",
            "ethereal synths mixed with acoustic instruments, third eye opening, seeing beyond the tracks, dreamlike yet grounded, future-gazing"
        ),
        Phase::Yoke => (
            "epic",
            "full orchestral crescendo, all themes united, connective tissue binding movement to movement, yoked together, powerful and cohesive"
        ),
        Phase::Evolve => (
            "triumphant",
            "triumphant fanfare, celebration at the final station, brass fanfare with orchestral swells, arrival, achievement, the Iron Road complete, first breath of new air"
        ),
    }
}

/// Genre-specific style guide for YouTube content generation
pub fn genre_style_guide(genre: Genre) -> &'static str {
    match genre {
        Genre::Cyberpunk =>
            "Futuristic aesthetic with neon accents, digital overlays, and technological themes. \
             Use modern terminology, references to AI and computing, and forward-looking perspectives. \
             Visual style: Dark backgrounds with bright neon highlights, holographic elements, and sleek interfaces.",
        Genre::Steampunk =>
            "Victorian industrial aesthetic with brass gears, steam power, and mechanical innovation. \
             Use historical metaphors mixed with technology, references to craftsmanship and engineering. \
             Visual style: Warm metallic tones, clockwork mechanisms, steam effects, and vintage industrial design.",
        Genre::Solarpunk =>
            "Optimistic future aesthetic with renewable energy, green technology, and sustainable living. \
             Use themes of harmony between nature and technology, abundance, and environmental solutions. \
             Visual style: Bright natural lighting, green architecture, solar panels, wind turbines, and organic integration.",
        Genre::DarkFantasy =>
            "Mysterious and atmospheric aesthetic with gothic elements, ancient knowledge, and hidden secrets. \
             Use themes of discovery, ancient wisdom, and the unknown. Maintain an educational tone while being engaging. \
             Visual style: Dramatic lighting, ancient architecture, mystical elements, and rich textures.",
    }
}

/// Content stage description for YouTube video structure
pub fn content_stage_description(stage: ContentStage) -> &'static str {
    match stage {
        ContentStage::Introduction => {
            "Introduction Stage — Hook the viewer in the first 5 seconds, preview the content, \
             establish relevance, and set expectations for what they'll learn."
        }
        ContentStage::Explanation => {
            "Explanation Stage — Clear, concise explanations of the core concepts. \
             Use examples, analogies, and visual descriptions to support understanding."
        }
        ContentStage::Example => {
            "Example Stage — Demonstrate the concept with practical examples. \
             Show real-world applications, walk through step-by-step processes, \
             and make abstract ideas concrete."
        }
        ContentStage::Analysis => {
            "Analysis Stage — Deeper examination of the topic. \
             Explore connections, implications, and critical thinking aspects. \
             Challenge viewers to engage with the material more deeply."
        }
        ContentStage::Conclusion => {
            "Conclusion Stage — Summarize key points, reinforce learning, \
             provide clear call-to-action, and end with forward-looking statement."
        }
    }
}

/// Build the YouTube content generation system prompt
pub fn build_narrative_system_prompt(context: &NarrativeContext) -> String {
    let style = genre_style_guide(context.genre);
    let stage = content_stage_description(context.content_stage);
    let tone = complexity_tone_guide(context.complexity);
    let depth = content_depth_directive(context.content_stage, context.complexity, context.current_slide, context.total_slides);

    format!("You are a YouTube content generator specializing in educational videos.

GENRE: {:?}
{}

CURRENT CONTENT STAGE:
{}

TONE DIRECTIVE:
{}

DEPTH DIRECTIVE:
{}

VIDEO PARAMETERS:
- Topic: {}
- Target Audience: {}
- Slide Duration: {} seconds
- Current Slide: {} of {}
- Complexity Level: {:.1}

Generate a 30-second narration script for this slide. Include:
1. Engaging opening hook
2. Clear explanation of the key concept
3. Visual description for LongCat image generation
4. Smooth transition to next slide

Keep the script exactly 30 seconds when spoken at normal pace.",
        context.genre, style, stage, tone, depth,
        context.topic, context.target_audience, context.slide_duration,
        context.current_slide, context.total_slides, context.complexity
    )
}
{}

PLAYER CHARACTER SHEET:
- Name: {}
{}
- Coal (energy reserves): {:.1}
- Steam (momentum): {:.1}  

}

/// Build the LongCat image generation prompt for YouTube slides
pub fn build_image_prompt(context: &NarrativeContext, script_content: &str) -> String {
    let style = genre_style_guide(context.genre);
    let mood = video_mood_prompt(context.content_stage, context.genre);

    format!("Generate a high-quality artistic image for a YouTube educational video.

GENRE STYLE:
{}
CONTENT STAGE: {:?}
MOOD: {}

SCRIPT CONTEXT:
{}

IMAGE REQUIREMENTS:
- Style: Consistent with genre aesthetic
- Mood: Match the content stage atmosphere
- Quality: High resolution suitable for 1080p video
- Composition: Balanced for text overlay space
- Colors: Appropriate for the genre and mood

Create an image that visually represents the key concept from the script while maintaining artistic consistency with the video's overall style.",
        style, context.content_stage, mood.1, script_content
    )
}

/// Generate narration script using LLM
pub async fn generate_narration(llm_url: &str, context: &NarrativeContext) -> Option<String> {
    let system_prompt = build_narrative_system_prompt(context);
    
    // TODO: Implement LLM call using inference module
    // For now, return a placeholder
    Some(format!("30-second narration for slide {} of {} about: {}", 
        context.current_slide, context.total_slides, context.topic))
}

/// Create narration script entry
pub fn create_narration_entry(content: String, context: &NarrativeContext) -> NarrationScript {
    NarrationScript {
        id: format!("slide_{}", context.current_slide),
        timestamp: chrono::Utc::now().to_rfc3339(),
        script_type: format!("{:?}", context.content_stage),
        content,
        genre: context.genre.clone(),
        slide_number: context.current_slide,
        total_slides: context.total_slides,
        duration: context.slide_duration,
        image_prompt: build_image_prompt(context, &content),
    }
}

// TODO: Implement remaining functions for YouTube pipeline
// - generate_failure_narrative()
// - generate_success_narrative() 
// - generate_critical_narrative()
// - generate_fumble_narrative()
// These will be adapted for YouTube content generation context

/// Generate a failure/error message for YouTube pipeline
pub fn generate_failure_narrative(context: &NarrativeContext, failure_reason: &str) -> String {
    format!(
        "Content generation failed at slide {} of {}. Error: {}. \
         Please check the pipeline logs and retry.",
        context.current_slide, context.total_slides, failure_reason
    )
}

/// Generate a success message for YouTube pipeline completion
pub fn generate_success_narrative(
    context: &NarrativeContext,
    slides_generated: u32,
    total_duration: u32,
) -> String {
    format!(
        "Successfully generated {} slides ({} seconds total) for topic: '{}'. \
         Video ready for assembly and YouTube upload.",
        slides_generated, total_duration, context.topic
    )
}

/// Generate a critical success message for YouTube pipeline
pub fn generate_critical_narrative(context: &NarrativeContext) -> String {
    format!(
        "OUTSTANDING CONTENT GENERATED! Video '{}' achieved exceptional quality metrics. \
         High engagement potential detected. Ready for immediate YouTube upload.",
        context.topic
    )
}

/// Generate a fumble/error message for YouTube pipeline
pub fn generate_fumble_narrative(context: &NarrativeContext) -> String {
    format!(
        "CONTENT GENERATION FAILED for topic '{}' at slide {}. \
         Pipeline error detected. Please check system resources and LongCat availability.",
        context.topic, context.current_slide
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genre_style_guide() {
        let cyber = genre_style_guide(Genre::Cyberpunk);
        assert!(cyber.to_lowercase().contains("futuristic"));

        let steam = genre_style_guide(Genre::Steampunk);
        assert!(steam.to_lowercase().contains("victorian"));
    }

    #[test]
    fn test_content_stage_description() {
        let desc = content_stage_description(ContentStage::Introduction);
        assert!(desc.contains("Introduction"));
        assert!(desc.contains("hook"));
    }

    #[test]
    fn test_complexity_tone_guide() {
        let simple = complexity_tone_guide(0.1);
        assert!(simple.contains("INTRODUCTORY"));

        let advanced = complexity_tone_guide(0.8);
        assert!(advanced.contains("ADVANCED"));
    }

    #[test]
    fn test_narrative_context_default() {
        let ctx = NarrativeContext::default();
        assert_eq!(ctx.slide_duration, 30);
        assert_eq!(ctx.current_slide, 1);
        assert_eq!(ctx.total_slides, 5);
    }

    #[test]
    fn test_build_narrative_system_prompt() {
        let ctx = NarrativeContext {
            topic: "Test Topic".to_string(),
            ..Default::default()
        };
        let prompt = build_narrative_system_prompt(&ctx);
        assert!(prompt.contains("Test Topic"));
        assert!(prompt.contains("30-second"));
    }
}
