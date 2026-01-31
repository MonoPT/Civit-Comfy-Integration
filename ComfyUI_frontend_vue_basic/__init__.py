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

from rust_civit_comfy_bindings import sum_as_string

@routes.get('/my_new_path')
async def my_function(request):

    #result = sum_as_string(20, 30)
    return web.json_response({"teste": True})