// TODO refactor the lifetimes
// TODO see if we can just implement a trait on a Struct or something, maybe we don't even need a trait?
// TODO it would be easier to pass around a struct that has everything on it, rather than having to pass params around everywhere

use candid::CandidType;
use candid::Decode;
use candid::IDLArgs;
use ic_agent::{export::Principal, Agent};
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::process::{Command, Stdio};

mod blob;
mod bool;
mod float32;
mod float64;
mod int;
mod int16;
mod int32;
mod int64;
mod int8;
mod nat;
mod nat16;
mod nat32;
mod nat64;
mod nat8;
mod null;
mod principal;
mod text;

const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

const RUST_KEYWORDS: [&str; 51] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

#[derive(Debug, Clone)]
pub struct ArbProgram<CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>> {
    code: String,
    arb_functions: Vec<ArbFunction<CandidValue>>,
}

#[derive(Debug, Clone)]
enum MethodType {
    QUERY,
    UPDATE,
}

#[derive(Debug, Clone)]
struct ArbFunction<CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>> {
    code: String,
    name: String,
    method_type: MethodType,
    arb_params: Vec<ArbParam<CandidValue>>,
    value: CandidValue,
}

#[derive(Debug, Clone)]
pub struct ArbParam<CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>> {
    code: String,
    name: String,
    value: CandidValue,
}

#[derive(Debug, Clone)]
struct ArbAlias {
    code: String,
    name: String,
}

pub fn run_candid_types_property_tests<
    T: Strategy<Value = ArbProgram<CandidValue>>,
    CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a> + std::cmp::PartialEq + 'static,
>(
    arb_program_strategy: T,
    assertion: fn(CandidValue, CandidValue),
    arb_param_to_candid_string: fn(ArbParam<CandidValue>) -> String,
    test_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut runner = TestRunner::new(Config {
        cases: env::var("KYBRA_CASES")
            .expect("KYBRA_CASES is not defined")
            .parse()
            .expect("KYBRA_CASES must be a u32"),
        max_shrink_iters: 0,
        ..Config::default()
    });

    runner.run(&arb_program_strategy, |arb_program| {
        let main_file_path = env::current_dir()?.join(format!("{test_path}/src/main.py"));

        std::fs::write(main_file_path, arb_program.code)?;

        let current_file_path = env::current_dir()?.join(test_path);

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
            let args_candid_string = arb_function.arb_params.clone().into_iter().map(|arb_param| arb_param_to_candid_string(arb_param)).collect::<Vec<String>>().join(",");

            println!("args_candid_string: {}", args_candid_string);

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

            let result = Decode!(&response, CandidValue)?;

            println!("function: {}", &arb_function.name);
            println!("\n");
            println!("result: {:#?}", result);
            println!("arb_function.value: {:#?}", &arb_function.value);
            println!("\n\n");

            assertion(result, arb_function.value);
        }

        Ok(())
    })?;

    Ok(())
}

pub fn create_arb_program<
    'a,
    'c,
    CandidValue: CandidType + Clone + Debug + for<'b> Deserialize<'b> + 'static,
    ArbCandidValueStrategy: Strategy<Value = CandidValue> + 'a,
    K: Strategy<Value = String> + 'c,
>(
    import_statement: String,
    arb_candid_value_strategy: &'c ArbCandidValueStrategy,
    params_return_string_getter: fn(Vec<ArbParam<CandidValue>>) -> String,
    params_return_value_getter: fn(Vec<ArbParam<CandidValue>>) -> CandidValue,
    arb_type_strategy: &'c K,
    no_params_return_string_getter: fn(CandidValue) -> String,
) -> impl Strategy<Value = ArbProgram<CandidValue>> + 'c {
    prop::collection::vec(create_arb_alias(arb_type_strategy), 1..100).prop_flat_map(
        move |arb_aliases| {
            let unique_arb_aliases = deduplicate_arb_aliases(arb_aliases);
            let import_statement = import_statement.clone();
            prop::collection::vec(
                create_arb_function(
                    &unique_arb_aliases,
                    arb_candid_value_strategy,
                    params_return_string_getter,
                    params_return_value_getter,
                    arb_type_strategy,
                    no_params_return_string_getter,
                ),
                0..100,
            )
            .prop_map(move |arb_functions| {
                let unique_arb_functions = deduplicate_arb_functions(arb_functions.clone());
                let unique_arb_function_codes = unique_arb_functions
                    .clone()
                    .iter()
                    .map(|arb_function| arb_function.code.clone())
                    .collect::<Vec<String>>()
                    .join("\n\n\n");

                let alias_codes = unique_arb_aliases
                    .iter()
                    .map(|arb_alias| arb_alias.code.clone())
                    .collect::<Vec<String>>()
                    .join("\n");

                ArbProgram {
                    code: format!(
                        "{import_statement}\n\n\n{alias_codes}\n\n\n{unique_arb_function_codes}"
                    ),
                    arb_functions: unique_arb_functions,
                }
            })
        },
    )
}

