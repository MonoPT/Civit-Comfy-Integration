import os
from src.manage_deps import ensure_dependency

__FILE__ = os.path.dirname(os.path.abspath(__file__))

pkg = "rust_civit_comfy_bindings-0.1.31-cp312-cp312-win_amd64.whl"

ensure_dependency(f"../target/wheels/")

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
    server = Process(
        target=start_server,
        args=(port,static_dir,str(comfy_path),),
        daemon=True
    )
    server.start()
        
    while True:
        ""
        
    print("Running server on another process with port:", port)

if __name__ == "__main__":
    main()
