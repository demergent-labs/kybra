// KYBRA_CASES=1 cargo test candid_types::blob::property_tests::basic -- --nocapture --exact

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
                "from kybra import blob, query, update".to_string(),
                &create_arb_blob(),
                params_return_string_getter,
                params_return_value_getter,
                &prop_oneof!["blob"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/blob",
        )
    }

    fn create_arb_blob() -> impl Strategy<Value = Vec<u8>> {
        any::<Vec<u8>>()
    }

    fn params_return_string_getter(arb_params: Vec<ArbParam<Vec<u8>>>) -> String {
        arb_params
            .iter()
            .map(|arb_param| arb_param.name.clone())
            .collect::<Vec<String>>()
            .join(" + ")
    }

    fn params_return_value_getter(arb_params: Vec<ArbParam<Vec<u8>>>) -> Vec<u8> {
        let mut accumulator = vec![];

        for mut arb_param in arb_params {
            accumulator.append(&mut arb_param.value);
        }

        accumulator
    }

    fn no_params_return_value_getter(return_value: Vec<u8>) -> String {
        format!(
            "bytes([{}])",
            return_value
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<Vec<u8>>) -> String {
        format!(
            "vec {{{}}}",
            arb_param
                .value
                .iter()
                .map(|item| { format!("{}: nat8", item) })
                .collect::<Vec<String>>()
                .join("; ")
        )
    }

    fn assertion(result: Vec<u8>, expected: Vec<u8>) {
        assert!(result == expected);
    }
}