fn create_arb_function<
    'b,
    'c,
    CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a> + 'c + 'b + 'static,
    ArbCandidValueStrategy: Strategy<Value = CandidValue>,
    K: Strategy<Value = String> + 'b,
>(
    arb_aliases: &Vec<ArbAlias>,
    arb_candid_value_strategy: &'b ArbCandidValueStrategy,
    params_return_string_getter: fn(Vec<ArbParam<CandidValue>>) -> String,
    params_return_value_getter: fn(Vec<ArbParam<CandidValue>>) -> CandidValue,
    arb_type_strategy: &'b K,
    no_params_return_string_getter: fn(CandidValue) -> String,
) -> impl Strategy<Value = ArbFunction<CandidValue>> + 'b {
    (
        create_arb_python_name(),
        prop::collection::vec(create_arb_param(arb_aliases, arb_candid_value_strategy, arb_type_strategy), 0..5), // TODO Once we can support more params, let's increase this number
        create_arb_type(arb_aliases, arb_type_strategy),
        prop_oneof!["@query", "@update"],
        arb_candid_value_strategy
    ).prop_map(move |(arb_function_name, arb_params, arb_return_type, arb_decorator, arb_return_value)| {
        let params = arb_params.iter().map(|arb_param| arb_param.code.clone()).collect::<Vec<String>>().join(", ");

        let (
            return_string,
            return_value
        ) = get_return_value(arb_params.clone(), arb_return_value.clone(), params_return_string_getter, params_return_value_getter, no_params_return_string_getter);

        ArbFunction {
            code: format!("{arb_decorator}\ndef {arb_function_name}({params}) -> {arb_return_type}:\n    return {return_string}"),
            name: arb_function_name,
            method_type: if arb_decorator == "@query" { MethodType::QUERY } else { MethodType::UPDATE },
            arb_params: arb_params.clone(),
            value: return_value
        }
    })
}

fn create_arb_alias<'a, K: Strategy<Value = String>>(
    arb_type_strategy: &'a K,
) -> impl Strategy<Value = ArbAlias> + 'a {
    (create_arb_python_name(), arb_type_strategy).prop_map(|(arb_python_name, arb_type)| ArbAlias {
        code: format!("{arb_python_name} = {arb_type}"),
        name: arb_python_name,
    })
}

fn create_arb_param<
    'b,
    CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>,
    T: Strategy<Value = CandidValue> + 'b,
    K: Strategy<Value = String> + 'b,
>(
    arb_aliases: &Vec<ArbAlias>,
    arb_candid_value_strategy: &'b T,
    arb_type_strategy: K,
) -> impl Strategy<Value = ArbParam<CandidValue>> + 'b {
    (
        create_arb_python_name(),
        create_arb_type(arb_aliases, arb_type_strategy),
        arb_candid_value_strategy,
    )
        .prop_map(
            |(arb_param_name, arb_param_type, arb_param_value)| ArbParam {
                code: format!("{arb_param_name}: {arb_param_type}"),
                name: arb_param_name.clone(),
                value: arb_param_value.clone(),
            },
        )
}

fn create_arb_type<T: Strategy<Value = String>>(
    arb_aliases: &Vec<ArbAlias>,
    arb_type_strategy: T,
) -> impl Strategy<Value = String> {
    prop_oneof![
        arb_type_strategy,
        Just(arb_aliases.clone())
            .prop_shuffle()
            .prop_map(|arb_aliases| arb_aliases[0].name.clone())
    ]
}

fn create_arb_python_name() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9_]*".prop_map(|arb_python_name| {
        if arb_python_name.len() == 1 {
            format!("{arb_python_name}_") // TODO this is strange https://github.com/demergent-labs/kybra/issues/218#issuecomment-1378085756
        } else if PYTHON_KEYWORDS.contains(&arb_python_name.as_str())
            || RUST_KEYWORDS.contains(&arb_python_name.as_str())
        // TODO we shouldn't have to strip out Rust keywords: https://github.com/demergent-labs/kybra/issues/218#issuecomment-1380999174
        {
            arb_python_name + "_"
        } else {
            arb_python_name
        }
    }) // TODO underscores are not valid as the first character until this is fixed: https://github.com/demergent-labs/kybra/issues/218#issuecomment-1376175383
}

fn deduplicate_arb_functions<CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>>(
    arb_functions: Vec<ArbFunction<CandidValue>>,
) -> Vec<ArbFunction<CandidValue>> {
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

fn get_return_value<'b, CandidValue: CandidType + Clone + Debug + for<'a> Deserialize<'a>>(
    arb_params: Vec<ArbParam<CandidValue>>,
    arb_return_value: CandidValue,
    params_return_string_getter: fn(Vec<ArbParam<CandidValue>>) -> String,
    params_return_value_getter: fn(Vec<ArbParam<CandidValue>>) -> CandidValue,
    no_params_return_string_getter: fn(CandidValue) -> String,
) -> (String, CandidValue) {
    if arb_params.len() == 0 {
        (
            no_params_return_string_getter(arb_return_value.clone()),
            arb_return_value,
        )
    } else {
        (
            params_return_string_getter(arb_params.clone()),
            params_return_value_getter(arb_params),
        )
    }
}
