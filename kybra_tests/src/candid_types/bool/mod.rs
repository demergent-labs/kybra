// KYBRA_CASES=1 cargo test candid_types::bool::property_tests::basic -- --nocapture --exact

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
                "from kybra import query, update".to_string(),
                &create_arb_bool(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["bool"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/bool",
        )
    }

    fn create_arb_bool() -> impl Strategy<Value = bool> {
        any::<bool>()
    }

    // TODO we should probably do random and/or instead of just anding everything together
    fn params_return_string_getter(arb_params: Vec<ArbParam<bool>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.name.clone())
            .collect::<Vec<String>>()
            .join(" and ")
    }

    // TODO we need to have a different little algorithm thing for bool
    // TODO we should probably do the same with f32, so let's make this part generic
    fn params_return_value_getter(arb_params: Vec<ArbParam<bool>>) -> bool {
        arb_params
            .iter()
            .fold(true, |acc, arb_param| acc && arb_param.value)
    }

    fn no_params_return_value_getter(return_value: bool) -> String {
        if return_value == true {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<bool>) -> String {
        format!("{}", arb_param.value)
    }

    fn assertion(result: bool, expected: bool) {
        assert!(result == expected);
    }
}
