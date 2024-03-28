import os
import json
from collections import OrderedDict


def process_file(filepath):
    with open(filepath, "r") as file:
        data = json.load(
            file, object_pairs_hook=OrderedDict
        )  # Ensure order is maintained

    updated = False
    for canister_name, canister_data in data.get("canisters", {}).items():
        # Check if 'post_install' exists and remove it
        if "post_install" in canister_data:
            del canister_data["post_install"]
            updated = True

    if updated:
        with open(filepath, "w") as file:
            json.dump(data, file, indent=4)
            file.write("\n")


def find_and_process_dfx_files(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file == "dfx.json":
                filepath = os.path.join(root, file)
                process_file(filepath)


if __name__ == "__main__":
    directory = "examples"  # Set this to your target directory
    find_and_process_dfx_files(directory)
