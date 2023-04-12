extern crate asciidoctor;
extern crate html_diff; 

// AsciiDoc Processing
// -------------------
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use html_diff::get_differences;

use asciidoctor::{Error, Lexer, Parser};
use asciidoctor::html::{self, Generator};

// PDF Processing
// --------------
use std::process::Command;


fn main() {
    generate_html_and_cmp("example");
    generate_pdf();
}


fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);

    return path.exists();
}

fn read_file(filename: &str) -> String {
    let mut string = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut string).unwrap();
    string
}


fn generate_html_and_cmp(name: &str) {

    let filename = format!("{}.adoc", name);

    if !file_exists(&filename) {
        println!("File {} does not exist", filename);
        return;
    }
 
    let file = read_file(filename.as_str());
    let lexer = Lexer::new(file.as_bytes());
    let mut parser = Parser::new(lexer);
    let mut buffer = Vec::new();
    {
        let mut generator = Generator {};
        loop {
            let node = parser.node();
            match node {
                Ok(node) => html::gen(&mut generator, &node, &mut buffer).unwrap(),
                Err(Error::Eof) => break,
                Err(err) => panic!("cannot parse asciidoctor: {}", err),
            }
        }
    }

    // temp out buffer
    let out_buffer  = &buffer.clone();

    let result_file = read_file(&format!("output/{}.html", name));
    let html = String::from_utf8(buffer).unwrap();
    let differences = get_differences(&result_file, &html);
    if !differences.is_empty() {
        let mut diffs = "\n".to_string();
        for diff in differences {
            diffs += &diff.to_string();
            diffs += "\n";
        }
        println!("{}", diffs);
        assert!(false);
        //assert_eq!(result_file, html);
    }

    // write buffer to file
    let mut file = File::create("new.html").unwrap();
    file.write_all(&out_buffer).unwrap();

}

fn generate_pdf() {
    let html_file_path = "new.html";
    let pdf_file_path = "output.pdf";

    // ASSUMES wkhtmltopdf is installed and in the PATH!!!
    let mut command = Command::new("wkhtmltopdf");

    // Add input and output file paths as arguments
    command.arg(html_file_path).arg(pdf_file_path);

    // Run the command and wait for it to complete
    let output = command.output().expect("Failed to execute command");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
        return;
    }

    println!("PDF Conversion complete");
}