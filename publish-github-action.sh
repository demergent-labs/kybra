#!/bin/bash

set -e

root_dir=$PWD

VERSION=$1

directories_json_string_with_linebreaks=$2
directories_json_string="${directories_json_string_with_linebreaks//$'\\n'/''}"
directories=$(echo "$directories_json_string" | jq -c -r '.[]')

sed -E -i "s/(__version__ = \")(.*)(\")/\1$VERSION\3/" kybra/__init__.py

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

git add --all
git commit -am "kybra-bot automated release $VERSION"
git push origin $GITHUB_HEAD_REF

git tag $VERSION
git push origin $VERSION
