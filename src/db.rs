use rusqlite::Connection;
use anyhow::{Result,Context,bail};

pub fn init_db() ->Result<()>{

let conn= Connection::open("todo.db").context("Error in creating db")?;
conn.execute("CREATE TABLE IF NOT EXISTS todos(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        task TEXT NOT NULL,
        done BOOLEAN NOT NULL DEFAULT 0)",
[],).context("error in Creating table")?;
Ok(())
}

pub fn add_task(conn:&Connection, title:String)->Result<()>{
conn.execute("INSERT INTO todos (task) VALUES (?1)",[&title],)
    .with_context(||format!("Failed to insert the task {}",&title))?;
Ok(())
}

pub fn update_status(conn:&Connection,id:usize)->Result<()>{

let r= conn.execute("UPDATE todos SET done=1 WHERE id=(?1)",[&id]).with_context(||format!("Failed to update task {}",&id))?; 
if r==0{
    bail!("No task with id : {}",id);
}
Ok(())
}


pub fn del_task(conn:&Connection,id:usize)->Result<()>{
let r=conn.execute("DELETE FROM todos WHERE id=(?1)",[&id]).with_context(||format!("Failed to delete task {}",&id))?;
if r==0{
    bail!("No task with id : {}",id );
}
Ok(())
}
pub fn list_tasks(conn:&Connection)->Result<()>{
let mut stm=conn.prepare("SELECT id,task,done FROM todos").context("Failed to Parse the SQL")?;
let results=stm.query_map([],|row|{
    Ok((
    row.get::<_,usize>(0)?,
    row.get::<_,String>(1)?,
    row.get::<_,bool>(2)?,))}).context("failed to fetch DB")?;
for task in results{
    let (id,task,done)=task.context("failed to load results")?;
    let status= if done {"X"} else {" "};
    println!("id:{id} | [{status}] | {task}"); }
Ok(())
}
