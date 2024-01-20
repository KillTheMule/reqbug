use async_trait::async_trait;
use tokio::io::Stdout;

use nvim_rs::{
  compat::tokio::Compat, create::tokio as create, Handler, Neovim, Value,
  exttypes::Buffer
};

#[derive(Clone)]
struct NeovimHandler();

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = Compat<Stdout>;

  async fn handle_notify(
    &self,
    _name: String,
    _args: Vec<Value>,
    _neovim: Neovim<Compat<Stdout>>
  ){}

}

#[tokio::main]
async fn main() {
  let handler: NeovimHandler = NeovimHandler{};

  let (nvim, io_handler) = create::new_parent(handler).await;

  //nvim.feedkeys("iab\ncd", "", true).await.unwrap();

  //let buf = Buffer { code_data: Value::from(1), neovim: nvim.clone() };
  let buf = nvim.get_current_buf().await.unwrap();

//  let arr = vec!["\n";1048577];
 // let s: String = arr.iter().copied().collect();

  for i in 0..20 { 
      //nvim.feedkeys(&s, "", true).await.unwrap();
      //buf.set_lines(-2, -1, false, vec![s]).await.unwrap();
      //buf.set_lines(-2, -1, false, vec!["a".to_string()]).await.unwrap();
      buf.set_name(&format!("a{i}")).await.unwrap();
      //std::thread::sleep(std::time::Duration::from_secs(2));
  }

  match io_handler.await {
    _ => {}
  }

}
