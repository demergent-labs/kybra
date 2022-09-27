import kybra
import os
import subprocess
import sys

compiler_path = os.path.dirname(kybra.__file__) + '/compiler'

subprocess.call([compiler_path + '/build.sh', sys.argv[1], sys.argv[2], sys.argv[3], compiler_path])
