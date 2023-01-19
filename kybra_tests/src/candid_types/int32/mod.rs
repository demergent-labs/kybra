// KYBRA_CASES=1 cargo test candid_types::int32::property_tests::basic -- --nocapture --exact

// TODO do we want to test with bigger numbers? I am just using u128 right now for the strategy. To get HUGE numbers we could create a string of numbers...some regex to give us ginormous numbers

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
                "from kybra import int32, query, update".to_string(),
                &create_arb_int32(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["int32"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/int32",
        )
    }

    fn create_arb_int32() -> impl Strategy<Value = i32> {
        // Not using any::<i32>() because adding some i32s together later can get too large
        // So I started with the largest i32 and divided by 5, since we only add up 5 params for now
        -429_496_729..429_496_729i32
    }

    fn params_return_string_getter(arb_params: Vec<ArbParam<i32>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.name.clone())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<i32>>) -> i32 {
        arb_params
            .iter()
            .fold(0, |acc, arb_param| acc + arb_param.value.clone())
    }

    fn no_params_return_value_getter(return_value: i32) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<i32>) -> String {
        format!("{}: int32", arb_param.value)
    }

    fn assertion(result: i32, expected: i32) {
        assert!(result == expected);
    }
}
