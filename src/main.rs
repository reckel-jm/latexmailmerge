use clap::Parser;
mod lib;

fn main() {    
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// The name of the LaTeX command which creates the the mail merge
        #[arg(long, default_value_t = String::from("\\merge"))]
        latex_command: String,
        /// First line to parse
        #[arg(short, long, default_value_t = 0)]
        firstline: u8,
        /// Delim
        #[arg(short, long, default_value_t = String::from(";"))]
        delim: String,
        /// Path of the CSV file
        filepath: String,
    }
    let options = Args::parse();

    let latex_string: String = lib::load_csv_file(&options.filepath, &options.delim, &options.firstline, &options.latex_command);
    println!("{}", latex_string);
}