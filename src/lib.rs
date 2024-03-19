use clap::{value_parser, Arg, ArgMatches, Command};


// 获取命令行指令
pub fn get_args() -> ArgMatches {
    let matches = Command::new("MyApp")
        .version("1.0.0")
        .author("zg")
        .about("A mini quadratic operator generator")
        .arg(
            Arg::new("num").short('n').long("num").value_parser(value_parser!(u8))
            .help("Use this parameter to control the number of generated topics")
        )
        .arg(
            Arg::new("realm").short('r').long("realm").value_parser(value_parser!(u8))
            .help("Use this parameter to control the range of values in the title")
        )
        .arg(
            Arg::new("exercisefile").short('e').long("exercisefile").value_parser(value_parser!(String))
            .help("Exercisefile name")
            .group("input")
        )
        .arg(
            Arg::new("answerfile").short('a').long("answerfile").value_parser(value_parser!(String))
            .help("Answerfile name")
            .requires("input")
        )
    .get_matches();

    matches
}

// 执行命令
pub fn run_args(matches: ArgMatches) {
    if let Some(num) = matches.get_one::<u8>("num") {
        println!("Value for num: {num}");
    }

    if let Some(realm) = matches.get_one::<u8>("realm") {
        println!("Value for realm: {realm}");
    }

    if let Some(exercisefile) = matches.get_one::<String>("exercisefile") {
        println!("Value for exercisefile: {exercisefile}");
    }

    if let Some(answerfile) = matches.get_one::<String>("answerfile") {
        println!("Value for answerfile: {answerfile}");
    }
}