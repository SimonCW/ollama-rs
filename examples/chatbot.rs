use std::io::stdin;

use ollama_rs::{Ollama, generation::completion::{GenerationContext, request::GenerationRequest, GenerationResponseStream}};
use tokio::io::{stdout, AsyncWriteExt,};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let mut stdout = stdout();
    let stdin = stdin();

    let mut context: Option<GenerationContext> = None;

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        
        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let mut request = GenerationRequest::new("llama2:latest".into(), input.to_string());
        if let Some(context) = context.clone() {
            request = request.context(context);
        }
        let mut stream: GenerationResponseStream = ollama.generate_stream(request).await?;

        while let Some(Ok(res)) = stream.next().await {
            stdout.write_all(res.response.as_bytes()).await?;
            stdout.flush().await?;

            if let Some(final_data) = res.final_data {
                context = Some(final_data.context);
            }
        }
    }

    Ok(())
}