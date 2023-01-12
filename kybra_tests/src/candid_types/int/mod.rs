// KYBRA_CASES=1 cargo test candid_types::int::property_tests::basic -- --nocapture --exact

// TODO do we want to test with bigger numbers? I am just using i128 right now for the strategy. To get HUGE numbers we could create a string of numbers...some regex to give us ginormous numbers

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
                "from kybra import int, query, update".to_string(),
                &create_arb_int(),
                params_return_value_getter,
                &prop_oneof!["int"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/int",
        )
    }

    fn create_arb_int() -> impl Strategy<Value = candid::Int> {
        any::<i128>().prop_map(|arb_i128| arb_i128.into())
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<candid::Int>>) -> candid::Int {
        arb_params
            .iter()
            .fold(candid::Int::from(0), |acc, arb_param| {
                acc + arb_param.value.clone()
            })
    }

    fn no_params_return_value_getter(return_value: candid::Int) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<candid::Int>) -> String {
        format!("{}: int", arb_param.value)
    }

    fn assertion(result: candid::Int, expected: candid::Int) {
        assert!(result == expected);
    }
}
