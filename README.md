# YART Example

This project demonstrates how to use the `yart` crate to create tools with the `rig::tool::Tool` trait and integrate them with a `GeminiClient` agent from `rig-core`. It includes two example tools built with the `#[rig_tool]` macro:

- **Calculator Tool**: Performs basic arithmetic operations (`add` or `subtract`) on two numbers.
- **Greeter Tool**: Greets a user by name in a configured language (English `en` or Spanish `es`).

## Prerequisites

- Rust (stable, edition 2021)
- Cargo
- A Gemini API key (set as `GEMINI_API_KEY` environment variable)

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/pupplecat/yart-example.git
cd yart-example
```

Copy `.env.sample` to `.env` and set your Gemini API key:

```bash
cp .env.sample .env
echo "GEMINI_API_KEY=your-api-key" > .env
```

Replace `your-api-key` with your actual Gemini API key.

## Usage

The project implements two tools integrated with a `GeminiClient` agent:

- **Calculator Tool**: Takes an operation (`add` or `subtract`) and two numbers, returning the result formatted to 2 decimal places.
- **Greeter Tool**: Greets a user in the language specified by the application context (`en` for English, `es` for Spanish).

Run the example to test the agent with predefined prompts:

```bash
cargo run
```

Expected output:

```text
Prompt: Add 5 and 3
Agent Response: The result is 8.00
Prompt: Subtract 10 from 15
Agent Response: The result of subtracting 10 from 15 is 5.00.
Prompt: Greet Alice in Spanish
Agent Response: ¡Hola, Alice!
Prompt: Say hello to Bob in English
Agent Response: Hello, Bob!
Prompt: Multiply 4 by 2
Agent Response: I am sorry, I cannot perform multiplication. I can only perform addition and subtraction. Would you like to add or subtract these numbers?
```

## Project Structure

```text
yart-example/
├── Cargo.toml          # Project metadata and dependencies
├── src/
│   └── main.rs         # Implementation of Calculator and Greeter tools with agent integration
├── .env.sample         # Sample environment file for GEMINI_API_KEY
└── README.md           # This file
```

## Code Walkthrough

- **Calculator Tool**:
  - Defined with `#[rig_tool(description = "Performs basic arithmetic operations")]`.
  - Accepts `CalcArgs` (operation, two numbers) and returns `CalcOutput` (result).
  - Handles invalid operations with `ToolError`.
- **Greeter Tool**:
  - Defined with `#[rig_tool(name = "greeter", description = "Greets a user in the configured language")]`.
  - Uses `AppContext` to store language preference (`en` or `es`).
  - Demonstrates context passing with `Arc`.
- **Agent Integration**:
  - Uses `GeminiClient` with a preamble to guide tool usage.
  - Tests prompts like "Add 5 and 3" and "Greet Alice in Spanish".
  - Includes tracing for debugging (set to `INFO` level).

See `src/main.rs` for the full implementation.

## Dependencies

- `yart`: Provides the `#[rig_tool]` macro and utilities (`ToolError`, `ToolOutput`).
- `rig-core`: Provides `GeminiClient` and agent functionality.
- `serde`: For serialization/deserialization of inputs and outputs.
- `schemars`: For generating JSON schemas for tool arguments.
- `tokio`: For async runtime support.
- `dotenvy`: For loading environment variables from `.env`.
- `tracing-subscriber`: For logging agent interactions.

## Contributing

Contributions are welcome! Submit pull requests or open issues on the [project repository](https://github.com/pupplecat/yart-example).

## License

MIT

## Learn More

- YART Crate: [crates.io/crates/yart](https://crates.io/crates/yart)
- YART Repository: [github.com/pupplecat/yart](https://github.com/pupplecat/yart)
- YART Documentation: [docs.rs/yart](https://docs.rs/yart/0.1.0/yart)
