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

    fn params_return_string_getter(arb_params: Vec<ArbParam<bool>>) -> String {
        arb_params
            .iter()
            .enumerate()
            .map(|(index, arb_param)| {
                if index == arb_params.len() - 1 {
                    arb_param.name.to_string()
                } else {
                    format!(
                        "{} {} ",
                        arb_param.name,
                        if index % 2 == 0 { "and" } else { "or" }
                    )
                }
            })
            .collect::<String>()
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<bool>>) -> bool {
        arb_params
            .iter()
            .enumerate()
            .fold(true, |acc, (index, arb_param)| {
                if index % 2 != 0 {
                    acc && arb_param.value
                } else {
                    acc || arb_param.value
                }
            })
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
