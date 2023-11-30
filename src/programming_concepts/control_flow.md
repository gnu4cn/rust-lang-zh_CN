# 控制流程（Control Flow）

根据条件是否为真，来运行某些代码，或者在条件为真时重复运行某些代码的能力，是绝大多数语言的根基。实现Rust代码执行流程控制最常见的结构，即是 `if` 表达式和循环。

## `if` 表达式

`if` 表达式实现了根据条件对代码进行分支。提供到一个条件，然后就表明，“在该条件满足时，运行这个代码块。在条件不满足时，就不要运行这个代码块。”

请在 `projects` 目录下，创建一个新的、名为 `branches` 的项目，来探索这个 `if` 表达式。在 `src/main.rs` 文件中，输入以下代码：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println! ("条件为真");
    } else {
        println! ("条件为假");
    }
}
```

全部 `if` 表达式，都是以关键字 `if` 开头的，接着的是一个条件。在此示例中，那个条件就变量 `number` 是否小于 `5` 进行检查。是把要在条件为真时立即执行的代码块，放在条件之后、一对花括号里头。`if`表达式中与那些条件相关联的代码块，有时也叫做 *支臂（arms）*，这与在第 2 章的 [将猜数与秘密数字比较](Ch02_Programming_a_Guessing_Game.md#将猜数与秘数相比较) 小节中讨论过的 `match` 表达式中的支臂一样。

作为可选项，还可以包含一个 `else` 表达式，即这里做的那样，从而给到程序一个替代性的、将在条件求解结果为 `false` 时执行的代码块。在未提供`else`表达式，且条件为 `false` 时，程序将直接跳过那个 `if` 代码块，而前往接下来的代码处。

尝试运行此代码；将看到下面的输出：

```console
$ cargo run                                                            ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/branches`
条件为真
```

下面来试着将 `number` 的值修改为一个令到该条件为 `false` 的值，看看会发生什么：

```rust
    let number = 7;
```

再运行这个程序，然后看看输出：

```console
$ cargo run                                                            1 ✘
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
条件为假
```

还值得注意的是，此代码中的条件 *必须* 是个 `bool` 类型。在条件不是 `bool` 类型时，就会收到错误。比如，尝试运行下面的代码：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number {
        println! ("数字是 3");
    }
}
```

这次的 `if` 条件求解为一个 `3` 的值，进而 Rust 抛出一个错误：

```console
$ cargo run                     ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

该错误表明 Rust 期望得到一个 `bool` 值但得到的是个整数。与诸如 Ruby 和 JavaScript 那样的语言不同，Rust 不会自动将非布尔值转换为布尔值。必须显式地且一直提供给 `if` 一个布尔值作为其条件。比如希望那个 `if` 代码块，仅在某个数字不等于 `0` 的时候运行，那么就可以将这个 `if` 表达式修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println! ("数字为非零数");
    }
}
```

运行此代码，就会打印出 `数字为非零数`。


### 用 `else if` 来处理多个条件

通过在 `else if` 表达式中，结合 `if` 和 `else`，就可以使用多个条件。比如：

文件名：`src/main.rs`

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println! ("数字可被 4 整除");
    } else if number % 3 == 0 {
        println! ("数字可被 3 整除");
    } else if number % 2 == 0 {
        println! ("数字可被 2 整除");
    } else {
        println! ("数字不可被 4、3 或 2 整除");
    }
}
```

此程序有着其可接收的四个可能路径。在运行他时，就会看到下面的输出：

```console
$ cargo run                                                                           101 ✘
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
数字可被 3 整除
```

在该程序执行时，就会依次检查各个 `if` 表达式，并执行那第一个条件成立的代码体。请注意即便 `6` 是可被 `2` 整除的，却并未看到输出 `数字可被 2 整除`，也没看到那个 `else` 代码块的 `数字不能被 4、3 或 2 整除` 文字。这是由于 Rust 只执行了第一个为真条件下的代码块，而一旦他发现了一个，就在不会检查剩下的那些条件了。

