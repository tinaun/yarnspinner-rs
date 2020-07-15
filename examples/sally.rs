use yarnspinner::{DialogRunner, Error};

const SALLY_SRC: &str = include_str!("sally.yarn");

fn run() -> Result<(), Error<'static>>{

    let mut runner = DialogRunner::new();
    runner.load_yarn_file(SALLY_SRC)?;

    while let Some(e) = runner.next_event() {
        println!("dialog event: {:?}", e);
    }


    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}