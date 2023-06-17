mod client;

fn main() {
   let one = client::connect();
   println!("{:?}", one);
}