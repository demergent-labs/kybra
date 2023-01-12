// KYBRA_CASES=1 cargo test candid_types::nat32::property_tests::basic -- --nocapture --exact

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
                "from kybra import nat32, query, update".to_string(),
                &create_arb_nat(),
                params_return_value_getter,
                &prop_oneof!["nat32"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/nat32",
        )
    }

    fn create_arb_nat() -> impl Strategy<Value = u32> {
        // Not using any::<u32>() because adding some u32s together later can get too large
        // So I started with the largest u32 and divided by ten, since we only add up 5 params for now
        0..858_993_459u32
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<u32>>) -> u32 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value)
    }

    fn no_params_return_value_getter(return_value: u32) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<u32>) -> String {
        format!("{}: nat32", arb_param.value)
    }

    fn assertion(result: u32, expected: u32) {
        assert!(result == expected);
    }
}
