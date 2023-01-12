// KYBRA_CASES=1 cargo test candid_types::principal::property_tests::basic -- --nocapture --exact

// TODO we probably need to expand the capabilities of the tests here
// TODO we want to test every principal API if possible
// TODO Maybe we can just have multiple principal tests to cover each API

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
                "from kybra import Principal, query, update".to_string(),
                &create_arb_principal(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["Principal"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/principal",
        )
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<candid::Principal>) -> String {
        format!("principal \"{}\"", arb_param.value.to_text())
    }

    fn create_arb_principal() -> impl Strategy<Value = candid::Principal> {
        // any::<Vec<u8>>().prop_map(|blob| candid::Principal::from_slice(&blob[0..28]))
        prop::collection::vec(any::<u8>(), 0..29)
            .prop_map(|blob| candid::Principal::from_slice(&blob))
    }

    fn no_params_return_value_getter(return_value: candid::Principal) -> String {
        format!("Principal.from_str(\"{}\")", return_value.to_text())
    }

    // TODO again we aren't really testing multiple parameters
    fn params_return_string_getter(arb_params: Vec<ArbParam<candid::Principal>>) -> String {
        arb_params[0].name.to_string()
        // arb_params
        //     .iter()
        //     .map(|arb_param| format!("Principal.fromText(\"{}\")", arb_param.name.clone()))
        //     .collect::<Vec<String>>()
        //     .join(" + ")
    }

    fn params_return_value_getter(
        arb_params: Vec<ArbParam<candid::Principal>>,
    ) -> candid::Principal {
        arb_params[0].value
        // arb_params
        //     .iter()
        //     .map(|arb_param| arb_param.value.to_string())
        //     .collect::<Vec<String>>()
        //     .join("")
    }

    fn assertion(result: candid::Principal, expected: candid::Principal) {
        assert!(result.to_text() == expected.to_text());
    }
}
