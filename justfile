set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

dev:
    cd backend/civitComfyBindings/; maturin build
    cd backend/test_python_server; uv run main.py

dev-frontend:
    cd front-end; npm run dev