使用太多的 `else if` 表达式，就会让代码杂乱无章，因此在有多于一个这样的表达式时，或许就应对代码进行重构了。第 6 章描述了针对这样情况的一种强大的 Rust 分支结构，名为`match` 模式匹配。


### 在 `let` 语句中使用 `if` 关键字

由于 `if` 是个表达式，那么就可以在 `let` 表达式的右边使用他，来将其结算结果，赋值给某个变量，如下面的清单 3-2 所示：

文件名：`src/main.rs`

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println! ("number 的值为：{}", number);
}
```

*清单 3-2：将`if` 表达式的结果赋值给某个变量*

其中的 `number` 变量，就会被绑定到那个 `if` 表达式的计算结果上。运行此代码看看会发生什么：

```console
$ cargo run                                                                               ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/branches`
number 的值为：5
```

请记住代码块会求解到其中最后一个表达式的值，且数字本身也就是表达式。在此示例中，整个 `if` 表达式的值，是取决于会执行到哪个代码块的。这就意味着那些该 `if` 表达式各个支臂的、具备作为 `if` 表达式运算结果的那些值，必须要是相同类型；在清单 3-2 中，`if` 支臂和 `else` 支臂的运算结果，就都是 `i32` 类型的整数。若这些类型不匹配，就如下面的示例中那样，则会收到错误：

