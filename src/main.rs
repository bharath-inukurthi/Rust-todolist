use clap::{Parser,Subcommand};
use rusqlite::Connection;
use anyhow::{Context,Result};
mod file;
mod db;

#[derive(Parser)]
#[command(name="todo")]
#[command(about="A simple cli app to learn rust ")]

struct Cli{
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands{
    Add{
        task: String,
    },
    List,
    Done{
        id:usize,
    },
    Remove{
        id:usize,
    },
}

fn main()->Result<()>{
    //file::file();
    db::init_db()?;
    let conn = Connection::open("todo.db").context("DB connection failed")?;
    file::init()?;
    let cli =Cli::parse();
    match cli.command{
        Commands::Add{task}=>{
            println!("Adding task {task}");
            db::add_task(&conn,task)?;
        }
        Commands::List=>{
            db::list_tasks(&conn)?;
            println!("Listing tasks");
        }
        Commands::Remove{id}=>{
            db::del_task(&conn,id)?;
            println!("Removed {id}");
        }
        Commands::Done{id}=>{
            db::update_status(&conn,id)?;
            println!("marking {id} as done");
        }
    }
    Ok(())
}


