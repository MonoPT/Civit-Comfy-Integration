from .ComfyUIFEExampleVueBasic import NODE_CLASS_MAPPINGS
import os
import nodes
from comfy_config import config_parser

from .manage_deps import ensure_dependency
from importlib.metadata import version

custom_node_dir = os.path.dirname(os.path.realpath(__file__))
print("==========================")

# Install and update dependencies
ensure_dependency("rust_civit_comfy_bindings")

project_config = config_parser.extract_node_configuration(custom_node_dir)

print("##", project_config.project.name)
print("rust-civit-comfy-bindings version is: ", version("rust-civit-comfy-bindings"))

js_dir = os.path.join(os.path.dirname(os.path.realpath(__file__)), "js")

nodes.EXTENSION_WEB_DIRS[project_config.project.name] = js_dir

__all__ = ['NODE_CLASS_MAPPINGS']

from server import PromptServer
from aiohttp import web
import aiohttp

routes = PromptServer.instance.routes

import socketserver
import socket

def is_port_free(port, host="127.0.0.1"):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex((host, port)) != 0

with socketserver.TCPServer(("localhost", 0), None) as server:
    port = server.server_address[1]
    
from rust_civit_comfy_bindings import sum_as_string, start_server

print(f"Civit backend server started on port {port}")

def reverse_proxy(request, rest_of_path: str):
    return web.json_response({"path": rest_of_path})
    
UPSTREAM = f"http://127.0.0.1:{port}"
    
print(f"Upstream: {UPSTREAM}")
    
@routes.get(r"/civit/{rest_of_path:.*}")
async def my_function(request: web.Request) -> web.StreamResponse:
    upstream_url = f"{UPSTREAM}{request.rel_url}"
    
    #print(upstream_url)

    async with aiohttp.ClientSession() as session:
        async with session.request(
            method=request.method,
            url=upstream_url,
            headers={k: v for k, v in request.headers.items()
                     if k.lower() != "host"},
            data=await request.read(),
            allow_redirects=False,
        ) as resp:

            proxy_resp = web.StreamResponse(
                status=resp.status,
                headers=resp.headers
            )

            await proxy_resp.prepare(request)

            async for chunk in resp.content.iter_chunked(8192):
                await proxy_resp.write(chunk)

            await proxy_resp.write_eof()
            return proxy_resp
    
@routes.get('/civit-comfy-port')
async def my_function(request):
    return web.json_response({"port": port})
    
file_p = os.path.dirname(os.path.abspath(__file__))

import subprocess
import sys
from pathlib import Path

static_dir = f"{file_p}/front-end/build"
comfy_path = str(Path(f"{file_p}../../../").resolve())

print(f"Static dir: {static_dir}")
print(f"Detected comfy path: {comfy_path}")

print("==========================")

subprocess.Popen(
    [sys.executable, f"{file_p}/server.py", f"{port}", {static_dir}, comfy_path],
    stdin=subprocess.DEVNULL,
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
    close_fds=True
)

