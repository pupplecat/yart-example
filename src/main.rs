use rig::{
    agent::PromptRequest,
    providers::gemini::{completion::GEMINI_2_0_FLASH, Client as GeminiClient},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use yart::ToolError;

#[derive(Clone)]
struct AppContext {
    language: String, // e.g., "en" for English, "es" for Spanish
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CalcArgs {
    #[schemars(description = "Mathematical operation: available 'add', 'subtract'")]
    operation: String, // "add" or "subtract"
    a: f64,
    b: f64,
}

#[derive(Deserialize, Serialize)]
pub struct CalcOutput {
    result: f64,
}

#[yart::rig_tool(description = "Performs basic arithmetic operations,")]
async fn calculator(args: CalcArgs) -> anyhow::Result<CalcOutput, ToolError> {
    let result = match args.operation.as_str() {
        "add" => args.a + args.b,
        "subtract" => args.a - args.b,
        _ => {
            return Err(ToolError::new(
                "Invalid operation: must be 'add' or 'subtract'",
            ))
        }
    };
    Ok(CalcOutput { result })
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GreetArgs {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GreetOutput {
    message: String,
}

#[yart::rig_tool(
    name = "greeter",
    description = "Greets a user in the configured language"
)]
async fn greeter(ctx: Arc<AppContext>, args: GreetArgs) -> anyhow::Result<GreetOutput, ToolError> {
    let message = match ctx.language.as_str() {
        "en" => format!("Hello, {}!", args.name),
        "es" => format!("¡Hola, {}!", args.name),
        _ => return Err(ToolError::new("Unsupported language")),
    };
    Ok(GreetOutput { message })
}

// Setup function to initialize tracing only once
pub fn init_tracing() {
    // Use a static Once to ensure initialization happens once
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .init();
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing, set to DEBUG to see more detail
    init_tracing();
    // Load environment variable.
    dotenvy::dotenv().ok();
    // Initialize Gemini client
    let gemini_client = GeminiClient::from_env();

    // Create agent with Calculator and Greeter tools
    let ctx = Arc::new(AppContext {
        language: "es".to_string(),
    });
    let agent = gemini_client
        .agent(GEMINI_2_0_FLASH)
        .preamble(
            "You are an assistant that helps with basic arithmetic and personalized greetings. \
             Use the `Calculator` tool for operations like 'add' or 'subtract', formatting results to 2 decimal places. \
             Use the `Greeter` tool to greet users in the configured language (English 'en' or Spanish 'es'). \
             If a prompt is unclear or a tool fails, provide a clear error message and suggest valid inputs."
        )
        .max_tokens(1048)
        .tool(Calculator::new())
        .tool(Greeter::new(ctx))
        .build();

    // Test prompts
    let prompts = vec![
        "Add 5 and 3",
        "Subtract 10 from 15",
        "Greet Alice in Spanish",
        "Say hello to Bob in English",
        "Multiply 4 by 2",
    ];

    for prompt in prompts {
        println!("Prompt: {}", prompt);
        let ret = PromptRequest::new(&agent, prompt).multi_turn(5).await?;
        println!("Agent Response: {}", ret);
    }

    Ok(())
}
