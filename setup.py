from kybra import __version__
from pathlib import Path
from setuptools import setup  # type: ignore

setup(
    name="kybra",
    version=__version__,
    package_data={"kybra": ["compiler/**", "canisters/**", "py.typed"]},
    include_package_data=True,
    packages=["kybra"],
    install_requires=["modulegraph==0.19.3"],
    long_description=Path("README.md").read_text(encoding="utf-8"),
    long_description_content_type="text/markdown",
)
