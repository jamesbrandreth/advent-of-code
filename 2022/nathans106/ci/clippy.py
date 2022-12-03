import subprocess
import sys

result = subprocess.run("cargo clippy -- -Dwarnings", cwd="2022/nathans106")
sys.exit(result.returncode)
