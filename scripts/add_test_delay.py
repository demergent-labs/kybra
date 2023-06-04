import os

# Define the directory to search
directory = 'examples'

# Walk through all directories and files in the specified directory
for root, dirs, files in os.walk(directory):
    for file in files:
        # Find all 'pretest.ts' files
        if file == 'pretest.ts':
            file_path = os.path.join(root, file)
            with open(file_path, 'r') as f:
                lines = f.readlines()
            # Look for the 'dfx deploy' command line
            for i, line in enumerate(lines):
                if 'dfx deploy' in line:
                    # Find the closing brace of this execSync statement
                    while not '});' in lines[i]:
                        i += 1
                    # Add new line after the closing brace of the 'dfx deploy' command
                    lines.insert(i+1, '\n    await new Promise((resolve) => setTimeout(resolve, 10_000));\n')
                    break
            # Write changes back to file
            with open(file_path, 'w') as f:
                f.writelines(lines)
