// KYBRA_CASES=1 cargo test candid_types::float32::property_tests::basic -- --nocapture --exact

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
                "from kybra import float32, query, update".to_string(),
                &create_arb_float32(),
                params_return_value_getter,
                &prop_oneof!["float32"],
                no_params_return_value_getter,
            ),
            assertion,
            arb_param_to_candid_string,
            "src/candid_types/float32",
        )
    }

    fn create_arb_float32() -> impl Strategy<Value = f32> {
        any::<f32>()
    }

    // TODO since Python doesn't have f32, I wonder if we're seeing some major issues
    fn params_return_value_getter(arb_params: Vec<ArbParam<f32>>) -> f32 {
        // TODO we shouldn't have to do this trick: https://github.com/demergent-labs/kybra/issues/218#issuecomment-1379791236
        arb_params.iter().fold(0.0, |acc, arb_param| {
            acc + if arb_param.value == -0.0 {
                0.0
            } else {
                arb_param.value
            }
        })
    }

    fn no_params_return_value_getter(return_value: f32) -> String {
        format!("{:#?}", return_value)
    }

    fn arb_param_to_candid_string(arb_param: ArbParam<f32>) -> String {
        format!("{:#?}: float32", arb_param.value)
    }

    fn assertion(result: f32, expected: f32) {
        assert!(result.to_ne_bytes() == expected.to_ne_bytes());
    }
}
