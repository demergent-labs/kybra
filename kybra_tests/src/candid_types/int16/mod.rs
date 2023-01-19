// KYBRA_CASES=1 cargo test candid_types::int16::property_tests::basic -- --nocapture --exact

#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use std::error::Error;

    use crate::candid_types::create_arb_program;
    use crate::candid_types::run_candid_types_property_tests;
    use crate::candid_types::ArbParam;

    #[test]
    fn basic() -> Result<(), Box<dyn Error>> {
        run_candid_types_property_tests(
            create_arb_program(
                "from kybra import int16, query, update".to_string(),
                &create_arb_int16(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["int16"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/int16",
        )
    }

    fn create_arb_int16() -> impl Strategy<Value = i16> {
        // Not using any::<i16>() because adding some i16s together later can get too large
        // So I started with the largest i16 and divided by 5, since we only add up 5 params for now
        -6_553..6_553i16
    }

    fn params_return_string_getter(arb_params: Vec<ArbParam<i16>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.name.clone())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<i16>>) -> i16 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value.clone())
    }

    fn no_params_return_value_getter(return_value: i16) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<i16>) -> String {
        format!("{}: int16", arb_param.value)
    }

    fn assertion(result: i16, expected: i16) {
        assert!(result == expected);
    }
}
