import kybra
import os
import subprocess
import sys

compiler_path = os.path.dirname(kybra.__file__) + '/compiler'
build_sh_path = compiler_path + '/build.sh'

subprocess.call([build_sh_path, sys.argv[1], sys.argv[2], sys.argv[3], compiler_path])
