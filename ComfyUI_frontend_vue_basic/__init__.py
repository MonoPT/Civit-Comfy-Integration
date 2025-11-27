from .ComfyUIFEExampleVueBasic import NODE_CLASS_MAPPINGS
import os
import nodes
from comfy_config import config_parser

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
@routes.post('/my_new_path')
async def my_function(request):
    the_data = await request.post()
    # the_data now holds a dictionary of the values sent
    #MyClass.handle_my_message(the_data)
    return web.json_response({"teste": True})