use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

use sewup::{primitives::EwasmAny, types::Address};
use sewup_derive::Value;
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Value)]
struct Puzzle {
    hint: String,
    word: String,
    reward: String,
}

#[derive(Default, Clone, Debug, Serialize)]
struct PuzzleInfo {
    hint: String,
    size: usize,
}

impl std::convert::From<Puzzle> for PuzzleInfo {
    fn from(puzzle: Puzzle) -> Self {
        let Puzzle { hint, word, .. } = puzzle;
        PuzzleInfo {
            hint,
            size: word.len(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Input {
    address: String,
}

#[ewasm_constructor]
fn constructor() {
    let mut storage =
        sewup::kv::Store::new().expect("there is no return for constructor currently");
    let puzzle_bucket = storage
        .bucket::<Address, Puzzle>("puzzles")
        .expect("there is no return for constructor currently");

    storage.save(puzzle_bucket);
    storage
        .commit()
        .expect("there is no return for constructor currently");
}

#[ewasm_fn]
fn set_puzzle(puzzle: Puzzle) -> Result<EwasmAny> {
    let caller_address = sewup::utils::caller();

    let mut storage = sewup::kv::Store::load(None)?;
    let mut puzzle_bucket = storage.bucket::<Address, Puzzle>("puzzles")?;
    puzzle_bucket.set(caller_address, puzzle);
    storage.save(puzzle_bucket);
    storage.commit()?;
    Ok(().into())
}

#[ewasm_fn]
fn get_puzzle_info(input: Input) -> Result<EwasmAny> {
    let mut storage = sewup::kv::Store::load(None)?;
    let mut puzzle_bucket = storage.bucket::<Address, Puzzle>("puzzles")?;
    let address = Address::from_str(&input.address)?;
    return if let Some(p) = puzzle_bucket.get(address)? {
        let info: PuzzleInfo = p.into();
        Ok(info.into())
    } else {
        Err(anyhow::anyhow!("There is no puzzle in this address"))
    };
}

#[ewasm_main(auto)]
fn main() -> Result<EwasmAny> {
    use sewup::primitives::Contract;
    use sewup_derive::ewasm_input_from;

    let contract = Contract::new()?;
    match contract.get_function_selector()? {
        ewasm_fn_sig!(set_puzzle) => {
            ewasm_input_from!(contract move set_puzzle)
        }
        ewasm_fn_sig!(get_puzzle_info) => return ewasm_input_from!(contract move get_puzzle_info),
        _ => return Err(anyhow::anyhow!("UnknownHandle")),
    };

    Ok(().into())
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup::primitives::Contract;
    use sewup_derive::{ewasm_assert_ok, ewasm_auto_assert_eq};

    #[ewasm_test]
    fn test_set_and_solve_puzzle() {
        let puzzle = Puzzle {
            hint: "A fruit".into(),
            word: "Apple".into(),
            reward: "You are the apple of my eye".into(),
        };

        ewasm_assert_ok!(
            set_puzzle(puzzle) by "1cCA28600d7491365520B31b466f88647B9839eC"
        );

        let input = Input {
            address: "0x1cCA28600d7491365520B31b466f88647B9839eC".to_string(),
        };

        let info = PuzzleInfo {
            hint: "A fruit".into(),
            size: 5,
        };
        ewasm_auto_assert_eq!(get_puzzle_info(input), info);
    }
}