文件名：`src/main.rs`

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println! ("number 的值为：{}", number);
}
```

在尝试编译这段代码时，就会收到错误。其中的 `if` 与 `else` 支臂的值类型不兼容，同时 Rust 还准确标明了在程序中何处发现的该问题：

```console
$ cargo run                                                                               ✔
   Compiling branches v0.1.0 (/home/peng/rust-lang/projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

`if` 代码块中的表达式，求解为整数，而`else` 代码块中的表达式求解为了字符串。由于变量必须有着单一类型，且 Rust 需要知道在运行时变量 `number` 的类型是什么，那么显然这代码是不会工作的。清楚 `number` 的类型，就允许编译器在所有用到 `number` 的地方，验证其类型的有效性。而如果只有在运行时才确定出 `number` 的类型，那么 Rust 就无法做到这一点；若编译器务必要对全部变量的多个假定类型进行跟踪，那么编译器就会更为复杂，且做到更少代码保证。

## 循环下的重复

多次执行某个代码块常常是有用的。对于这类任务，Rust 提供了数种 *循环（loops）*，所谓循环，是指会贯通执行循环体里头的代码到结束，并随后立即回到开头开始执行。首先构造一个名为 `loops` 的新项目，来进行这些循环的实验。

Rust 有着三种循环：`loop`、`while` 及 `for`。接下来就要各个进行尝试。


### 用 `loop` 关键字对代码进行重复

`loop` 关键字告诉 Rust 去一直一遍又一遍执行代码块，抑或直到显式地告诉他停下来为止。

作为示例，将 `loops` 目录中的 `src/main.rs` 文件修改为下面这样：

文件名：`src/main.rs`

```rust
fn main() {
    loop {
        println! (”再次！“);
    }
}
```

在运行这个程序时，就会看到一遍又一遍地持续打印出 `再次！`，知道手动停止这个程序为止。大多数终端程序，都支持键盘快捷键 `ctrl-c` 来中断某个卡在无尽循环中的某个程序。来尝试一下：

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
再次！
再次！
再次！
再次！
^C再次！
```

其中的符号 `^C` 表示按下 `ctrl-c` 的地方。在那个 `^C` 之后，可能会也可能不会看到 `再次！` 被打印出来，取决于程序接收到中断信号时，代码在循环中的何处。

幸运的是，Rust 还提供了一种运用代码来跳出循环的方式。可在循环中放置 `break` 关键字，而告诉程序在何时结束执行这个循环。还记得在第 2 章的 [猜对数字后退出程序](Ch02_Programming_a_Guessing_Game.md#猜对后的退出) 小节，就在那个猜数游戏中这样做了，在通过猜到正确数字而赢得游戏时退出那个程序。

在那个猜数游戏中，还使用了 `continue` 关键字，循环中的 `continue` 关键字，告诉程序去跳过循环本次迭代的其余全部代码，而前往下一次迭代。

在有着循环里头的循环时，那么 `break` 与 `continue` 就会应用在他们所在点位处的最内层循环（if you have loops within loops, `break` and `continue` apply to the innermost loop at that point）。可选择在某个循环上指定一个 *循环标签（loop label）*，这样就可以与 `break` 或 `continue` 结合使用，来指明这些关键字是要应用到打上标签的循环，而不再是那最里层的循环了。下面就是一个有着两个嵌套循环的示例：

```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println! ("计数 = {}", count);
        let mut remaining = 10;

        loop {
            println! ("剩余 = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println! ("最终计数 = {}", count);
}
```

其中的外层循环有着标签 `'counting_up`，同时他将从 `0` 计数到 `2`。而其中的内层循环不带标签，会从 `10` 计数到 `9`。其中的第一个未指定标签的 `break` 语句，将只会退出那个内部循环。而那个 `break 'counting_up;` 语句，则会将外层循环退出。此代码会打印出：

```console
$ cargo run                                                                           INT ✘
   Compiling loops v0.1.0 (/home/peng/rust-lang/projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/loops`
计数 = 0
剩余 = 10
剩余 = 9
计数 = 1
剩余 = 10
剩余 = 9
计数 = 2
剩余 = 10
最终计数 = 2
```

### 自循环返回值

**Returning Values from Loops**

`loop` 的一个用途，即是对一个明知会失败的操作进行重试，比如检查某个线程是否已完成他的作业。还可能需要将那个操作的结果，从循环传出来给代码的其余部分。要实现这一点，可将想要返回的值，添加在用于停止该循环的 `break` 表达式之后；那个值就会被返回到该循环的外面，进而就可以用到那个值了，如下所示：

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println! ("结果为：{}", result);
}
```

在这个循环之前，这里声明了一个名为 `counter` 的变量，并将其初始化为 `0`。随后声明了一个名为 `result` 变量，来保存从该循环所返回的值。在该循环的每次迭代上，是要给 `counter` 变量加上 `1` 的，并随后检查那个计数器是否等于 `10`。在计数器等于 `10` 的时候，就使用有着值 `counter * 2` 的 `break` 关键字。在该循环之后，使用了一个分号来结束将值 `counter * 2` 赋值给 `result` 的那个语句。最后，这里打印出了在 `result` 里的值，即这个示例中的 `20`。


### 用于在多个循环之间消除歧义的循环标签

**Loop Labels to Disambiguate Between Multiple Loops**

如果咱们有着一些循环内的循环，`break` 和 `continue` 会应用于最内层循环的其所在之处。咱们可以选择性地，在某个循环上指定出，随后可与 `break` 或 `continue` 一起使用的 *循环标签，loop label*，从而指明这些关键字，会应用于带标签的循环，而不是最内层的循环。循环标签必须以单引号开头。下面是个有着两个嵌套循环的示例：


```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println! ("count = {count}");
        let mut remaining = 10;

        loop {
            println! ("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println! ("End count = {count}");
}
```


其中外层的循环，有着标签 `'counting_up`，而其将从 0 计数到 2。不带标签的内层循环，将从 10 递减计数到 9。第一个未指定标签的 `break`，只会退出那个内层循环。`break 'counting_up;` 语句将退出外循环。这段代码将打印：


```console
$ cargo run
   Compiling loop_label v0.1.0 (C:\tools\msys64\home\Lenny.Peng\rust-lang-zh_CN\projects\loop_label)
    Finished dev [unoptimized + debuginfo] target(s) in 0.95s
     Running `target\debug\loop_label.exe`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

### 使用 `while` 的条件循环

程序经常会对循环里的条件进行求值。当条件为真时，该循环就运行。在条件不再为真时，程序就调用 `break`，把循环停下来。使用 `loop`、`if`、`else` 与 `break` 来实现与此相似的行为，是可能的；若愿意这样做，现在就可以在程序中尝试一下。不过由于这种模式如此常见，以至于 Rust 为此而有了一个内建的语言结构，那就是叫做 `while` 的循环。在下面的清单 3-3 中，就使用了 `while` 来将该程序循环三次，每次都倒计数，并随后在循环结束之后，打印出一条消息而退出。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println! ("{}!", number);

        number -= 1;
    }

    println! ("点火！！");
}
```

*清单 3-3：使用 `while` 循环在条件保持为真期间运行代码*

此代码结构，消除了使用 `loop`、`if`、`else`、及 `break` 实现同样结构时，很多不可缺少的嵌套，且此结构更为清晰。在条件保持为真期间，代码就会运行；否则，他将退出循环。


###  使用 `for` 对集合进行遍历

可选择使用 `while` 结构，来对集合，诸如数组，的那些元素进行循环。作为示例，下面清单 3-4 中的循环，将打印出数组 `a` 中的各个元素。

文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < a.len() {
        println! ("值为：{}", a[index]);

        index += 1;
    }
}
```

*清单 3-4：使用 `while` 循环遍历集合的各个元素*

这个程序里，代码会根据那个数组中的元素，往上计数。是以索引 `0` 开始，然后循环，直到循环到了数组中最后的那个索引（即，在 `index < 5` 不再为 `true` 时）。运行此代码将打印出数组中的所有元素：

```console
$ cargo run                                                                                  ✔
   Compiling loops v0.1.0 (/home/peng/rust-lang/projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/loops`
值为：10
值为：20
值为：30
值为：40
值为：50
```

全部五个数组值都会出现在终端里，跟预期一样。尽管 `index` 在某个时间点达到值 `5`，但该循环会在尝试从那个数组获取第六个值之前，就停止执行。

但这种方法易于出错；在索引值或测试条件不正确时，就会导致该程序出错。比如，若把数组 `a` 的定义修改为有四个元素，而忘记了将那个条件更新到 `while index < 4`，此代码就会出错。由于编译器增加了在那个循环过程中，每次迭代上用于执行对 `index` 是否在数组边界内的，条件检查时间，因此这种方法还慢。

作为一种位为简练的替代，就可使用 `for` 循环而对集合中的各个元素，执行一些代码。`for` 循环看起来就跟下面清单 3-5 中的代码一样：

文件名：`src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println! ("值为：{}", element);
    }
}
```

*清单 3-5：使用 `for` 循环对结合的各个元素进行遍历*


在运行这段代码时，将看到与清单 3-4 中同样的输出。更重要的是，现在业已提升了代码的安全性，并消除了可能因超出那个数组末端，或因索引未足够触及而遗失掉一些数组项目，而导致的代码错误。

使用这个 `for` 循环，在更改了那个数组中值的个数时，就无需记得，像清单 3-4 中所使用的方式那样，去修改任何其他代码。

`for` 循环的安全与简洁，使得他们成为了 Rust 中最常用的循环结构。即使在那种要以确切次数来运行某些代码的情形下，如同清单 3-3 中用到 `while` 循环的倒计时示例，大多数 Rust 公民也将会使用 `for` 循环。要以确切次数运行某些代码，则要用到由标准库提供的 `Range` 特性了，`Range` 会依序生成自某个数字开始，并在另一数字之前结束，其间的全部数字来。

下面就是使用 `for` 循环，和另一个至今还未讲到的、用于逆转那个范围的 `rev` 方法，来实现那个倒计时的样子：

文件名：`src/main.rs`

```rust
fn main() {
    for number in (1..4).rev() {
        println! ("{}!", number);
    }

    println! ("发射！！");
}
```

此代码要更好一些，不是吗？


## 总结

咱们做到了！这第 3 章内容可真不少：在这里掌握了变量、标量与复合数据类型、函数、代码注释、`if`表达式，还有循环！请构建一些程序来完成下面这些事情，从而练习一下本章所讨论的那些概念：

- 对法式温度和摄氏温度之间互相转换；
- 生成第 n 个斐波拉基数；
- 利用圣诞颂歌 “The Twelve Days of Christmas” 中的重复，而打印出这首颂歌的歌词来；

在做好了继续新内容的学习后，就将要讨论到 Rust 中的一个在其他编程语言中并不多见的概念：所有权（ownership）。

## 练习答案


<details>
    <summary>“法式温度与摄氏温度的转换”</summary>

```rust
use std::io;
use std::process;

