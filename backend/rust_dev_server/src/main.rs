fn main() {
    use tokio::runtime::Runtime;
    
    let rt = Runtime::new()
        .unwrap();
    
    use std::fs;
    
    let cwd = std::env::current_dir().unwrap();
    
    let front_end = fs::canonicalize("../front-end/build").unwrap();
    let civit_folder = fs::canonicalize("./rust_dev_server/teste/").unwrap();
    
    
    dbg!(&front_end);
    dbg!(&civit_folder);
    
    rt.block_on(app::start_civit_frontend_server(3090, front_end.to_string_lossy().to_string().as_str(), civit_folder.to_string_lossy().to_string().as_str()));
}
