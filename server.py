import argparse
import os

file_p = os.path.dirname(os.path.abspath(__file__))

static_dir = f"{file_p}/front-end/build"

def main(port: int):    
    print("Running server on another process with port:", port)
    
    from rust_civit_comfy_bindings import start_server
    
    start_server(port, static_dir)
    
    while True: 
        pass

if __name__ == "__main__":    
    print("Hello world")
    parser = argparse.ArgumentParser(description="Start server on a given port")
    parser.add_argument(
        "port",
        type=int,
        help="Port number to run the server on"
    )

    args = parser.parse_args()
    main(args.port)
