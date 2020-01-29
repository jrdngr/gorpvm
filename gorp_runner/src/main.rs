use std::path::PathBuf;
use structopt::StructOpt;
use gorp_cpu::Cpu;

fn main() {
    let options = Options::from_args();
    
    let run_message = format!("Running: {:?}", &options.path);
    let terminated_message = format!("{:?} terminated successfully", &options.path);
    let padding_len = usize::max(run_message.len(), terminated_message.len());

    println!("{0:-<1$}", "", padding_len + 4);
    println!("| {0:^1$} |", &run_message, padding_len);
    println!("{0:-<1$}", "", padding_len + 4);
    println!("");
    
    
    let mut cpu = Cpu::new();
    cpu.load_assembly_file(&options.path);
    cpu.run();

    
    println!("");
    println!("{0:-<1$}", "", padding_len + 4);
    println!("| {0:^1$} |", &terminated_message, padding_len);
    println!("{0:-<1$}", "", padding_len + 4);
}

#[derive(StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}
