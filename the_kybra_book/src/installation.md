# Installation

Follow the instructions exactly as stated below to avoid issues.

You should be using a \*nix environment (Linux, Mac OS, [WSL if using Windows](https://learn.microsoft.com/en-us/windows/wsl/install)) with bash and have the following installed on your system:

-   Python 3.10.7
-   dfx 0.12.1
-   Python VS Code Extension

#### Python 3.10.7

It is highly recommended to install Python 3.10.7 using [pyenv](https://github.com/pyenv/pyenv). To do so, use the [pyenv installer](https://github.com/pyenv/pyenv-installer) as shown below:

```bash
# install pyenv
curl https://pyenv.run | bash

# install Python 3.10.7
~/.pyenv/bin/pyenv install 3.10.7
```

#### dfx

Run the following command to install dfx 0.12.1:

```bash
DFX_VERSION=0.12.1 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

#### Python VS Code Extension

It is highly recommended to use VS Code and to install the [Microsoft Python extension](https://marketplace.visualstudio.com/items?itemName=ms-python.python) to get full type checking support from within the editor:

##### Extension

```
VS Code -> Preferences -> Extensions -> Search for Python by Microsoft and install it
```

##### Set python.analysis.typeCheckingMode

Set the setting `python.analysis.typeCheckingMode` to `strict`:

```
VS Code -> Preferences -> Settings -> Search for python.analysis.typeCheckingMode and set it to strict
```
