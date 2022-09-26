from distutils.core import setup

setup(
    name='kybra',
    version='0.0.1',
    package_data={
        '': ['compiler/**']
    },
    include_package_data=True,
    packages=['kybra']
)
