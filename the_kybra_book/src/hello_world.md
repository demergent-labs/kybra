# Hello World

-   [The project directory and file structure](#the-project-directory-and-file-structure)
-   [main.py](#mainpy)
-   [dfx.json](#dfxjson)
-   [Local deployment](#local-deployment)
-   [Interacting with your canister from the command line](#interacting-with-your-canister-from-the-command-line)
-   [Interacting with your canister from the web UI](#interacting-with-your-canister-from-the-web-ui)

Let's build your first application (canister) with Kybra!

Before embarking please ensure you've followed all of [the installation instructions](./installation.md).

We'll build a simple `Hello World` canister that shows the basics of importing Kybra, exposing a query method, exposing an update method, and storing some state in a global variable. We'll then interact with it from the command line and from our web browser.

## The project directory and file structure

Assuming you're starting completely from scratch, run these commands to setup your project's directory and file structure:

```bash
mkdir kybra_hello_world
cd kybra_hello_world

mkdir src

touch src/main.py
touch dfx.json
```

Now create and source a virtual environment:

```bash
~/.pyenv/versions/3.10.7/bin/python -m venv venv
source venv/bin/activate
```

Now install Kybra:

```bash
pip install kybra
```

Open up `kybra_hello_world` in your text editor (we recommend [VS Code](https://code.visualstudio.com/) with the [Microsoft Python extension](https://marketplace.visualstudio.com/items?itemName=ms-python.python)).

## main.py

Here's the main code of the project, which you should put in the `kybra_hello_world/src/main.py` file of your canister:

```python
from kybra import query, update, void

# This is a global variable that is stored on the heap
message: str = ''

# Query calls complete quickly because they do not go through consensus
@query
def get_message() -> str:
    return message

# Update calls take a few seconds to complete
# This is because they persist state changes and go through consensus
@update
def set_message(new_message: str) -> void:
    global message
    message = new_message # This change will be persisted
```

Let's discuss each section of the code.

```python
from kybra import query, update, void
```

The code starts off by importing the `query` and `update` decorators from `kybra`, along with the `void` type. The `kybra` module provides most of the Internet Computer (IC) APIs for your canister.

```python
# This is a global variable that is stored on the heap
message: str = ''
```

We have created a global variable to store the state of our application. This variable is in scope to all of the functions defined in this module. We have annotated it with a type and set it equal to an empty string.

```python
# Query calls complete quickly because they do not go through consensus
@query
def get_message() -> str:
    return message
```

We are exposing a canister query method here. When query methods are called they execute quickly because they do not have to go through consensus. This method simply returns our global `message` variable.

```python
# Update calls take a few seconds to complete
# This is because they persist state changes and go through consensus
@update
def set_message(new_message: str) -> void:
    global message
    message = new_message # This change will be persisted
```

We are exposing an update method here. When update methods are called they take a few seconds to complete. This is because they persist changes and go through consensus. A majority of nodes in a subnet must agree on all state changes introduced in calls to update methods. This method accepts a `string` from the caller and will store it in our global `message` variable.

That's it! We've created a very simple getter/setter `Hello World` application. But no `Hello World` project is complete without actually yelling `Hello world`!

To do that, we'll need to setup the rest of our project.

## dfx.json

Create the following in `kybra_hello_world/dfx.json`:

```json
{
    "canisters": {
        "kybra_hello_world": {
            "type": "custom",
            "build": "python -m kybra kybra_hello_world src/main.py src/main.did",
            "candid": "src/main.did",
            "wasm": ".kybra/kybra_hello_world/kybra_hello_world.wasm.gz",
            "declarations": {
                "output": "test/dfx_generated/kybra_hello_world"
            }
        }
    }
}
```

## Local deployment

Let's deploy to our local replica.

First startup the replica:

```bash
dfx start --background
```

Then deploy the canister:

```bash
dfx deploy
```

## Interacting with your canister from the command line

Once we've deployed we can ask for our message:

```bash
dfx canister call kybra_hello_world get_message
```

We should see `("")` representing an empty message.

Now let's yell `Hello World!`:

```bash
dfx canister call kybra_hello_world set_message '("Hello World!")'
```

Retrieve the message:

```bash
dfx canister call kybra_hello_world get_message
```

We should see `("Hello World!")`.

## Interacting with your canister from the web UI

After deploying your canister, you should see output similar to the following in your terminal:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    kybra_hello_world: http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
```

Open up [http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai](http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai) or the equivalent URL from your terminal to access the web UI and interact with your canister.
