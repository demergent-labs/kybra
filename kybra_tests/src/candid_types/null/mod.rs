// KYBRA_CASES=1 cargo test candid_types::null::property_tests::basic -- --nocapture --exact

// TODO apparently aliases to None aren't valid for static analysis, but they still work
// TODO we probably want to update the alias code to accomodate this

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
                "from kybra import null, query, update".to_string(),
                &create_arb_null(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["null", "None"],
                no_params_return_string_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/null",
        )
    }

    fn create_arb_null() -> impl Strategy<Value = ()> {
        any::<()>()
    }

    // TODO we should really make this better
    // TODO we should use all of the params if possible
    fn params_return_string_getter(arb_params: Vec<ArbParam<()>>) -> String {
        // arb_params
        //     .iter()
        //     .map(|arb_param| arb_param.name.clone())
        //     .collect::<Vec<String>>()
        //     .join(" == ")
        arb_params[0].name.to_string()
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<()>>) -> () {
        ()
    }

    fn no_params_return_string_getter(return_value: ()) -> String {
        "None".to_string()
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<()>) -> String {
        "null".to_string()
    }

    fn assertion(result: (), expected: ()) {
        assert!(result == expected);
    }
}
