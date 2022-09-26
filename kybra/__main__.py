import kybra
import os
import subprocess
import sys

subprocess.call([os.path.dirname(kybra.__file__) + '/compiler/build.sh', sys.argv[1], sys.argv[2], sys.argv[3]])
