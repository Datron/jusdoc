#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use anyhow::Ok;
use candle::{quantized::gguf_file, Device, Tensor};
use candle_transformers::models::quantized_llama as model;
use candle_transformers::{generation::LogitsProcessor, models::quantized_llama::ModelWeights};
use std::{fs::File, io::Write, path::PathBuf};
use tokenizers::Tokenizer;

fn print_token(next_token: u32, tokenizer: &Tokenizer) {
    // Extracting the last token as a string is complicated, here we just apply some simple
    // heuristics as it seems to work well enough for this example. See the following for more
    // details:
    // https://github.com/huggingface/tokenizers/issues/1141#issuecomment-1562644141
    if let Some(text) = tokenizer.id_to_token(next_token) {
        let text = text.replace('▁', " ");
        let ascii = text
            .strip_prefix("<0x")
            .and_then(|t| t.strip_suffix('>'))
            .and_then(|t| u8::from_str_radix(t, 16).ok());
        match ascii {
            None => print!("{text}"),
            Some(ascii) => {
                if let Some(chr) = char::from_u32(ascii as u32) {
                    if chr.is_ascii() {
                        print!("{chr}")
                    }
                }
            }
        }
        let _ = std::io::stdout().flush();
    }
}


const MODEL_PATH: &str = "models/llama-2-7b-chat.Q4_K_M.gguf";
const TOKENIZER_PATH: &str = "tokenizer.json";
fn main() -> anyhow::Result<()> {
    let mut model_file = File::open(&MODEL_PATH)?;
    let model = gguf_file::Content::read(&mut model_file)?;
    let mut model = ModelWeights::from_gguf(model, &mut model_file)?;
    println!("model ready!");

    let tokenizer = PathBuf::from(TOKENIZER_PATH);
    let tokenizer = Tokenizer::from_file(tokenizer).map_err(anyhow::Error::msg)?;
    println!("Tokenizer ready!");
    let prompt = String::from("give me a recipe for mac and cheese");
    let pre_prompt_tokens = vec![];
    let tokens = tokenizer
        .encode(prompt.as_str(), true)
        .map_err(anyhow::Error::msg)?;

    let prompt_tokens = [&pre_prompt_tokens, tokens.get_ids()].concat();
    let prompt_tokens = if prompt_tokens.len() + 250 > model::MAX_SEQ_LEN - 10 {
        let to_remove = prompt_tokens.len() + 250 + 10 - model::MAX_SEQ_LEN;
        prompt_tokens[prompt_tokens.len().saturating_sub(to_remove)..].to_vec()
    } else {
        prompt_tokens
    };

    let mut logits_processor = LogitsProcessor::new(299792458, Some(0.8), None);
    let mut all_tokens = vec![];
    let mut next_token = {
        let input = Tensor::new(prompt_tokens.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
        let logits = model.forward(&input, 0)?;
        let logits = logits.squeeze(0)?;
        logits_processor.sample(&logits)?
    };
    all_tokens.push(next_token);
    print_token(next_token, &tokenizer);

    for index in 0..500 {
        let input = Tensor::new(&[next_token], &Device::Cpu)?.unsqueeze(0)?;
        let logits = model.forward(&input, prompt_tokens.len() + index)?;
        let logits = logits.squeeze(0)?;
        // let logits = if args.repeat_penalty == 1. {
        //     logits
        // } else {
        let start_at = all_tokens.len().saturating_sub(64);
        let logits = candle_transformers::utils::apply_repeat_penalty(
            &logits,
            1.1,
            &all_tokens[start_at..],
        )?;
        // };
        next_token = logits_processor.sample(&logits)?;
        all_tokens.push(next_token);
        print_token(next_token, &tokenizer);
    }

    Ok(())
}
