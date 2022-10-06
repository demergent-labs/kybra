from setuptools import setup

setup(
    name='kybra',
    version='0.0.5',
    package_data={
        '': ['compiler/**']
    },
    include_package_data=True,
    packages=['kybra'],
    install_requires=['modulegraph==0.19.3']
)
