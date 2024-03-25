use clap::{value_parser, Arg, ArgMatches, Command};
use rand::distributions::{Distribution, Standard};
use rand::Rng; // 使用rand库来生成随机数
use regex::Regex; // 使用regex库来使用正则表达式
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};

// 获取命令行指令
pub fn get_args() -> ArgMatches {
    let matches = Command::new("MyApp")
        .version("1.0.0")
        .author("zg")
        .about("A mini quadratic operator generator")
        .arg(
            Arg::new("num")
                .short('n')
                .long("num")
                .value_parser(value_parser!(usize))
                .help("Use this parameter to control the number of generated topics"),
        )
        .arg(
            Arg::new("realm")
                .short('r')
                .long("realm")
                .value_parser(value_parser!(i32))
                .help("Use this parameter to control the range of values in the title"),
        )
        .arg(
            Arg::new("exercisefile")
                .short('e')
                .long("exercisefile")
                .value_parser(value_parser!(String))
                .help("Exercisefile name")
                .group("input")
                .requires("answerfile"),
        )
        .arg(
            Arg::new("answerfile")
                .short('a')
                .long("answerfile")
                .value_parser(value_parser!(String))
                .help("Answerfile name")
                .requires("input"),
        )
        .get_matches();

    matches
}

// 执行命令
pub fn run_args(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    if let (Some(num), Some(realm)) = (
        matches.get_one::<usize>("num"),
        matches.get_one::<i32>("realm"),
    ) {
        generate_expression(num, realm);
    }
    if let (Some(exercisefile), Some(answerfile)) = (
        matches.get_one::<String>("exercisefile"),
        matches.get_one::<String>("answerfile"),
    ) {
        check_answer(exercisefile, answerfile)?;
    }

    Ok(())
}

// 写入文件
fn write_to_file(filename: &str, content: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename).expect("创建文件失败!");
    let mut ansfile = File::create("Answers.txt").expect("创建文件失败!");
    for (index, exp) in content.iter().enumerate() {
        write!(file, "{}. {} = \n", index + 1, exp)?;
        write!(
            ansfile,
            "{}. {} = {}\n",
            index + 1,
            exp,
            calculate_fraction(exp)
        )?;
    }
    Ok(())
}

