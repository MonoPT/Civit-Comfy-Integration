fn main() {
    use tokio::runtime::Runtime;
    
    let rt = Runtime::new()
        .unwrap();
    
    use std::fs;
    
    
    
    let front_end = fs::canonicalize("../../front-end/build").unwrap();
    
    let civit_folder = fs::canonicalize("./teste/").unwrap();
        
    rt.block_on(app::start_civit_frontend_server(3090, front_end.to_string_lossy().to_string().as_str(), civit_folder.to_string_lossy().to_string().as_str()));
}
