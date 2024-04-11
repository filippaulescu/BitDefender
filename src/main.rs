use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use std::io::Write;

#[derive(Debug, serde::Serialize)]
struct FileData{
    name :String,
    filenames: Vec<String>,
}

/* 
fn list_zip_contents<R: io::Read + io::Seek>(
    reader: R,
    writer: &mut dyn io::Write,
) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        writeln!(writer, "Filename: {}", file.name())?;
    }

    Ok(())
}
*/
/*
fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
        //std::io::copy(&mut file, &mut std::io::stdout())?;
    }

    Ok(())
}
*/

fn list_zip_contents<R: io::Read + io::Seek>(
    reader: R
) -> zip::result::ZipResult<Vec <String>> {
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut filenames = Vec:: new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        filenames.push(file.name().to_string());
    }

    Ok(filenames)
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Obține argumentele din linia de comandă
    let args: Vec<String> = env::args().collect();

    // Primește calea către director ca primul argument
    let dir_path = &args[1];

    // Deschide fișierul pentru scriere
    let output_path = "output.txt";
    let mut zip_files: Vec<FileData> = Vec::new();

    // Iterează prin fiecare element din director
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        //INCANTATIE
        let filename = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        zip_files.push(FileData{ 
            name : filename,
            filenames: list_zip_contents(fs::File::open(entry.path())?)?
        });
    }

    //println!("{:?}",zip_files);
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for i in zip_files.iter() {
        
        serde_json::to_writer(&mut writer, &zip_files)?;
        writer.write_all(&[b'\n'])?;
        
    }


    Ok(())



}
