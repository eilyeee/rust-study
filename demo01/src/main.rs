pub mod fib;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let url = "http://www.rust-lang.org/";
  let output = "rust.md";
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url)?.text()?;
  
  println!("Converting html to markdown。。。");

  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes())?;

  println!("Converting markdown has benn saved in {}", output);
  fib::main();

  Ok(())
}
