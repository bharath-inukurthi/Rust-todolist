//use std::path::Path;
//use std::env;
use std::io;
use std::fs::{OpenOptions};
//use std::io::{BufReader,BufWriter,Write};
use std::io::Write;

const FILENAME:&str ="todo.json";
static mut COUNTER:usize=0;

pub fn init()->io::Result<()>{


let mut file=OpenOptions::new().create(true).write(true).open(FILENAME)?;
writeln!(file,"{}",unsafe{COUNTER})?;
Ok(())
}
/*pub fn file(){

let file=Path::new("todo.json");
let path=file.display();
println!("path is {path}");
println!("pwd is {}",env::current_dir().expect("reson").display());
}*/

/*pub fn add_task(title:String)->io::Result<()>{

let mut file= OpenOptions::new().write(true).append(true).open(FILENAME)?;
writeln!(file,"[{},{}]",unsafe{counter},title)?;
println!("{}",unsafe{counter});
unsafe{
counter+=1;
}

Ok(())
}
*/
