// KYBRA_CASES=1 cargo test candid_types::int8::property_tests::basic -- --nocapture --exact

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
                "from kybra import int8, query, update".to_string(),
                &create_arb_int8(),
                params_return_value_getter,
                &prop_oneof!["int8"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/int8",
        )
    }

    fn create_arb_int8() -> impl Strategy<Value = i8> {
        // Not using any::<i8>() because adding some i8s together later can get too large
        // So I started with the largest i8 and divided by 5, since we only add up 5 params for now
        -128..25i8
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<i8>>) -> i8 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value.clone())
    }

    fn no_params_return_value_getter(return_value: i8) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<i8>) -> String {
        format!("{}: int8", arb_param.value)
    }

    fn assertion(result: i8, expected: i8) {
        assert!(result == expected);
    }
}
