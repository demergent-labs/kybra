// KYBRA_CASES=1 cargo test candid_types::nat16::property_tests::basic -- --nocapture --exact

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
                "from kybra import nat16, query, update".to_string(),
                &create_arb_nat(),
                params_return_value_getter,
                &prop_oneof!["nat16"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/nat16",
        )
    }

    fn create_arb_nat() -> impl Strategy<Value = u16> {
        // Not using any::<u16>() because adding some u16s together later can get too large
        // So I started with the largest u16 and divided by 5, since we only add up 5 params for now
        0..13_107u16
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<u16>>) -> u16 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value)
    }

    fn no_params_return_value_getter(return_value: u16) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<u16>) -> String {
        format!("{}: nat16", arb_param.value)
    }

    fn assertion(result: u16, expected: u16) {
        assert!(result == expected);
    }
}
