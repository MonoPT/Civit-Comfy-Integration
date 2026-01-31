import subprocess
import sys

from pathlib import Path

def pip_install(pkg):
    wheel = Path(pkg).resolve()
    
    subprocess.check_call([
        sys.executable, "-m", "pip", "install", "--force-reinstall", str(wheel)
    ])

import importlib.util

def ensure(pkg):
    if importlib.util.find_spec(pkg) is None:
        pip_install(pkg)


def ensure_dependency(name: str):
    pip_install(name)