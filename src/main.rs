mod api;
mod filters;

use crate::api::{Civit, ImagesOptions, ModelsOptions};



#[tokio::main]
async fn main() {
    let civit = Civit::new("7577577650f1dfd4e270c231f7c105de"); 
    
    let response = civit.models(
        ModelsOptions::default()
            .limit(1)
    ).await;
        
    println!("{response:#?}");    
}
