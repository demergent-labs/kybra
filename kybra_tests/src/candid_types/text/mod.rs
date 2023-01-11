// KYBRA_CASES=1 cargo test candid_types::text::property_tests::basic -- --nocapture --exact

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
                "from kybra import query, text, update".to_string(),
                &create_arb_string(),
                params_return_value_getter,
            ),
            assertion,
        )
    }

    fn create_arb_string() -> impl Strategy<Value = String> {
        any::<String>().prop_map(|s| s.replace("\\", "\\\\").replace("\"", "\\\""))
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<String>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.value.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn assertion(result: String, expected: String) {
        assert!(result.replace("\\", "\\\\").replace("\"", "\\\"") == expected);
    }
}
