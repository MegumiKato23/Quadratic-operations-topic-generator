use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let matches = miniopgenerator::get_args();

    miniopgenerator::run_args(matches)
}