fn fah_to_cels(f: f32) -> f32 {
    return (f - 32.0) / 1.8;
}

fn cels_to_fah(c: f32) -> f32 {
    return c * 1.8 + 32.0;
}

fn main() {
    println! ("法式温度与摄氏温度之间的转换");

    loop {
        println! ("\n-----------------\n请选择：
            '1'-摄氏温度/'2'-法式温度/'Q'/\"quit\" 退出程序。
            '1'/'2'/'Q'/\"quit\"[1]：");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("读取输入失败！");

        let temp_type = temp_type.trim();

        if temp_type.eq("Q") || temp_type.eq("quit") { process::exit(0); }

        if ! temp_type.eq("1") && ! temp_type.eq("2") && ! temp_type.eq("") {
            println! ("无效输入，请输入 '1'、'2'、'Q'、\"quit\"，或直接按下回车键");
            continue;
        }

        if temp_type.eq("1") || temp_type.eq("") {
            println! ("请输入要转换的摄氏温度：");
            let temp = get_temp_input();

            println! ("摄氏温度： {:.2}°C，约为法氏温度：{:.2}°F", temp, cels_to_fah(temp));
        }

        if temp_type.eq("2") {
            println! ("请输入要转换的法氏温度：");
            let temp = get_temp_input();

            println! ("法氏温度：{:.2}°F，约为摄氏温度：{:.2}°C", temp, fah_to_cels(temp));
        }
    }
}

