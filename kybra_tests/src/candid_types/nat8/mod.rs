// KYBRA_CASES=1 cargo test candid_types::nat8::property_tests::basic -- --nocapture --exact

#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use std::error::Error;

    use crate::candid_types::create_arb_program; // TODO this is foolishness
    use crate::candid_types::run_candid_types_property_tests;
    use crate::candid_types::ArbParam;

    #[test]
    fn basic() -> Result<(), Box<dyn Error>> {
        run_candid_types_property_tests(
            create_arb_program(
                // TODO we probably don't need to pass in this create_arb_program...at all
                "from kybra import nat8, query, update".to_string(),
                &create_arb_nat8(),
                params_return_value_getter,
                &prop_oneof!["nat8"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/nat8",
        )
    }

    fn create_arb_nat8() -> impl Strategy<Value = u8> {
        // Not using any::<u8>() because adding some u8s together later can get too large
        // So I started with the largest u8 and divided by 5, since we only add up 5 params for now
        0..51u8
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<u8>>) -> u8 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value)
    }

    fn no_params_return_value_getter(return_value: u8) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<u8>) -> String {
        format!("{}: nat8", arb_param.value)
    }

    fn assertion(result: u8, expected: u8) {
        assert!(result == expected);
    }
}
