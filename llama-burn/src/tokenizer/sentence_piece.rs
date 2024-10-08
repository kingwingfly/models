use tokenizers::Tokenizer as BaseTokenizer;

use super::Tokenizer;

const BOS_TOKEN_ID: u32 = 1;
const EOS_TOKEN_ID: u32 = 2;

pub struct SentiencePieceTokenizer {
    bpe: BaseTokenizer,
    bos_token_id: u32,
    eos_token_id: u32,
}

impl Tokenizer for SentiencePieceTokenizer {
    /// Load the [SentenciePiece](https://github.com/google/sentencepiece) tokenizer.
    fn new(tokenizer_path: &str) -> Result<Self, String> {
        let bpe = BaseTokenizer::from_file(tokenizer_path).map_err(|e| e.to_string())?;

        Ok(Self {
            bpe,
            bos_token_id: BOS_TOKEN_ID,
            eos_token_id: EOS_TOKEN_ID,
        })
    }

    fn encode(&self, text: &str, bos: bool, eos: bool) -> Vec<u32> {
        let bos_token = if bos { vec![self.bos_token_id] } else { vec![] };
        let eos_token = if eos { vec![self.eos_token_id] } else { vec![] };

        let tokens = self.bpe.encode(text, false).unwrap().get_ids().to_vec();

        [bos_token, tokens, eos_token]
            .into_iter()
            .flat_map(|t| t.into_iter())
            .collect()
    }

    fn decode(&self, tokens: Vec<u32>) -> String {
        self.bpe
            .decode(
                &tokens.into_iter().map(|t| t as u32).collect::<Vec<u32>>(),
                true,
            )
            .unwrap()
    }

    fn bos_id(&self) -> u32 {
        self.bos_token_id
    }

    fn eos_id(&self) -> u32 {
        self.eos_token_id
    }

    fn stop_ids(&self) -> Vec<u32> {
        vec![self.eos_id()]
    }
}
