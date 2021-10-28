use sewup_derive::{ewasm_constructor, ewasm_fn, ewasm_fn_sig, ewasm_main, ewasm_test};

#[ewasm_constructor]
fn constructor() {}

#[ewasm_fn]
fn handler() -> Result<(), &'static str> {
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
        ewasm_fn_sig!(handler) => handler()?,
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
    fn test_handler_ok() {
        ewasm_rusty_assert_ok!(handler());
    }
}
