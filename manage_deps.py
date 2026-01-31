import subprocess
import sys

def pip_install(pkg): # Upgrade or install
    subprocess.check_call([
        sys.executable, "-m", "pip", "install", "--upgrade", pkg
    ])

import importlib.util

def ensure(pkg): # Use in case i want to check if any version exists
    if importlib.util.find_spec(pkg) is None:
        pip_install(pkg)


def ensure_dependency(name: str):
    pip_install(name)