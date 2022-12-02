import subprocess
import sys

result = subprocess.run("cargo clippy", cwd="2022/nathans106")
sys.exit(result.returncode)
