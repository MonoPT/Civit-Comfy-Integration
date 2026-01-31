from .ComfyUIFEExampleVueBasic import NODE_CLASS_MAPPINGS
import os
import nodes
from comfy_config import config_parser

from .manage_deps import ensure_dependency

custom_node_dir = os.path.dirname(os.path.realpath(__file__))
print("==========================")

project_config = config_parser.extract_node_configuration(custom_node_dir)

print(project_config.project.name)

print("==========================")

js_dir = os.path.join(os.path.dirname(os.path.realpath(__file__)), "js")

nodes.EXTENSION_WEB_DIRS[project_config.project.name] = js_dir

__all__ = ['NODE_CLASS_MAPPINGS']

from server import PromptServer
from aiohttp import web
routes = PromptServer.instance.routes

# Install and update dependencies
ensure_dependency("rust_civit_comfy_bindings")

from importlib.metadata import version

print("rust-civit-comfy-bindings version is: ", version("rust-civit-comfy-bindings"))

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

from rust_civit_comfy_bindings import sum_as_string, start_server

print(f"Civit backend server started on port {port}")

## Catch all route - rev proxy
@routes.get('/my_new_path')
async def my_function(request):

    return web.json_response({"teste": True})
    
@routes.get('/civit-comfy-port')
async def my_function(request):
    return web.json_response({"port": port})
    
file_p = os.path.dirname(os.path.abspath(__file__))

import subprocess
import sys

subprocess.Popen(
    [sys.executable, f"{file_p}/server.py", f"{port}"],
    stdin=subprocess.DEVNULL,
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
    close_fds=True
)

