use std::error::Error;
use std::fs::File;
use std::io::Write;

use eval::eval;
use clap::{value_parser, Arg, ArgMatches, Command};
use rand::Rng; // 使用rand库来生成随机数
use rand::distributions::{Distribution, Standard};

// 获取命令行指令
pub fn get_args() -> ArgMatches {
    let matches = Command::new("MyApp")
        .version("1.0.0")
        .author("zg")
        .about("A mini quadratic operator generator")
        .arg(
            Arg::new("num").short('n').long("num").value_parser(value_parser!(usize))
            .help("Use this parameter to control the number of generated topics")
        )
        .arg(
            Arg::new("realm").short('r').long("realm").value_parser(value_parser!(i32))
            .help("Use this parameter to control the range of values in the title")
        )
        .arg(
            Arg::new("exercisefile").short('e').long("exercisefile").value_parser(value_parser!(String))
            .help("Exercisefile name")
            .group("input")
            .requires("answerfile")
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
pub fn run_args(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    if let (Some(num), Some(realm)) = (matches.get_one::<usize>("num"), matches.get_one::<i32>("realm")) {
        let mut rng = rand::thread_rng();
        let mut vecexp: Vec<_> = Vec::new();
        for _ in 0..*num {
            // 构建一个包含最多三个运算符的表达式树
            let expression_tree = build_expression_tree(&mut rng, 2, *realm);

            // 将表达式树转换为字符串
            let mut expression_str = expression_tree.to_string();

            if expression_str.chars().last().unwrap() != ')' {
                if rand::thread_rng().gen_bool(0.5) {
                    expression_str = add_braces(&expression_str);
                }
            }
            vecexp.push(expression_str);
        }
        write_to_file("Exercises.txt", vecexp).expect("写入文件失败!");
    }
    if let Some(exercisefile) = matches.get_one::<String>("exercisefile") {
        println!("Value for exercisefile: {exercisefile}");
    }

    if let Some(answerfile) = matches.get_one::<String>("answerfile") {
        println!("Value for answerfile: {answerfile}");
    }

    Ok(())
}

fn write_to_file(filename: &str, content: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename).expect("创建文件失败!");
    let mut ansfile = File::create("Answers.txt").expect("创建文件失败!");
    for (index, exp) in content.iter().enumerate() {
        write!(file, "{}. {} = \n", index + 1, exp)?;
        write!(ansfile, "{}. {} = {}\n", index + 1, exp, eval(exp).unwrap())?;
    }
    Ok(())
}

// 定义一个枚举来表示四则运算符
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// 为Operator实现一个方法，用于将其转换为字符串表示
impl Operator {
    fn to_string(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

// 定义一个枚举来表示表达式节点
enum ExpressionNode {
    Number(i32),
    Operation(Box<ExpressionNode>, Operator, Box<ExpressionNode>),
}

// 实现Distribution trait，以便我们可以使用rand::random()来生成Operator
impl Distribution<Operator> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Operator {
        match rng.gen_range(0..4) {
            0 => Operator::Add,
            1 => Operator::Subtract,
            2 => Operator::Multiply,
            _ => Operator::Divide,
        }
    }
}

// 递归函数来构建表达式树
fn build_expression_tree(rng: &mut impl Rng, depth: usize, realm: i32) -> ExpressionNode {
    let node = ExpressionNode::Number(rng.gen_range(1..realm));
    let operator = rng.sample(Standard);

    if depth > 1 {
        // 随机决定是否继续添加运算符
        if rng.gen_bool(0.5) {
            let right = match operator {
                Operator::Divide => {
                    // 确保除法结果为真分数
                    let numerator = match node {
                        ExpressionNode::Number(value) => value,
                        _ => panic!("获取随机值失败!"),
                    };
                    let denominator = rng.gen_range(1..realm) + numerator;
                    Box::new(ExpressionNode::Number(denominator))
                },
                Operator::Subtract => {
                    // 确保减法结果为正数
                    let numerator = match node {
                        ExpressionNode::Number(value) => value,
                        _ => panic!("获取随机值失败!"),
                    };
                    if numerator == 1 {
                        Box::new(ExpressionNode::Number(numerator))
                    } else {
                        Box::new(ExpressionNode::Number(numerator - (rng.gen_range(1..numerator))))     
                    }
                }
                _ => Box::new(build_expression_tree(rng, depth - 1, realm)),
            };
            ExpressionNode::Operation(Box::new(node), operator, right)
        } else {
            // 如果不添加运算符，直接返回
            let right = generate_right(rng, &node, &operator, realm);
            ExpressionNode::Operation(Box::new(node), operator, right)
        }
    } else {
        // 如果达到最大深度，直接返回
        let right = generate_right(rng, &node, &operator, realm);
        ExpressionNode::Operation(Box::new(node), operator, right)
    }
}

fn generate_right(rng: &mut impl Rng, node: &ExpressionNode, operator: &Operator, realm: i32) -> Box<ExpressionNode> {
    match operator {
        Operator::Divide => {
            // 确保除法结果为真分数
            let numerator = match *node {
                ExpressionNode::Number(value) => value,
                _ => panic!("获取随机值失败!"),
            };
            let denominator = rng.gen_range(1..realm) + numerator;
            Box::new(ExpressionNode::Number(denominator))
        },
        Operator::Subtract => {
            // 确保减法结果为正数
            let numerator = match *node {
                ExpressionNode::Number(value) => value,
                _ => panic!("获取随机值失败!"),
            };
            if numerator == 1 {
                Box::new(ExpressionNode::Number(numerator))
            } else {
                Box::new(ExpressionNode::Number(numerator - (rng.gen_range(1..numerator))))     
            }
        }
        _ => Box::new(ExpressionNode::Number(rng.gen_range(1..realm))),
    }
}

// 实现一个方法来将表达式节点转换为字符串
impl ExpressionNode {
    fn to_string(&self) -> String {
        match self {
            ExpressionNode::Number(num) => num.to_string(),
            ExpressionNode::Operation(left, op, right) => {
                let left_str = left.to_string();
                let right_str = right.to_string();
                match right.as_ref() {
                    ExpressionNode::Number(_) => return format!("{} {} {}", left_str, op.to_string(), right_str),
                    _ => (),
                }
                if rand::thread_rng().gen_bool(0.5) {
                    format!("{} {} ({})", left_str, op.to_string(), right_str)
                } else {
                    format!("{} {} {}", left_str, op.to_string(), right_str)
                }
            }
        }
    }
}

fn add_braces(exp: &str) -> String {
    let operators = vec!['+', '-', '*', '/'];
    let pos1 = exp.chars().position(|c| operators.contains(&c));
    let pos2 = exp[pos1.unwrap() + 1..].chars().position(|c| operators.contains(&c));

    match (pos1, pos2) {
        (Some(index1), Some(index2)) => format!("({}){}", &exp[0..(index1 + index2)], &exp[(index1 + index2)..]),
        _ => exp.to_string()
    }
}