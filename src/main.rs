use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    /// name of the zip archive
    name: String,
    /// list of files in the zip archive
    files: Vec<String>,
}

use std::collections::HashMap;
use std::collections::HashSet;
type Term = String;
type DocumentID = String;
type IndexType = HashMap<Term, HashSet<DocumentID>>;

fn load_data(data_filename: &str) -> Result<Vec<FileData>, Box<dyn std::error::Error>> {
    let file = File::open(data_filename)?;
    let reader = BufReader::new(file);

    let mut data: Vec<FileData> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        data.push(serde_json::from_str(line)?);
    }

    let mut index = IndexType::new(); 
    for fd in data.iter_mut(){
        //trebuie split-uit dupa /, am vazut ceva cu .split("/")
        for term in fd.files.iter_mut(){
            //println!("Filename: {}\n", entry2.to_string());
            let x = index.get_mut(term);
            if let Some(z ) = x {
                z.insert(fd.name.clone());
            } else {
                let mut s = HashSet::new();
                s.insert(fd.name.clone());
                index.insert(term.to_string(),  s);
            }
        }
    }

    Ok(data)
}

fn run_search(data: &IndexType, terms: Vec<&str>){
    let mut counter: HashMap<DocumentID, u64> = HashMap::new();
    for term in &terms{
            if let Some(docs) = data.get(*term){

            }else{
                
            }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    use std::time::Instant;
    let start = Instant::now();
    let data_filename = &args[1];
    let data = load_data(&data_filename)?;
    
    println!("loaded data for {} files", data.len());
    println!("time {:?} ", start.elapsed());

    Ok(())
}