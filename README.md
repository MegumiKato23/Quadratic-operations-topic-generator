# Quadratic-operations-topic-generator

## 使用方法

1. 使用 `-n` 参数来控制生成题目的数量。例如，运行以下命令：

    ```bash
    cargo run -- -n 10
    ```

    将会生成 10 个题目。

2. 使用 `-r` 参数来控制题目中数值（自然数、真分数和真分数分母）的范围。例如，运行以下命令：

    ```bash
    cargo run -- -r 10
    ```

    将会生成数值在 10 以内（不包括 10）的四则运算题目。这个参数可以设置为 1 或其他自然数。这个参数是必须的，如果没有提供，程序将会报错并显示帮助信息。

3. 程序还支持对给定的题目文件和答案文件进行答案的判断和数量统计。使用以下参数运行程序：

    ```bash
    cargo run -- -e <exercisefile>.txt -a <answerfile>.txt
    ```

    其中 `<exercisefile>.txt` 和 `<answerfile>.txt` 分别是题目文件和答案文件的名称。统计结果将会输出到文件 `Grade.txt` 中。
