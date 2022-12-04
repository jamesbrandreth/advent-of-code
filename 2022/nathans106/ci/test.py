import subprocess
import sys

result = subprocess.run("cargo test", cwd="2022/nathans106")
sys.exit(result.returncode)
