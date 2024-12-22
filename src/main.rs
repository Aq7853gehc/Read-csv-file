use std::error::Error;
use std::env::args;

use csv;

fn main() {
    let args:Vec<_> = args().collect();
    if args.len() < 2 {
        eprint!("Usage: {} `<filePath>`",args[0]);
        return;
    }

    if let Err(e) = read_file_from(&*args[1]){
        eprint!("{}",e);
    }

}

fn read_file_from(path:&str)->Result<(),Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result?;
        println!("{:?}",record);
    } 
    Ok(())
}   