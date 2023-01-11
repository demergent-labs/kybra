// KYBRA_CASES=1 cargo test candid_types::nat::property_tests::basic -- --nocapture --exact

// TODO abstract away what is common to other tests
// TODO we actually aren't testing the generated candid files...
// TODO do we want to test with bigger numbers? I am just using u128 right now for the strategy. To get HUGE numbers we could create a string of numbers...some regex to give us ginormous numbers

#[cfg(test)]
mod property_tests {
    use candid::Decode;
    use candid::IDLArgs;
    use ic_agent::{export::Principal, Agent};
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestRunner};
    use std::collections::HashSet;
    use std::env;
    use std::error::Error;
    use std::process::{Command, Stdio};

    #[test]
    fn basic() -> Result<(), Box<dyn Error>> {
        let mut runner = TestRunner::new(Config {
            cases: env::var("KYBRA_CASES")
                .expect("KYBRA_CASES is not defined")
                .parse()
                .expect("KYBRA_CASES must be a u32"),
            max_shrink_iters: 0,
            ..Config::default()
        });

        runner.run(&create_arb_program(), |arb_program| {
            let main_file_path = env::current_dir()?.join("src/candid_types/nat/src/main.py");

            std::fs::write(main_file_path, arb_program.code)?;

            let current_file_path = env::current_dir()?.join("src/candid_types/nat");

            let mut child = Command::new("bash")
                .arg("-c")
                .arg("~/.pyenv/versions/3.10.7/bin/python -m venv .dfx/kybra/venv; source .dfx/kybra/venv/bin/activate; pip install ../../../../; dfx deploy --yes")
                .current_dir(current_file_path.clone())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            child.wait()?;

            let agent = Agent::builder()
                .with_url("http://localhost:8000")
                .build()?;

            tokio_test::block_on(async {
                agent.fetch_root_key().await
            })?;

            // TODO if we can do this dynamically we might be able to use one dfx instance which would be awesome
            let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")?;

            for arb_function in arb_program.arb_functions {
                let args_candid_string = arb_function.arb_params.clone().into_iter().map(|arb_param| format!("{}: nat", arb_param.value)).collect::<Vec<String>>().join(",");
                let args: IDLArgs = format!("({args_candid_string})").parse()?;
                let encoded: Vec<u8> = args.to_bytes()?;

                let response = tokio_test::block_on(async {
                    match arb_function.method_type {
                        MethodType::QUERY => {
                            agent
                                .query(&canister_id, &arb_function.name)
                                .with_arg(&encoded)
                                .call()
                                .await
                        },
                        MethodType::UPDATE => {
                            agent
                                .update(&canister_id, &arb_function.name)
                                .with_arg(&encoded)
                                .call_and_wait()
                                .await
                        }
                    }
                })?;

                let result = Decode!(&response, candid::Nat)?;

                println!("function: {}", &arb_function.name);
                println!("\n");
                println!("result: {}", result);
                println!("arb_function.value: {}", &arb_function.value);
                println!("\n\n");

                assert!(result == arb_function.value);
            }

            Ok(())
        })?;

        Ok(())
    }

    #[derive(Debug, Clone)]
    struct ArbProgram {
        code: String,
        arb_functions: Vec<ArbFunction>,
    }

    #[derive(Debug, Clone)]
    enum MethodType {
        QUERY,
        UPDATE,
    }

    #[derive(Debug, Clone)]
    struct ArbFunction {
        code: String,
        name: String,
        method_type: MethodType,
        arb_params: Vec<ArbParam>,
        value: candid::Nat,
    }

    #[derive(Debug, Clone)]
    struct ArbParam {
        code: String,
        name: String,
        value: candid::Nat,
    }

    #[derive(Debug, Clone)]
    struct ArbAlias {
        code: String,
        name: String,
    }

    // TODO aliases have to be unique as well
    fn create_arb_program() -> impl Strategy<Value = ArbProgram> {
        prop::collection::vec(create_arb_alias(), 0..100).prop_flat_map(|arb_aliases| {
            let unique_arb_aliases = deduplicate_arb_aliases(arb_aliases);

            prop::collection::vec(create_arb_function(&unique_arb_aliases), 0..100).prop_map(move |arb_functions| {
                let unique_arb_functions = deduplicate_arb_functions(arb_functions.clone());
                let unique_arb_function_codes = unique_arb_functions.clone().iter().map(|arb_function| {
                    arb_function.code.clone()
                })
                .collect::<Vec<String>>()
                .join("\n\n\n");

                let alias_codes = unique_arb_aliases.iter().map(|arb_alias| arb_alias.code.clone()).collect::<Vec<String>>().join("\n");

                ArbProgram {
                    code: format!("from kybra import nat, query, update\n\n\n{alias_codes}\n\n\n{unique_arb_function_codes}"),
                    arb_functions
                }
            })
        })
    }

    fn create_arb_function(arb_aliases: &Vec<ArbAlias>) -> impl Strategy<Value = ArbFunction> {
        (
            create_arb_python_name(),
            prop::collection::vec(create_arb_param(arb_aliases), 0..5), // TODO Once we can support more params, let's increase this number
            create_arb_type(arb_aliases),
            prop_oneof!["@query", "@update"],
            create_arb_nat()
        ).prop_map(|(arb_function_name, arb_params, arb_return_type, arb_decorator, arb_return_value)| {
            let params = arb_params.iter().map(|arb_param| arb_param.code.clone()).collect::<Vec<String>>().join(", ");

            let (
                return_string,
                return_value
            ) = get_return_info(&arb_params, &arb_return_value);

            ArbFunction {
                code: format!("{arb_decorator}\ndef {arb_function_name}({params}) -> {arb_return_type}:\n    return {return_string}"),
                name: arb_function_name,
                method_type: if arb_decorator == "@query" { MethodType::QUERY } else { MethodType::UPDATE },
                arb_params: arb_params,
                value: return_value
            }
        })
    }

    fn create_arb_alias() -> impl Strategy<Value = ArbAlias> {
        (create_arb_python_name(), "nat").prop_map(|(arb_python_name, arb_type)| ArbAlias {
            code: format!("{arb_python_name} = {arb_type}"),
            name: arb_python_name,
        })
    }

    fn create_arb_param(arb_aliases: &Vec<ArbAlias>) -> impl Strategy<Value = ArbParam> {
        (
            create_arb_python_name(),
            create_arb_type(arb_aliases),
            create_arb_nat(),
        )
            .prop_map(
                |(arb_param_name, arb_param_type, arb_param_value)| ArbParam {
                    code: format!("{arb_param_name}: {arb_param_type}"),
                    name: arb_param_name,
                    value: arb_param_value,
                },
            )
    }

    fn create_arb_type(arb_aliases: &Vec<ArbAlias>) -> impl Strategy<Value = String> {
        prop_oneof![
            "nat",
            Just(arb_aliases.clone())
                .prop_shuffle()
                .prop_map(|arb_aliases| if arb_aliases.len() == 0 {
                    "nat".to_string()
                } else {
                    arb_aliases[0].name.clone()
                })
        ]
    }

    fn create_arb_python_name() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_]*" // TODO underscores are not valid as the first character until this is fixed: https://github.com/demergent-labs/kybra/issues/218#issuecomment-1376175383
    }

    fn create_arb_nat() -> impl Strategy<Value = candid::Nat> {
        any::<u128>().prop_map(|arb_u128| arb_u128.into())
    }

    fn deduplicate_arb_functions(arb_functions: Vec<ArbFunction>) -> Vec<ArbFunction> {
        let mut set = HashSet::new();
        let mut result = Vec::new();
        for arb_function in arb_functions {
            if set.insert(arb_function.clone().name) {
                result.push(arb_function.clone());
            }
        }
        result
    }

    fn deduplicate_arb_aliases(arb_aliases: Vec<ArbAlias>) -> Vec<ArbAlias> {
        let mut set = HashSet::new();
        let mut result = Vec::new();
        for arb_alias in arb_aliases {
            if set.insert(arb_alias.clone().name) {
                result.push(arb_alias.clone());
            }
        }
        result
    }

    fn get_return_info(
        arb_params: &Vec<ArbParam>,
        arb_return_value: &candid::Nat,
    ) -> (String, candid::Nat) {
        if arb_params.len() == 0 {
            let return_string = arb_return_value.to_string();
            let return_value = arb_return_value.clone();

            (return_string, return_value)
        } else {
            let return_string = arb_params
                .iter()
                .map(|arb_param| arb_param.name.clone())
                .collect::<Vec<String>>()
                .join(" + ");
            let return_value = arb_params
                .iter()
                .fold(candid::Nat::from(0), |acc, arb_param| {
                    acc + arb_param.value.clone()
                });

            (return_string, return_value)
        }
    }
}
