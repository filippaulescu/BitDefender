use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use serde::{Deserialize, Serialize};

type Term = String;
type DocumentId = String;
//type IndexData = HashMap<Term, HashSet<DocumentId>>;

struct IndexData {
    terms_to_docs: HashMap<Term, HashSet<DocumentId>>,
    idf: HashMap<Term, i64>
}

impl IndexData {
    // Constructor for `IndexData`
    fn new() -> Self {
        IndexData {
            terms_to_docs: HashMap::new(),
            idf: HashMap::new(),
        }
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    /// name of the zip archive
    name: DocumentId,
    /// list of files in the zip archive
    files: Vec<String>,
}

fn load_data(data_filename: &str) -> Result<IndexData, Box<dyn std::error::Error>> {
    let file = File::open(data_filename)?;
    let reader = BufReader::new(file);

    let mut index = IndexData::new();
    for line in reader.lines() {
        let line = line?;

        let fd: FileData = serde_json::from_str(&line)?;
        for file in fd.files {
            for term in file.split("/") {
                // index
                //     .entry(term.to_string())
                //     .or_insert(HashSet::new())
                //     .insert(fd.name.clone());
                if let Some(set) = index.terms_to_docs.get_mut(term) {
                    set.insert(fd.name.clone());
                } else {
                    let mut set = HashSet::new();
                    set.insert(fd.name.clone());
                    index.terms_to_docs.insert(term.to_string(), set);
                }
            }
        }
    }

    Ok(index)
}

fn run_search(data: &IndexData, terms: Vec<&str>) {
    let mut counter: HashMap<DocumentId, u64> = HashMap::new();
    for term in &terms {
        if let Some(docs) = data.terms_to_docs.get(*term) {
            for doc in docs {
                let x = counter.entry(doc.to_string()).or_insert(0);
                *x += 1;
            }
        }
    }

    /*
    for (doc, cnt) in &counter {
        println!("in {doc} found {cnt}/{} terms", terms.len());
    }
    println!("found {} documents", counter.len());
    */
    
    let mut scores: Vec<(DocumentId, f64)> = Vec::new();
    for (doc, cnt) in counter {
        let score = cnt as f64 / terms.len() as f64;
        scores.push((doc, score));
    }

    for (doc, score) in &scores {
        println!("Document {doc} has a score of {score:.2}");
    }
    println!("Found {} documents", scores.len());
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let data_filename = &args[1];

    let start = Instant::now();
    let data = load_data(&data_filename)?;
    println!("loaded data for {} terms", data.terms_to_docs.len());
    println!("elapsed time: {:?}", start.elapsed());

    let pair_count = data.terms_to_docs.iter().map(|(_, docs)| docs.len()).sum::<usize>();
    println!("there are {} term-docid pairs", pair_count);

    let start = Instant::now();
    let search = vec!["lombok", "AUTHORS", "README.md"];
    run_search(&data, search);
    println!("search took: {:?}", start.elapsed());
    Ok(())
}