import os
import json


def insert_metadata(canister_config, metadata):
    # If 'declarations' is present, insert metadata before it; else, append metadata
    if "declarations" in canister_config:
        keys = list(canister_config.keys())
        decl_index = keys.index("declarations")
        # Insert metadata before 'declarations'
        new_config = {}
        for key in keys[:decl_index]:
            new_config[key] = canister_config[key]
        new_config["metadata"] = metadata
        for key in keys[decl_index:]:
            new_config[key] = canister_config[key]
        return new_config
    else:
        canister_config["metadata"] = metadata
        return canister_config


def update_dfx_file(dfx_file_path):
    with open(dfx_file_path, "r") as file:
        data = json.load(file)

    modified = False
    for canister_name, canister_config in data.get("canisters", {}).items():
        if canister_config.get("build", "").startswith("python -m kybra"):
            candid_service_path = canister_config.get("candid", "")
            new_metadata = [
                {"name": "candid:service", "path": candid_service_path},
                {"name": "cdk:name", "content": "kybra"},
            ]
            if "metadata" in canister_config:
                existing_names = [md["name"] for md in canister_config["metadata"]]
                for md in new_metadata:
                    if md["name"] not in existing_names:
                        canister_config["metadata"].append(md)
            else:
                canister_config = insert_metadata(canister_config, new_metadata)
                data["canisters"][canister_name] = canister_config
            modified = True

    if modified:
        with open(dfx_file_path, "w", newline="\n") as file:
            json.dump(data, file, indent=4)
            file.write("\n")  # Ensure there's a newline at the end
            print(f"Updated {dfx_file_path}")


def search_and_update_dfx_json_files(parent_directory):
    for directory in next(os.walk(parent_directory))[1]:  # List directories only
        dfx_file_path = os.path.join(parent_directory, directory, "dfx.json")
        if os.path.isfile(dfx_file_path):
            print(f"Processing {dfx_file_path}...")
            update_dfx_file(dfx_file_path)


parent_directory = "examples"  # Replace with your parent directory path
search_and_update_dfx_json_files(parent_directory)
