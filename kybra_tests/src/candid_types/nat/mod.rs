// KYBRA_CASES=1 cargo test candid_types::nat::property_tests::basic -- --nocapture --exact

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
                "from kybra import nat, query, update".to_string(),
                &create_arb_nat(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["nat"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/nat",
        )
    }

    fn create_arb_nat() -> impl Strategy<Value = candid::Nat> {
        any::<u128>().prop_map(|arb_u128| arb_u128.into())
    }

    fn params_return_string_getter(arb_params: Vec<ArbParam<candid::Nat>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.name.clone())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<candid::Nat>>) -> candid::Nat {
        arb_params
            .iter()
            .fold(candid::Nat::from(0), |acc, arb_param| {
                acc + arb_param.value.clone()
            })
    }

    fn no_params_return_value_getter(return_value: candid::Nat) -> String {
        format!("{}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<candid::Nat>) -> String {
        format!("{}: nat", arb_param.value)
    }

    fn assertion(result: candid::Nat, expected: candid::Nat) {
        assert!(result == expected);
    }
}
