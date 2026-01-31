import subprocess
import sys

def pip_install(pkg):
    subprocess.check_call([
        sys.executable, "-m", "pip", "install", "--upgrade", pkg
    ])

import importlib.util

def ensure(pkg):
    if importlib.util.find_spec(pkg) is None:
        pip_install(pkg)



def ensure_dependency(name: str):
    ensure(name)