use civit::{
    Civit,
    ModelsOptions,
    ImagesOptions,
    TagsOptions,
    DownloadOptions
};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    //civit::test().await;
    /*let mut civit = Civit::new()
        .update_api_key("7577577650f1dfd4e270c231f7c105de")
        .set_auth_token("eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2R0NNIn0..2v2O5l9IiMjn_fuK.N19jcolPuTOiigD5DJ0ZyfZ1v8A_nzC0qmRexKyFTzn4mSfIURw_d_vh7yTwoIivGYi1N9-_n4x0oFLCnOteLmjfr5zniJCGLgx_VuoasNg8VX5jAwHsLhjNuyjT3P9DtFtlQueqs35t5kETy0wxwidKlRsNgHsvAp24JVgH04XCnliSmrK6_mBgnZ7VWFVucVwd2TLLvdoWXA5CiuIeEM6J2Ks2RQi_KG3lDIwjroWaMhaDxO_mrpRSV8AkDSu5jxRvO40d-jrLtfTVDwewoa6U4buNRBcTjgt6FcY_QICK_heL1enckAyd4foi65MvD93vBwE9FiHUGm3ZSxwKwsTW8XXPnkzPOrCHKpBjRlqeYmCLHJUF7igeeuLb_f-DgKosV81b7uDoWV9IH5TeYOjdHeIy-2F9BJqoAYSqHNnTE1RwPmGalRq-F8Wnb1E9uYRbnAWPFuRWYFVfvt9L-PQCWCLTCG_RkfJRbAyCFdJNfPCo2lZqF1NPSCRcBCT_G2tLJdkq91HZLWh7jj2cs5mJ3svooHJw9iXJ_-b4QCC3zeDQgOqqSEezqFEJUq4PpZknA6DMrR2sO4LDk_ZNPWzeNYk6NNHCpzuanIGbHrVlp_7wq3rb0fLs6B2r_lQMmZMPfnFxpAy9DCxQvO8jCvmUzHRPHIiTkhxwZLsnYQVjhN6FBCEgBDkYlqw0gQ6DuNDUpjK4c_9F2o4cgRXQqe313pyOcMnQuZUesNsBp4AxTGfNHohsNtAtMVjikDw2i6BD86Gpqtv_JEnYpFhGGzgMU7Kkrd_2V9YCmNFaDKcpTtZ84k8zSyYPbDNIHQTxVOTPQR9NsT5mCoxvhtPF4kg9CznZLuTU52HMQ8UlPaAcu6bbDZtepeLklhqGLhcgNK06pLJx8FgoKK3LEVZWzzIGL3VwTM48cdc1HfHpRijlXOheTKvBRGQK1kMrEcjjOeD3EOXtrWuuLU5LWZYmuUo4AXq0eP9Y44hk1cm02KGHBJbP0wSytfRQY8-UtcBM6E5GbNXfM2MkcDF0efLI1OCPGadeW4iBUF1IkWYI0Qd8Wz8tFGbswfF7BhsJNjF5K9mBQZIDcd6apjrz4i98gmD2bAYVtBQIQ6ndy4fNy30lCL_GJvex7k-_xgxhze1WFjEyjbcJdOdqq7yIwusEHJwOnBy-hqUhWw-IbujWLxogSIy7DE3K38xefehUaSKzgxyjwh3m1_Xm3gXL4oSVer3xYBwXZ88qRzP6EnEK1K6Dk2gv25GOOr1llV8dYfuwvHEbcYK-PHtgdmH3IALxB3CFa0FNfRa8tkppocQdGASB.aPgaSlCSH_XRxmP9Wl0Vyg"); 
        
    //let collection = civit.collections().await;
    
    //let c1 = collection.get(1).unwrap();
    
    //let models = civit.collection_data(c1.id).await;*/
    
    
    let app = Router::new().route("/", get(hello_world));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3090").await.unwrap();
    let addr = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{addr}");
    
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

//let user_data = civit.user_data().await.unwrap();
//dbg!(user_data.collections);
/* 
let civit = Civit::new().update_api_key("7577577650f1dfd4e270c231f7c105de"); 
//let response = civit.models(ModelsOptions::default().limit(1)).await;
//let response = civit.model_by_id(1102).await;  
//let response = civit.model_by_version(297064).await;   
//let response = civit.tags(TagsOptions::default().limit(1).page(50)).await; 
//let response = civit.download(297064, DownloadOptions::default()).await;

//dbg!(response);

let app = Router::new().route("/", get(hello_world));

let listener = tokio::net::TcpListener::bind("0.0.0.0:3090").await.unwrap();
let addr = listener.local_addr().unwrap().port();
println!("http://127.0.0.1:{addr}");

axum::serve(listener, app).await.unwrap();*/