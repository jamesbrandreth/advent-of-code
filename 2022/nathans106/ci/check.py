import subprocess
import sys

result = subprocess.run("cargo check", cwd="2022/nathans106")
sys.exit(result.returncode)
