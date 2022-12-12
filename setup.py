from setuptools import setup
from kybra import __version__

setup(
    name='kybra',
    version=__version__,
    package_data={
        'kybra': ['compiler/**', 'canisters/**']
    },
    include_package_data=True,
    packages=['kybra'],
    install_requires=['modulegraph==0.19.3']
)
