set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

dev:
    cd backend/civitComfyBindings/; maturin build
    cd backend/test_python_server; uv run main.py

dev-frontend:
    cd front-end; npm run dev

publish-list:
    cd backend/target/wheels; ls
    
publish file token:
    cd backend/civitComfyBindings/; maturin build --release; 
    cd backend/target/wheels; uv publish {{file}} --token {{token}}