fn get_temp_input() -> f32 {
    return loop {
        let mut tmp = String::new();

        io::stdin()
            .read_line(&mut tmp)
            .expect("读取输入失败");

        match tmp.trim().parse() {
            Ok(num) => { break num },
            Err(_) => {
                println! ("请输入一个浮点数，比如 -10.0, 15.6");
                continue
            }
        };
    };
}
```

</details>


<details>
    <summary>"生成第 n 个斐波拉基数"</summary>


```rust
use std::io;
use num_format::{Locale, ToFormattedString};
// use std::process;

fn nth_fibonacci(n: u64) -> u64 {

    if n == 0 || n == 1 {
        return n;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}

fn main() {
    println! ("找出第 n 个斐波拉基数");

    'main_loop: loop {
        println! ("请输入 n: （Q/quit 退出程序）");

        let n: u64 = loop {
            let mut tmp = String::new();

            io::stdin()
                .read_line(&mut tmp)
                .expect("读取输入失败！");

            let tmp = tmp.trim();

            if tmp.eq("Q") || tmp.eq("quit") {
                // process::exit(0);
                break 'main_loop;
            }

            match tmp.parse() {
                Ok(num) => { break num },
                Err(_) => {
                    println! ("请输入一个正整数！\n");
                    continue;
                },
            };
        };

        println! ("第 {} 个斐波拉基数为：{}",
            n,
            nth_fibonacci(n).to_formatted_string(&Locale::en));
    }
}
```

</details>


<details>
    <summary>"打印圣诞颂歌 ‘The Twelve Days of Christmas’ 歌词"</summary>

```rust
fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "nineth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let amounts = [
        "A",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve"
    ];
    let things = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese-a-laying",
        "swans-a-swimming",
        "maids-a-milking",
        "ladies dancing",
        "lords-a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for num in 1..=12 {
        println! ("\nOn the {} day of Christmas,\nMy true love gave to me:",
            days[num-1]);
        for tmp in (0..num).rev() {
            if tmp == 0 && num == 1 {
                println! ("{} {}.", amounts[tmp], things[tmp]);
            }
            if tmp == 0 && num != 1 {
                println! ("And {} {}.", amounts[tmp].to_lowercase(), things[tmp]);
            }
            if tmp != 0 {
                println! ("{} {},", amounts[tmp], things[tmp]);
            }
        }
    }
}
```

</details>
