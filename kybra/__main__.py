import site
import subprocess
import sys

subprocess.call([site.getsitepackages()[0] + '/compiler/build.sh', sys.argv[1], sys.argv[2], sys.argv[3]])
