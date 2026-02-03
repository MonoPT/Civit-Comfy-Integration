import argparse

def main(port: int, static_dir, comfy_path):    
    print("Running server on another process with port:", port)
    
    from rust_civit_comfy_bindings import start_server
    
    start_server(port, static_dir, comfy_path)
    
    while True: 
        pass

if __name__ == "__main__":    
    parser = argparse.ArgumentParser(description="Start server on a given port")
    parser.add_argument(
        "port",
        type=int,
        help="Port number to run the server on"
    )
    parser.add_argument(
        "static_folder",
        type=str,
        help="Frontend static build"
    )
    parser.add_argument(
        "comfy_path",
        type=str,
        help="ComfyUI path"
    )
        
    args = parser.parse_args()
    main(args.port, args.static_folder, args.comfy_path)