// 检查答案
fn check_answer(exercisefile: &String, answerfile: &String) -> io::Result<()> {
    let exercises = fs::read_to_string(exercisefile)?;
    let answers = fs::read_to_string(answerfile)?;

    // 使用正则表达式来消去题目前面的序号
    let re = Regex::new(r"\d+\. ").unwrap();

    let mut correct_count = 0;
    let mut incorrect_count = 0;
    let mut correct_indices = Vec::new();
    let mut incorrect_indices = Vec::new();

    for (index, (question, answer)) in exercises.lines().zip(answers.lines()).enumerate() {
        let tokens: Vec<_> = answer.split(' ').collect();
        let question = &question[0..question.trim().len() - 1];
        if let Some(captures) = re.captures(question) {
            let calculate_ans = calculate_fraction(
                &question[captures.get(0).unwrap().end()..]
                    .trim()
                    .to_string(),
            );
            if let Some(ans) = tokens.last() {
                if ans.to_string() == calculate_ans {
                    correct_count += 1;
                    correct_indices.push(index + 1);
                } else {
                    incorrect_count += 1;
                    incorrect_indices.push(index + 1);
                }
            }
        }
    }

    let grade_result = format!(
        "Correct: {} ({:?})\nWrong: {} ({:?})",
        correct_count, correct_indices, incorrect_count, incorrect_indices
    );

    fs::write("Grade.txt", grade_result)?;

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
                }
                Operator::Subtract => {
                    // 确保减法结果为正数
                    let numerator = match node {
                        ExpressionNode::Number(value) => value,
                        _ => panic!("获取随机值失败!"),
                    };
                    if numerator == 1 {
                        Box::new(ExpressionNode::Number(numerator))
                    } else {
                        Box::new(ExpressionNode::Number(
                            numerator - (rng.gen_range(1..numerator)),
                        ))
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

fn generate_right(
    rng: &mut impl Rng,
    node: &ExpressionNode,
    operator: &Operator,
    realm: i32,
) -> Box<ExpressionNode> {
    match operator {
        Operator::Divide => {
            // 确保除法结果为真分数
            let numerator = match *node {
                ExpressionNode::Number(value) => value,
                _ => panic!("获取随机值失败!"),
            };
            let denominator = rng.gen_range(1..realm) + numerator;
            Box::new(ExpressionNode::Number(denominator))
        }
        Operator::Subtract => {
            // 确保减法结果为正数
            let numerator = match *node {
                ExpressionNode::Number(value) => value,
                _ => panic!("获取随机值失败!"),
            };
            if numerator == 1 {
                Box::new(ExpressionNode::Number(numerator))
            } else {
                Box::new(ExpressionNode::Number(
                    numerator - (rng.gen_range(1..numerator)),
                ))
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
                    ExpressionNode::Number(_) => {
                        return format!("{} {} {}", left_str, op.to_string(), right_str)
                    }
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

// 添加左括号
fn add_braces(exp: &str) -> String {
    let operators = vec!['+', '-', '*', '/'];
    let pos1 = exp.chars().position(|c| operators.contains(&c));
    let pos2 = exp[pos1.unwrap() + 1..]
        .chars()
        .position(|c| operators.contains(&c));

    match (pos1, pos2) {
        (Some(index1), Some(index2)) => format!(
            "({}){}",
            &exp[0..(index1 + index2)],
            &exp[(index1 + index2)..]
        ),
        _ => exp.to_string(),
    }
}

fn generate_expression(num: &usize, realm: &i32) {
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

// 四则计算
fn calculate_fraction(exp: &String) -> String {
    let tokens: Vec<&str> = exp.split(&[' ', '(', ')']).collect();
    let (opening_index, closing_index) = find_parentheses_indices(&tokens);
    match (opening_index, closing_index) {
        (Some(op_index), Some(close_index)) => {
            let num1 = tokens[op_index + 1].parse::<i32>().unwrap();
            let num2 = tokens[close_index - 1].parse::<i32>().unwrap();
            let op1 = tokens[op_index + 2];
            let (op2, num3) = if op_index == 0 {
                (
                    tokens[close_index + 1],
                    tokens[close_index + 2].parse::<i32>().unwrap(),
                )
            } else {
                (tokens[op_index - 1], tokens[0].parse::<i32>().unwrap())
            };
            let ans = calculate(num1, num2, op1);
            if op1 == "/" {
                if op2 == "+" {
                    return format!("{}'{}", num3, ans);
                } else if op2 == "*" {
                    return calculate(num1 * num3, num2, op1);
                } else if op2 == "/" {
                    return calculate(num1, num2 * num3, op1);
                } else {
                    return calculate(num1 - (num2 * num3), num2, op1);
                }
            } else {
                let ans = ans.parse::<i32>().unwrap();
                return calculate(ans, num3, op2);
            }
        }
        _ => {
            let num1 = tokens[0].parse::<i32>().unwrap();
            let num2 = tokens[2].parse::<i32>().unwrap();
            let op1 = tokens[1];
            if tokens.len() < 5 {
                calculate(num1, num2, op1)
            } else {
                let num3 = tokens[4].parse::<i32>().unwrap();
                let op2 = tokens[3];
                if op1 == "/" {
                    let ans = calculate(num1, num2, op1);
                    if op2 == "+" {
                        return format!("{}'{}", num3, ans);
                    } else if op2 == "*" {
                        return calculate(num1 * num3, num2, op1);
                    } else if op2 == "/" {
                        return calculate(num1, num2 * num3, op1);
                    } else {
                        return calculate(num1 - (num2 * num3), num2, op1);
                    }
                } else if op1 == "*" {
                    return calculate(num1 * num2, num3, op2);
                } else if op2 == "/" {
                    if op1 == "-" {
                        return calculate((num1 * num3) - num2, num3, op2);
                    } else {
                        return format!("{}'{}", num1, calculate(num2, num3, op2));
                    }
                } else if op2 == "*" {
                    return calculate(num1, num2 * num3, op1);
                } else {
                    let ans = calculate(num1, num2, op1).parse::<i32>().unwrap();
                    calculate(ans, num3, op2)
                }
            }
        }
    }
}

fn calculate(num1: i32, num2: i32, op: &str) -> String {
    if op == "+" {
        (num1 + num2).to_string()
    } else if op == "-" {
        (num1 - num2).to_string()
    } else if op == "*" {
        (num1 * num2).to_string()
    } else {
        if num2 == 0 {
            panic!("除数不能为0");
        }
        let common_divisor = gcd(num1, num2);
        let num1 = num1 / common_divisor;
        let num2 = num2 / common_divisor;
        if num1 > num2 {
            format!("{}'{}/{}", num1 / num2, num1 % num2, num2)
        } else {
            format!("{}/{}", num1, num2)
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn find_parentheses_indices(vec: &Vec<&str>) -> (Option<usize>, Option<usize>) {
    let opening_parenthesis_index = vec.iter().position(|&item| item == "");
    let closing_parenthesis_index = vec.iter().rposition(|&item| item == "");

    (opening_parenthesis_index, closing_parenthesis_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    // 一个辅助函数，用于构建命令行参数
    fn make_args<'a>(args: Vec<&'a str>) -> Vec<OsString> {
        args.iter().map(|s| OsString::from(s)).collect()
    }

    #[test]
    fn test_get_args_num() {
        let args = make_args(vec![
            "MyApp", "-n", "10", // 设置生成主题的数量为10
        ]);

        let matches = Command::new("MyApp")
            .version("1.0.0")
            .author("zg")
            .about("A mini quadratic operator generator")
            .arg(
                Arg::new("num")
                    .short('n')
                    .long("num")
                    .value_parser(value_parser!(usize))
                    .help("Use this parameter to control the number of generated topics"),
            )
            // ... 其他参数
            .try_get_matches_from(args)
            .unwrap();

        assert_eq!(*matches.get_one::<usize>("num").unwrap(), 10);
    }

    #[test]
    fn test_get_args_realm() {
        let args = make_args(vec![
            "MyApp", "-r", "100", // 设置值的范围为-100到100
        ]);

        let matches = Command::new("MyApp")
            .version("1.0.0")
            .author("zg")
            .about("A mini quadratic operator generator")
            .arg(
                Arg::new("realm")
                    .short('r')
                    .long("realm")
                    .value_parser(value_parser!(i32))
                    .help("Use this parameter to control the range of values in the title"),
            )
            // ... 其他参数
            .try_get_matches_from(args)
            .unwrap();

        assert_eq!(*matches.get_one::<i32>("realm").unwrap(), 100);
    }

    #[test]
    fn test_get_args_exercise() {
        let args = make_args(vec!["MyApp", "-e", "exercise.txt", "-a", "answer.txt"]);

        let matches = Command::new("MyApp")
            .version("1.0.0")
            .author("zg")
            .about("A mini quadratic operator generator")
            .arg(
                Arg::new("exercisefile")
                    .short('e')
                    .long("exercisefile")
                    .value_parser(value_parser!(String))
                    .help("Exercisefile name")
                    .group("input")
                    .requires("answerfile"),
            )
            .arg(
                Arg::new("answerfile")
                    .short('a')
                    .long("answerfile")
                    .value_parser(value_parser!(String))
                    .help("Answerfile name")
                    .requires("input"),
            )
            .try_get_matches_from(args)
            .unwrap();

        assert_eq!(
            *matches.get_one::<String>("exercisefile").unwrap(),
            "exercise.txt"
        );
        assert_eq!(
            *matches.get_one::<String>("answerfile").unwrap(),
            "answer.txt"
        );
    }
}
