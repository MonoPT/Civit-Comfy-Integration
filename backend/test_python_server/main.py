import os

import subprocess
from pathlib import Path
import importlib.util

__FILE__ = os.path.dirname(os.path.abspath(__file__))

module_path = Path(f"{__FILE__}/../../manage_deps.py")

print(module_path)

requireDep = importlib.util.spec_from_file_location(
    "my_module",
    module_path
)

module = importlib.util.module_from_spec(requireDep)
requireDep.loader.exec_module(module)

module.ensure_dependency("rust_civit_comfy_bindings")

from rust_civit_comfy_bindings import sum_as_string, start_server

import socketserver
import socket

def is_port_free(port, host="127.0.0.1"):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex((host, port)) != 0

if is_port_free(3090):
    port = 3090
else:
    with socketserver.TCPServer(("localhost", 0), None) as server:
        port = server.server_address[1]

from multiprocessing import Process
from pathlib import Path


static_dir = f"{__FILE__}/../../front-end/build"
comfy_path = Path(f"C:/ComfyUI").resolve()
print(f"Comfy path: {comfy_path}. Dont forget to update this if yours is different")

def main():
    import sys
    
    subprocess.Popen(
        [sys.executable, f"{__FILE__}/../../server.py", f"{port}", static_dir, comfy_path],
        stdin=subprocess.DEVNULL,
        stdout=None,
        stderr=None,
        close_fds=True
    )
    
    print("Running server on another process with port:", port)
    
    while True:
        ""
        
if __name__ == "__main__":
    main()
