use serde_derive::{Deserialize, Serialize};

use sewup::types::Address;
use sewup_derive::Value;
use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Value)]
struct Puzzle {
    hint: String,
    word: String,
    reward: String,
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
fn set_puzzle(puzzle: Puzzle) -> Result<(), &'static str> {
    let caller_address = sewup::utils::caller();

    let mut storage = sewup::kv::Store::load(None).map_err(|_| "fail to load storage")?;
    let mut puzzle_bucket = storage
        .bucket::<Address, Puzzle>("puzzles")
        .map_err(|_| "fail to load puzzles")?;
    puzzle_bucket.set(caller_address, puzzle);
    storage.save(puzzle_bucket);
    storage.commit().map_err(|_| "fail to store puzzle")?;
    Ok(())
}

#[ewasm_main(rusty)]
fn main() -> Result<(), &'static str> {
    use sewup::primitives::Contract;
    use sewup_derive::ewasm_input_from;

    let contract = Contract::new().map_err(|_| "NewContractError")?;
    match contract
        .get_function_selector()
        .map_err(|_| "FailGetFnSelector")?
    {
        ewasm_fn_sig!(set_puzzle) => {
            ewasm_input_from!(contract move set_puzzle, |_| "Fail to set puzzle")
        }
        _ => return Err("UnknownHandle"),
    };

    Ok(())
}

#[ewasm_test]
mod tests {
    use super::*;
    use sewup::primitives::Contract;
    use sewup_derive::{ewasm_assert_eq, ewasm_rusty_assert_ok, ewasm_rusty_err_output};

    #[ewasm_test]
    fn test_set_puzzle() {
        let puzzle = Puzzle {
            hint: "A fruit".into(),
            word: "Apple".into(),
            reward: "You are the apple of my eye".into(),
        };
        ewasm_rusty_assert_ok!(
            set_puzzle(puzzle) by "1cCA28600d7491365520B31b466f88647B9839eC"
        );
    }
}
