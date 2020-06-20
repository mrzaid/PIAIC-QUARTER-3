use async_std::task;
use surf; //as a waiter which gets url

async fn fetch(url: &str)-> Result<String, surf::Exception>{
    surf::get(url).recv_string().await
}
async fn exce(){
 match fetch ("https://www.google.com/").await{
     Ok(res) => println!("{}",res),
     Err(e) => println!("Error 404 {}",e),
 }
}
fn main(){
    task::block_on(exce())
}