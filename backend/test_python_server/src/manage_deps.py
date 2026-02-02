import subprocess
import sys
import os
from pathlib import Path

def pip_install(pkg):
    wheel = Path(pkg).resolve()
        
    whl_files = [f for f in os.listdir(wheel) if f.endswith('.whl')]
        
    whl_files.sort(reverse=True)
    
    wheel = f"{wheel}/{whl_files[0]}"
       
    subprocess.check_call([
        sys.executable, "-m", "pip", "install", "--force-reinstall", str(wheel)
    ])

import importlib.util

def ensure(pkg):
    if importlib.util.find_spec(pkg) is None:
        pip_install(pkg)


def ensure_dependency(name: str):
    pip_install(name)