#!/bin/bash

set -e

root_dir=$PWD

VERSION=$1

directories_json_string_with_linebreaks=$2
directories_json_string="${directories_json_string_with_linebreaks//$'\\n'/''}"
directories=$(echo "$directories_json_string" | jq -c -r '.[]')

sed -E -i "s/(__version__ = \")(.*)(\")/\1$VERSION\3/" kybra/__init__.py
sed -E -i "s/(\"version\": \")(.*)(\")/\1$VERSION\3/" kybra/compiler/dfx_extension/extension.json

# prepare on new machine
~/.pyenv/versions/3.10.7/bin/python -m pip install --upgrade build
~/.pyenv/versions/3.10.7/bin/python -m pip install --upgrade twine

# build
~/.pyenv/versions/3.10.7/bin/python -m build

# upload
~/.pyenv/versions/3.10.7/bin/python -m twine upload -u __token__ -p "$PYPI_TOKEN" --skip-existing dist/*

# TODO loop through checking for the status instead of sleeping
echo -e "sleeping for 30 seconds to ensure kybra==$VERSION is fully registered on PyPI"

sleep 30

for directory in ${directories[@]}
do
    cd $directory

    sed -E -i "s/(kybra==)(.*)()/\1$VERSION\3/" requirements.txt

    cd $root_dir
done

cd examples/simple_erc20
~/.pyenv/versions/3.10.7/bin/python -m venv venv
source venv/bin/activate
pip install ../..
python -m kybra install-dfx-extension
dfx start --background
KYBRA_COMPILE_RUST_PYTHON_STDLIB=true KYBRA_REBUILD=true dfx deploy
cd .kybra/simple_erc20
tar -czf "$HOME/.config/kybra/$VERSION/rust_python_stdlib.tar.gz" "rust_python_stdlib"

git add --all
git commit -am "kybra-bot automated release $VERSION"
git push origin $GITHUB_HEAD_REF

git tag $VERSION
git push origin $VERSION

if [[ "$VERSION" == *"rc"* ]];
then
    gh release create "$VERSION" "$HOME/.config/kybra/$VERSION/rust_python_stdlib.tar.gz" -t "$VERSION" --prerelease
else
    gh release create "$VERSION" "$HOME/.config/kybra/$VERSION/rust_python_stdlib.tar.gz" -t "$VERSION"
fi
