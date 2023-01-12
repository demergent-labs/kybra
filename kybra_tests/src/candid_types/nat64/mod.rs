// KYBRA_CASES=1 cargo test candid_types::nat64::property_tests::basic -- --nocapture --exact

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
                "from kybra import nat64, query, update".to_string(),
                &create_arb_nat(),
                params_return_value_getter,
                &prop_oneof!["nat64"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/nat64",
        )
    }

    fn create_arb_nat() -> impl Strategy<Value = u64> {
        // Not using any::<u64>() because adding some u64s together later can get too large
        // So I started with the largest u64 and divided by 5, since we only add up 5 params for now
        0..1_844_674_407_370_955_161u64
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<u64>>) -> u64 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value)
    }

    fn no_params_return_value_getter(return_value: u64) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<u64>) -> String {
        format!("{}: nat64", arb_param.value)
    }

    fn assertion(result: u64, expected: u64) {
        assert!(result == expected);
    }
}
