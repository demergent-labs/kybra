# Installation

-   [Dependencies](#dependencies)
-   [Common installation issues](#common-installation-issues)

## Dependencies

Follow the instructions exactly as stated below to avoid issues.

You should be using a \*nix environment (Linux, Mac OS, [WSL if using Windows](https://learn.microsoft.com/en-us/windows/wsl/install)) with bash and have the following installed on your system:

-   Python 3.10.7
-   dfx 0.22.0
-   Python VS Code Extension

### Python 3.10.7

It is highly recommended to install Python 3.10.7 using [pyenv](https://github.com/pyenv/pyenv). To do so, use the [pyenv installer](https://github.com/pyenv/pyenv-installer) as shown below:

```bash
# install pyenv
curl https://pyenv.run | bash

# install Python 3.10.7
~/.pyenv/bin/pyenv install 3.10.7
```

### dfx

Run the following command to install dfx 0.22.0:

```bash
DFX_VERSION=0.22.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

If after trying to run `dfx` commands you encounter an error such as `dfx: command not found`, you might need to add `$HOME/bin` to your path. Here's an example of doing this in your `.bashrc`:

```bash
echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
```

### Python VS Code Extension

It is highly recommended to use VS Code and to install the [Microsoft Python extension](https://marketplace.visualstudio.com/items?itemName=ms-python.python) to get full type checking support from within the editor:

#### Extension

```
VS Code -> Preferences -> Extensions -> Search for Python by Microsoft and install it
```

#### Set python.analysis.typeCheckingMode

Set the setting `python.analysis.typeCheckingMode` to `strict`:

```
VS Code -> Preferences -> Settings -> Search for python.analysis.typeCheckingMode and set it to strict
```

## Common installation issues

### Ubuntu

Error:

```bash
linker cc not found
```

Resolution:

```bash
sudo apt install build-essential
```

Error:

```bash
is cmake not installed?
```

Resolution:

```bash
sudo apt install cmake
```

Error:

```bash
ERROR: The Python ssl extension was not compiled. Missing the OpenSSL lib
```

Resolution:

You may have the right version of open ssl but you might be missing libssl-dev

```bash
sudo apt-get install libssl-dev
```
