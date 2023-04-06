# 最后项目：构建一个多线程的 Web 服务器

**Final Project: Building a Multithreaded Web Server**


这是一个漫长的旅程，但我们已经到达了本书的结尾。在本章中，咱们将一起构建又一个项目，来演示咱们在最后这些章中，曾涉及到的一些概念，同时回顾一些较早的内容。

对于这个最后的项目，咱们将构造一个讲出 “你好”，且在浏览器中看起来如图 20-1 那样的一个 web 浏览器。


![咱们最后一起做的项目](images/20-01.png)

*图 20-1：咱们最后一起做的项目*

以下是构建这个 web 服务器的计划：

1. 学习一点有关 TCP 与 HTTP 方面的知识；
2. 在某个套接字上监听 TCP 连接；
3. 解析少数几个 HTTP 请求；
4. 创建出某种恰当的 HTTP 响应；
5. 运用线程池，提升咱们服务器的吞吐量。

在咱们开始动手前，咱们应注意到一个情况：咱们将运用的方法，将不会是在 Rust 下构建 web 服务器的最佳方法。在 [crates.io](https://crates.io/) 上，一些社区成员已经发布了数个，适合用于生产环境，提供了更完整功能的 web 服务器，以及咱们将要构建的线程池实现的代码箱。但是，本章中咱们的意图，是要帮助咱们学习掌握，而非走那样的捷径。由于 Rust 是门系统编程语言，因此咱们可以选择咱们打算着手的抽象层次，并可以触及到相比其他语言中，可行的或可操作的更低级别。因此咱们将亲自编写这个基本的 HTTP 服务器与线程池，如此咱们便可以学习这些代码箱之后的，今后可能会用到的一些一般概念与技巧。


## 构建一个单线程的 Web 服务器

咱们将通过让一个单线程的 web 服务器工作起来而开始。在咱们开始前，先来看看在构建 web 服务器中涉及到的一些协议的快速概览。这些协议的细节，超出了本书范围，而简要概述，就将给到咱们所需的信息。

Web 服务器中涉及的两种主要谢谢，分别是 *超文本传输协议，Hypertext Transfer Protocol, HTTP* 与 *传输控制协议，Transmission Control Protocol, TCP*。两种协议都是 *请求-响应，request-response* 的协议，表示 *客户端，client* 发起请求，而 *服务器，server* 监听到请求并提供给客户端一个响应。这些请求和响应的内容是由两种协议定义的。

TCP 是种描述了信息如何从一台服务器到达另一服务器，但并未指明信息为何的低级别。HTTP 则是经由定义请求与响应的内容，而于 TCP 之上构建的。技术上要在其他协议上使用 HTTP 是可行的，但在绝大多数情况下，HTTP 都在 TCP 上发送他的数据。咱们将在 TCP 的原始字节，与 HTTP 请求和响应下，进行工作。

### 监听 TCP 连接

**Listen to the TCP Connection**

咱们的 web 服务器需要监听某个 TCP 连接，因此那便是咱们将要做的第一部分工作。标准库提供了一个 `std::net` 模组，允许咱们完成这一点。咱们来以寻常方式构造一个新的项目：

```console
$ cargo new hello --vcs none
     Created binary (application) `hello` package
$ cd hello
```

现在请输入下面清单 20-1 中 `src/main.rs` 里的代码来开始。这段代码会在本地地址 `127.0.0.1:7878` 处监听传入的 TCP 流。当他获取到一个传入流时，他就会打印 `连接已建立！`。

文件名：`src/main.rs`

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println! ("连接已建立！");
    }
}
```

*清单 20-1：监听传入流并在咱们接收到某个流时打印一条消息*

运用 `TcpListener`，咱们就可以在地址 `127.0.0.1:7878` 处监听 TCP 连接。在这个地址中，冒号之前的部分，是个表示咱们的计算机的 IP 地址（在所有计算机上这都是同样的，而并不特别表示本书作者的计算机），同时 `7878` 为端口。咱们之所以选择了这个端口，有两个原因：通常不是在这个端口上接收 HTTP，因此咱们的服务器，大概率不会与咱们可能在咱们的机器上运行的任何别的 web 服务器冲突，而 `7878` 则是电话机上输入的 *rust*。

这个场景中的 `bind` 函数，会像将返回一个新 `TcpListener` 实例的 `new` 函数一样工作。该函数之所以叫做 `bind`，是因为在网络通信中，连接到要监听的端口，被称为 “绑定到端口”。

`bind` 函数返回的是个 `Result<T, E>`，表明有可能绑定失败。比如，连接到端口 `80` 需要管理员权限（非管理员只可以监听高于 `1023` 的那些端口，译注：在 *nix 平台上有此限制，但在 Win 平台上没有），因此若咱们在非管理员下尝试连接到端口 `80`，端口绑定就不会工作。在比如咱们运行了这个程序的两个实例，而因此有两个程序在监听同一端口时，端口绑定也不会工作。由于咱们仅是处于学习目的，而编写的一个基本服务器，因此咱们就不会纠结于处理这些类别的错误；相反，咱们使用 `unwrap` 来在错误发生时停止这个程序。

`TcpListener` 上的 `incoming` 方法，会返回一个给到咱们流（更具体的，是一些类型 `TcpStream` 的流）序列的迭代器，an iterator that gives us a sequence of streams。单一的 *流，stream* 表示了客户端与服务器之间的一个打开的连接，an open connection。而一个 *连接，connection* 则是客户端连接到服务器过程中，完整的请求与响应的叫法，服务器会生成一个响应，且服务器会关闭这个连接。就这样，咱们将从那个 `TcpStream` 读取，来看看客户端发送了什么，并于随后把咱们的响应写到这个流，以将数据发送回客户端。总的来说，这个 `for` 循环将依次处理每个连接，并为咱们产生一系列要处理的流。

至于现在，咱们对流的处理，是由在流有任何错误时，调用 `unwrap` 来终止咱们的程序所构成；若没有任何错误，那么这个程序就会打印一条消息。在下一代码清单中，咱们将为流成功的情形，添加更多功能。在客户端连接到服务器时，咱们可能会从那个 `incoming` 方法收到错误的原因，便是咱们没有真正在一些连接上迭代。相反，咱们是在一些 *连接尝试，connection attempts* 上迭代。连接可能因为数种原因而不成功，许多的这些原因都是特定于操作系统的。比如，许多操作系统都有他们所支持的并发开启连接数限制，a limit to the number of simultaneous open connecitons；超出那个数目的新建连接尝试就会产生错误，除非一些开启的连接关闭。

咱们来尝试运行这段代码！在终端里运行 `cargo run` 并随后在 web 浏览器中加载 `127.0.0.1:7878`。由于服务器没有正确发回任何数据，因此浏览器应给出像是 `Connection reset,` 的错误消息。但当咱们看着终端时，应看到在浏览器连接到服务器时，有数条打印处的消息！

> 注：可使用 `$curl 127.0.0.1:7878` 命令进行调试，且使用 `curl` 也是网络编程调试中常用的方法。

```console
     Running `target/debug/hello`
连接已建立！
连接已建立！
连接已建立！
连接已建立！
```

有的时候，咱们会看到一次浏览器请求下打印出的多条消息；原因可能是浏览器在构造页面请求时，也会构造其他资源的请求，像是出现在浏览器 tab 分页中的 `favicon.ico` 图标。

也可由可能是由于这个服务器没有响应任何数据，浏览器因此会尝试多次连接到这个服务器。在 `stream` 超出作用域，而在那个循环结束出被丢弃时，连接就会作为 `drop` 实现的一部分而被关闭。由于故障可能是临时的，因此浏览器有时会以重试处理关闭的连接。重要的是，咱们已然成功得到了到 TCP 连接的句柄，a handle to a TCP connection！

请记得在咱们完成运行代码的特定版本时，要通过按下 `Ctrl-c` 来停止这个程序。以后在完成了各套代码修改后，要通过运行 `cargo run` 命令重启这个程序，来确保咱们是在运行最新的代码。


### 读取请求

**Reading the Request**

咱们来实现读取来自浏览器请求的功能！为将首选获取到连接，及随后对连接采取一些措施这两个关注点分离，咱们将开启一个用于处理连接的新函数。在这个新的 `handle_connection` 函数中，咱们将从 TCP 流读取数据，并将其打印出来，从而咱们就可以看到从浏览器发出的数据。请将代码修改为清单 20-2 这样。

文件名：`src/main.rs`

```rust
#![allow(warnings)]
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufferedReader::new(stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println! ("请求：{:#?}", http_request);
}
```

*清单 20-2：从 `TcpStream` 读取并打印出数据*


咱们将 `std::io::prelude` 与 `std::io::BufReader` 带入作用域，来获取到实现从 TCP 流读取和写入的那些特质与类型的访问。在 `main` 函数的那个 `for` 循环中，不再是打印一条声称咱们已构造一个连接的消息，咱们限制调用了新的 `handle_conn` 函数，并把那个 `stream` 传递给他。

在 `handle_conn` 函数中，咱们创建了一个新的，封装着到 `stream` 的一个可变引用的 `BufReader` 实例。`BufReader` 会通过管理到 `std::io::Read` 特质一些方法的调用，为咱们添加缓冲。

咱们创建了一个名为 `http_req` 的变量，来收集浏览器发送到咱们服务器的请求的那些行。通过添加那个 `Vec<_>` 类型注解，咱们表明了咱们打算把这些行收集到一个矢量值中。

`BufReader` 实现了 `std::io::BufRead` 特质，该特质提供了 `lines` 方法。`lines` 方法会经由当其发现一个新行字节，a newline byte, 时分割数据流，而返回一个 `Result<String, std::io::Error` 的迭代器。而要获取各个 `String`，咱们就要映射，map，并 `unwrap` 各个 `Result`。若数据不是有效的 UTF-8，或从 TCP 流读取存在问题时，这个 `Result` 就可能是个错误。再次声明，生产程序应更优雅地处理这些报错，但咱们为简化目的，而选择了在错误情形下停止这个程序。

浏览器通过在一行中发送两个新行字符，发出 HTTP 请求结束信号。因此要从 TCP 流获取一次请求，咱们就要取那些直到咱们得到一个为空字符串的行为止的那些行。一旦咱们将这些行收集到那个矢量值中，咱们就用漂亮的调试格式，把他们打印处理，如此咱们就可以看看，web 浏览器正发送给咱们服务器的那些指令。

咱们来试试这段代码！启动这个程序，并再次于浏览器中构造一次请求。请注意咱们仍会在浏览器中得到一个错误页面，但终端中咱们程序的输出，现在将看起来类似于下面这样：

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
请求: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```


> 注：使用 `curl --noproxy '*' 127.0.0.1:7878` 的输出，如下面这样：


```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello`
请求：[
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: curl/7.68.0",
    "Accept: */*",
]
```

根据咱们的浏览器，咱们可能会得到些许不同的输出。既然咱们打印了请求数据，咱们就可以通过查看请求第一行中 `GET` 之后的路径，而发现为何咱们会从一次浏览器请求，得到多个连接。若重复的连接都是在请求 `/`，咱们就知道由于浏览器没有从咱们的程序得到响应，因此其是在尝试重复获取 `/`。

下面来对这一请求数据加以细分，以搞清楚浏览器是在询问咱们的程序些什么。


### 近观 HTTP 请求

**A closer Look at an HTTP Request**


HTTP 是种基于文本的协议，而请求会采用下面这种格式：

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

第一行是保存着有关该客户端正请求什么的信息的 *请求行，request line*。该请求行的第一部分，表示正使用的 *方法，method*，比如 `GET` 或 `POST`，描述了客户端是如何构造此请求的。咱们的客户端使用了一个 `GET` 请求，这意味着其是在询问信息。

请求行接下来的部分为 `/`，表示客户端正请求的 *同一资源标识符，Uniform Resource Identifier, URI*：URI 几乎是，但不完全与 *同一资源定位符，Uniform Resource Locator, URL* 一样。URIs 与 URLs 之间的区别对于这章中咱们的目的不重要，但 HTTP 的规格使用了 URI 这个词，因此咱们只能在此处暗自用 URL 代替 URI。

最后部分是客户端所用的 HTTP 版本，而随后这个请求行便以一个 *CRLF 序列，CRLF sequence* （CRLF 代表的是 *回车，carriage return* 与 *换行，line fedd*，是打字机时代的术语！）结束了。这个 CRLF 序列还可以写作 `\r\n`，其中的 `\r` 是个回车，而 `\n` 是个换行。CRLF 序列将请求行与其余的请求数据分开。请注意当 CRLF 被打印时，咱们会看到一个新行开始，而非 `\r\n`。

查看如今咱们从运行这个程序所接收到的请求行数据，咱们发现 `GET` 即为请求方法，`/` 便是请求的 URI，而 `HTTP/1.1` 则是请求的 HTTP 版本。

在请求行之后，从 `Host:` 开始的其余那些行，就是些头了。`GET` 请求没有请求体。

请从不同浏览器构造请求，或是询问不同地址，比如 `127.0.0.1:7878/test`，来发现请求数据会怎样变化。

> 注：运行 `curl --noproxy '*' 127.0.0.1:7878/test` 时，请求数据如下所示：


```console
请求：[
    "GET /test HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: curl/7.68.0",
    "Accept: */*",
]
```

既然咱们明白了浏览器是在询问什么，下面就来发回一些数据吧！


### 写下响应

**Writing a Response**

咱们将要实现发送响应客户端请求数据。响应有着下面的格式：

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

其中第一行是包含在响应中用到的 HTTP 版本的 *状态行，status line*、汇总了请求结果的一个数字的状态码、以及提供了状态码文字描述的一个原因短语，a reason phrase。在那个 CRLF 之后是一些 HTTP 头、另一个 CRLF 序列、及响应的响应体。

下面就是一个使用了 HTTP 版本 1.1 的示例响应，有着状态码 `200`、一个 `OK` 的原因短语、没有头部、也没有响应体。

```text
HTTP/1.1 200 OK\r\n\r\n
```

状态代码 `200` 是标准的成功响应。这个文本便是个极小的成功 HTTP 响应。下面来把这个响应，作为咱们到成功请求的响应，写到 TCP 流！在那个 `handle_conn` 函数中，移除曾是打印请求数据的 `println!`，而将其替换为下面清单 20-3 中的代码。

文件名：`src/main.rs`

```rust
fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let resp = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(resp.as_bytes()).unwrap();
}
```

*清单 20-3：将一个极小的成功 HTTP 响应写到 TCP 流*

那第一行定义了保存成功消息数据的 `resp` 变量。随后咱们在咱们的 `resp` 上调用 `as_bytes`，将字符串数据转换为一些字节。`stream` 上的 `write_all` 方法，会取一个 `&[u8]` 并将那些字节直接发送到 TCP 连接。由于 `write_all` 操作可能失败，咱们就像前面一样，于任何的错误结果上使用 `unwrap`。再次，在真实应用中，咱们会在这里加上错误处理。

有了这些修改，咱们来运行咱们的代码，并构造一次请求。咱们就不再打印任何数据到终端，因此咱们不会看到除 Cargo 的输出外，其他任何的输出。当咱们在 web 浏览器中加载 `127.0.0.1:7878` 时，咱们应得到一个空白页而非报错。咱们刚刚已经硬编码了接收 HTTP 请求并发送一次响应了！


### 返回真正的 HTML

**Returning Real HTML**

下面来实现返回相比空白页更多内容的功能。请在咱们的项目目录根处，而非 `src` 目录中创建一个新文件 `hello.html`。咱们可放入任何咱们想要的 HTML；下面清单 20-4 给出了一种可能。

文件名：`hello.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>你好!</title>
  </head>
  <body>
    <h1>你好!</h1>
    <p>来自 Rust 的问好</p>
  </body>
</html>
```

*清单 20-4：要在响应中返回的一个样例 HTML 文件*

这是个带有一个与一些文本的最小 HTML5 文档。要在收到一个请求时从服务器返回这个文档，咱们将如下清单 20-5 中所示那样，修改 `handle_conn` 来读取这个 HTML 文件，将其作为响应体，添加到一个响应，并将其发送。

文件名：`src/main.rs`

```rust
#![allow(warnings)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let resp =
        format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}
```

*清单 20-5：将 `hello.html` 的内容作为响应的响应体发送*

咱们已添加 `fs` 到那个 `use` 语句，来将标准库的文件系统模组带入到作用域中。把文件内容读取到一个字符串的代码应看起来不陌生；在第 12 章，于清单 12-4 中为咱们的 I/O 项目读取一个文件的内容时，咱们曾用到过他。

接下来，咱们使用了 `format!` 宏，来将那个文件的内容，添加为这个成功响应的响应体。为确保一个有效的 HTTP 响应，咱们添加了被设置为咱们的响应体大小的一个 `Content-Length` 头部，在这个示例中就是 `hello.html` 的大小。

以 `cargo run` 运行这段代码，并在浏览器中加载 `127.0.0.1:7878`；咱们应看到咱们的 HTML 被渲染了！

目前，咱们忽略了 `http_req` 中的响应数据，而只是无条件地发回那个 HTML 文件的内容。那就意味着当咱们在浏览器中尝试请求 `127.0.0.1:7878/something-else` 时，咱们将仍然得到这同样的 HTML 响应。此刻，咱们的服务器是非常有限的，且不会完成绝大多数 web 服务器所完成的那些事情。咱们打算根据请求定制咱们的响应，并只为格式良好的到 `/` 请求，发回这个 HTML 文件。


### 对请求加以验证并有选择地进行响应

**Validating the Request and Selectively Responding**

现在，咱们的 web 服务器将始终返回那个文件中的 HTML，而不管客户端请求的是什么。下面来添加在返回那个 HTML 文件前，检查浏览器是在请求 `/`，并在浏览器请求其他路径时，返回一个错误的功能。为此，咱们需要如下面清单 20-6 中所示的那样修改 `handle_conn`。这段新代码会将收到的请求，与咱们所知的 `/` 请求看起来的样子对比检查，并添加了 `if` 及 `else` 代码块来分别对待各种请求。

文件名：`src/main.rs`

```rust
fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    if req_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let resp =
            format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(resp.as_bytes()).unwrap();
    } else {
        // 别的一些请求
    }
}
```

*清单 20-6：以不同于其他请求方式，处理到 `/` 的请求*

咱们只打算看看 HTTP 请求的第一行，因此就不再将整个请求读取到一个矢量值了，咱们调用了 `next` 来从那个迭代器得到第一个条目。这里的首个 `unwrap` 会注意其中的 `Option`，并在迭代器没有条目时停止这个程序。第二个 `unwrap` 则会处理其中的 `Result`，并与清单 20-2 中所添加的 `map` 里的那个 `unwrap` 有着同样的效果。

接下来，咱们检查了 `req_line`，来看看其是否等于到 `/` 路径 `GET` 请求的请求行。在其等于时，那个 `if` 代码块就会返回咱们 HTML 文件的内容。

若 `req_line` *不* 等于到 `/` 路径 `GET` 请求的第一行时，就意味着咱们收到了一些别的请求。稍后咱们将添加代码到那个 `else` 代码块，来响应全部其他请求。

现在请运行此代码，并请求 `127.0.0.1:7878`；咱们应获取到 `hello.html` 中的 HTML。在咱们构造任何其他请求时，比如 `127.0.0.1:7878/something-else`，就将得到像是咱们曾在运行清单 20-1 及清单 20-2 中的代码时，所看到连接错误。

现在来将清单 20-7 中的代码，添加到那个 `else` 代码块，以返回一个带有状态代码 `404` 的响应，这通告了请求的内容未找到。咱们还将返回一些在浏览器中要渲染页面的 HTML，将这种响应表示给终端用户。

文件名：`src/main.rs`

```rust
    // --跳过代码--
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let resp =
            format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(resp.as_bytes()).unwrap();
    }
```

*清单 20-7：在请求的是除 `/` 外的其他路径时以状态代码 `404` 及一个错误页面进行响应*

此处，咱们的响应有着一个代码状态代码 `404`，及原因短语 `NOT FOUND` 的状态行。该响应的响应体，将是文件 `404.html` 中的 HTML。咱们将需要创建 `hello.html` 旁边，用于错误页面的 `404.html` 文件；请再次随意使用咱们想要的任何 HTML，或使用下面清单 20-8 中的示例 HTML。

文件名：`404.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>你好!</title>
  </head>
  <body>
    <h1>糟糕!</h1>
    <p>抱歉，我不明白你要什么。</p>
  </body>
</html>
```

*清单 20-8：全部 404 响应下要发回页面的示例内容*

在这些修改下，请再次运行咱们的服务器。请求 `127.0.0.1:7878` 应返回 `hello.html` 的内容，而任何别的请求，像是 `127.0.0.1:foo`，就应返回 `404.html` 中的报错 HTML。


### 初试重构

*A Touch of Refactoring*


此时的 `if` 与 `else` 两个代码块，有着很多重复：他们都在读取文件及将文件内容写到 TCP 流。唯二区别就是响应的状态行与文件名。下面就来通过抽取处这些差异到单独的 `if` 和 `else` 行，这些行将把响应状态行与文件名，赋值给两个变量；随后咱们就可以在代码中，不带条件地使用这两个变量，来读取文件并写下响应。下面清单 20-9 给出了替换了大段的 `if` 与 `else` 代码块后的最终代码。

文件名：`src/main.rs`

```rust
// --跳过代码--
fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ( "HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let resp =
        format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}
```

*清单 20-9：将 `if` 和 `else` 代码块重构为只包含两种情况下不同的代码*

现在 `if` 与 `else` 两个代码块，就只会返回一个元组中，响应状态行与文件名的相应值了；随后咱们运用第 18 章中曾讨论过的 `let` 语句中的模式，而使用了解构特性，来将这两个值复制给 `status_line` 与 `filename`。

原先那些重复代码，现在便是在 `if` 与 `else` 两个代码块外面，并使用了 `status_line` 与 `filename` 两个变量。这令到看出两种情况之间的差别更为容易，并意味着在咱们打算修改文件读取与响应写入工作方式时，只有一处要更新代码。清单 20-9 中代码的行为，将与清单 20-8 中的一致。

相当棒！现在咱们就有了一个以差不多 40 行 Rust 代码编写的，以一个内容页面响应一个到 `/` 的请求，并以一个 `404` 响应回应全部其他请求的简单 web 服务器了。

当前，咱们的服务器是运行在单线程下的，意味着其只能一次服务一个请求。接下来就要通过模拟一下低速请求，检查那怎样会称为一个问题。随后咱们将修复这个问题，从而让咱们的服务器可以一次处理多个请求。


## 将咱们的单线程服务器改写为多线程服务器

**Turning Our Single-Thread Server into a Multithreaded Server**

现在，这个服务器将依次处理每个请求，这意味着其将不会在前一个连接完成处理前，处理后一连接。若服务器收到了越来越多的请求，这种顺序执行就会越来越差。而若该服务器收到了一个要耗费较长时间处理的请求，即使后续的新请求可被快速处理，但其仍将不得不等待直到那个长时间请求完成。咱们需要修复这个问题，但首选，咱们将具体看看这个问题。


### 在当前服务器实现下模拟一个慢速请求

**Simulating a Slow Request in the Current Server Implemenation**

咱们将看看一个慢速处理的请求，能怎样影响那些到咱们当前服务器实现的其他请求。下面清单 20-10 以一个将导致服务器在响应前睡眠 5 秒的模拟慢速请求，实现了对到 `/sleep` 请求的处理。

文件名：`src/main.rs`

```rust
#![allow(warnings)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thred,
    time::Duration,
};
// --跳过代码--

fn handle_conn(mut stream: TcpStream) {
    // --跳过代码--

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ( "HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 0K", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --跳过代码--
}
```

*清单 20-10：通过睡眠 5 秒模拟慢速请求*

现在咱们有了三种情况，于是就已从 `if` 切换到了 `match`。咱们需要显式地在 `req_line` 切片上，与那三个字符串字面值进行模式匹配；`match` 不会像相等比较方式所做的那样，执行自动引用与解引用。

首条支臂与清单 20-9 的 `if` 代码块是一样的。第二条支臂，是将请求与 `/sleep` 匹配。在收到那个请求时，服务器将在渲染那个成功 HTML 页面之前，睡眠 5 秒。第三支臂则与清单 20-9 的那个 `else` 代码块是一样的。

咱们可以看出，咱们的服务器有多原始：真正的库将以一种不那么冗长的方式，处理多种请求的识别！

请使用 `cargo run` 启动服务器。随后打开两个浏览器窗口：一个用于 `http://127.0.0.1/7878`，另一个用于 `http://127.0.0.1:7878/sleep`。若咱们像之前一样进入那个 `/` URI 几次，咱们将看到其响应很快。但在进入 `/sleep` 并于随后加载 `/` 时，就会看到那个 `/` 会一直等待，知道 `sleep` 已经于加载之前睡眠了 5 秒。

咱们可以用来避免慢速请求后面那些请求滞后的技巧有多种；咱们将实现的技巧，便是线程池。


### 使用线程池提升吞吐量

**Improving Throughput with a Thread Pool**

所谓 *线程池，thread pool*，是指处于等待中，并准备好处理某项任务的一组生成的线程。在程序收到一项新任务时，他便指派线程池中的一个线程给该项任务，而那个线程就会处理这个任务。池中的剩余线程，则是可以处理任何的于这首个线程进行处理时，进来的那些任务的。在这首个线程完成其任务处理时，他就会回到空闲线程的线程池，准备处理某项新任务。线程池实现了连接的并发处理，从而提升咱们服务器的吞吐能力。

咱们将把池中线程数量，先知道一个较小的数目，以保护咱们免于拒绝服务攻击，Denial of Service(DoS) attacks；若咱们让咱们的程序在每个请求进入时，创建一个新线程，那么构造出一千万个请求到咱们的服务器的某人，就能经由耗尽咱们服务器的全部资源，而使得这些请求的处理陷入停滞，而造成极大破坏。

这种技巧只是提供 web 服务器吞吐量的许多方法之一。咱们可能探讨的其他选项分别是 *分叉汇合模型，fork/join model*、*单线程异步 I/O 模型，single-threaded async I/O model*，抑或 *多线程异步 I/O 模型，multi-threaded async I/O model*。若对此问题感兴趣，那么可以阅读有关其他解决方案的资料，并尝试实现他们；对于 Rust 这种底层编程语言，所有这些选项都是可行的。


在开始实现线程池前，咱们来聊聊用到这个池子的东西会是什么样子。在咱们正要尝试设计代码时，首先编写客户端界面，可有助于引导咱们的设计。要以咱们打算调用代码 API 的方式，编写出这些有组织架构的代码 API；随后在那种组织架构下实现功能，而非先实现功能而随后设计那些公开 API。

与第 12 章中项目里用到的测试驱动方式的开发，test-driven development，类似，这里咱们将运用编译器驱动的开发，compiler-driven development。咱们将先编写出咱们打算调用那些函数的代码，而随后会看看来自编译器的那些报错，以确定出接下来咱们应修改些什么，来让代码运作起来。在咱们进行那一步之前，咱们将探讨一下咱们并不会用到的一种技巧，作为开头。

#### 为每个请求生成一个线程

**Spawning a Thread for Each Request**

首先，咱们来探讨一下若咱们的代码给每隔连接创建一个新线程，他看起来会怎样。正如早先所提到的，由于潜在地生成无限数目线程的那些问题，这样做不是咱们的最终计划，但其为首先得到一个运作多线程服务器的起点。随后咱们将添加线程池作为一项改进，且将这两种方案进行对比将更容易一些。下面清单 20-11 给出了把 `main` 构造为于那个 `for` 循环里，生成一个新线程来处理每个 TCP 流的一些修改。

文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_conn(stream);
        });
    }
}
```

*清单 20-11：为每个 TCP 流生成一个新线程*

如同咱们在第 16 章中所学到的，`thread::spawn` 讲创建出一个新线程，并于随后在新线程中，运行那个闭包中的代码。当咱们运行此代码，并在浏览器中加载 `/sleep`，随后在另外两个浏览器 Tab 页中加载 `/`，咱们就会看到到 `/` 的请求就不必等待 `/sleep` 请求完毕了。不过，如同咱们曾提到过的，因为咱们正不带任何限制地构造新线程，而最终将使系统不堪重负。


#### <a id="creating-a-finite-number-of-threads"></a>创建有限数目的线程

**Creating a Finite Number of Threads**


咱们想要咱们的线程池，以类似的、熟悉的方式运作，而无需那些用到咱们 API 的代码有较大修改。下面清单 20-12 给出了咱们打算用到的 `ThreadPool`，而非 `thread::spawn`，的假想接口。

文件名：`src/main.rs`

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }
}
```

*清单 20-12：咱们设想的 `ThreadPool` 接口*

咱们使用了 `ThreadPool::new` 来创建出有着可配置线程数目的新线程，在此示例中为四个线程。随后，在那个 `for` 循环中，`pool.execute` 有着与 `thread::spawn` 类似的接口，其中他会取个闭包，并将其给到线程池中的某个线程运行。这段代码尚不会编译，但咱们将进行尝试，如此编译器就会引导咱们如何修复他。


#### 运行编译器驱动的开发，构建出 `ThreadPool`

**Building `ThreadPool` Using Compiler Driven Development**

请完成清单 20-12 中对 `src/main.rs` 的修改，然后咱们就来运用 `cargo check` 给出的编译器报错，驱动咱们的开发。下面就是咱们所得到的第一个报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
  --> src/main.rs:12:16
   |
12 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello` due to previous error
```

很棒！这个错误告诉我们，咱们需要一个 `ThreadPool` 类型或模组，因此咱们现在就将构建一个出来。咱们的 `ThreadPool` 实现，将独立于咱们的 web 服务器所完成工作的类型。因此，咱们就来将这个 `hello` 代码箱，从二进制代码箱切换为一个库代码箱，来保存咱们的 `ThreadPool` 实现。在咱们改变为库代码箱后，咱们就可以在打算用到线程池的任何项目，而不只是用来服务 web 请求中，也可以使用这个独立的线程池了。

请创建一个包含了下面这个咱们目前所能有的 `ThreadPool` 结构体极简定义的 `src/lib.rs` 文件：

文件名：`src/lib.rs`

```rust
pub struct ThreadPool;
```

随后编辑 `main.rs`，来通过加入下面的代码到 `src/main.rs` 顶部，将 `ThreadPool` 从那个库代码箱，带入作用域：

文件名：`src/main.rs`

```rust
use hello::ThreadPool;
```

这段代码仍不会工作，但咱们就来再检查一边，以得到咱们需要解决的下一报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
  --> src/main.rs:14:28
   |
14 |     let pool = ThreadPool::new(4);
   |                            ^^^ function or associated item not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` due to previous error
```

此报错表明，接下来咱们就要给 `ThreadPool` 创建一个名为 `new` 的关联函数。咱们还知道了那个 `new` 需要有一个可将 `4` 作为实参接收的形参，并应返回一个 `ThreadPool` 的实例。下面就来实现将有着那些特性的这个极简 `new` 函数：

文件名：`src/lib.rs`

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

由于咱们清楚一个负的线程数目不会有任何意义，因此咱们选择了 `usize` 作为那个 `size` 参数的类型。咱们还知道咱们将使用这个 `4` 作为线程集合中原始的个数，那即使这个 `usize` 类型的目的所在，正如第三章的 [整数类型](Ch03_Common_Programming_Concepts.md#integer-types) 小节中曾讨论过的。

下面来再次检查：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:19:14
   |
19 |         pool.execute(|| {
   |              ^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` due to previous error
```

现在的报错之所以出现，是因为在 `ThreadPool` 上咱们没有一个 `execute` 方法。回顾 ["创建有限数目的线程"](#creating-a-finite-number-of-threads) 小节到，咱们已决定咱们的线程池，应有一个类似与 `thread::spawn` 的接口。此外，咱们将实现这个 `execute` 函数，如此其便会取那个给到他的闭包，并将其交给线程池中的某个空闲进程运行。

咱们将在 `ThreadPool` 上定义这个 `execute` 方法，来取一个闭包作为参数。回顾第 13 章中 [“将捕获值迁移出闭包与 `Fn` 特质”](Ch13_Functional_Language_Features_Iterators_and_Closures.md#moving-captured-values-out-of-closures-and-the-Fn-traits) 到咱们可以三种不同特质，将闭包取作参数：`Fn`、`FnMut` 与 `FnOnce`。咱们需要确定出这里要使用何种类别的闭包。咱们清楚咱们将以完成一些类似于标准库的 `thread::spawn` 实现类似的东西结束，因此咱们就可以看看 `thread::spawn` 的签名在其参数上有些什么。文档给出咱们下面的东西：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

其中的 `F` 类型参数，就是咱们在这里所关心的那个；那个 `T` 类型参数于返回值相关，而咱们并不关心那个。咱们可以看出，`spawn` 使用 `FnOnce` 作为 `F` 上的特质边界。由于咱们将最终将把咱们在 `execute` 中获得的实参，传递给 `spawn`，因此这或许也正是咱们想要的。由于为运行某个请求的线程，将只执行那个请求的闭包一次，而这是与 `FnOnce` 中的 `Once` 是相匹配的，故咱们可以进一步确信，`FnOnce` 便是咱们要用到的特质。

其中的 `F` 类型参数，还有着特质边界 `Send` 与生命周期边界 `'static`，在咱们这种情况下他们是有用的：咱们需要 `Send` 来将闭包，从一个线程转移到另一线程，并由于咱们不知道那个线程将耗时多久来执行，因此而需要 `'static`。下面咱们就来在 `ThreadPool` 上，创建出将取到有着这些边界的，类型 `F` 的泛型参数的 `execute` 方法：

文件名：`src/lib.rs`

```rust
#![allow(warnings)]
pub struct ThreadPool;

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
```

由于这个 `FnOnce` 表示一个不会取参数，且返回的是单元类型 `()` 的闭包，因此咱们仍旧使用了 `FnOnce` 后的 `()`。就跟函数定义一样，返回值类型可以在签名中省略，但即使咱们没有参数，咱们仍需这对括号。

又一次，这仍是那个 `execute` 方法的极简实现：他什么也没做，但咱们只是在试着让咱们的代码编译。咱们再来对其加以检查：

```console
$ cargo check
    Checking hello v0.1.0 (/home/lenny.peng/rust-lang-zh_CN/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
```

他编译了！不过请注意，当咱们尝试 `cargo run` 并在浏览器中构造一次请求时，咱们将看到浏览器中，一些咱们在本章开头曾看到过的报错。咱们的库尚未真正调用传递给 `execute` 的那个闭包！

> 注意：咱们或许听说过与有着严格编译器语言，比如 Haskell 与 Rust，有关的一种说法，即 “若代码编译了，他就会工作。” 然而这种说法并非一概而论。咱们的项目编译了，但他绝对什么也没干！若咱们是在构建一个真实、完整的项目，那么此时就将是开始编写检查代码编译与否，*以及* 是否具有咱们想要的行为的单元测试的好时机。

#### 在 `new` 中验证线程数目

**Validating the Number of Threads in `new`**

咱们没有对 `new` 与 `execute` 的参数做任何事情。下面就来以咱们打算的行为，实现这两个函数的函数体。咱们来构思一下 `new`，作为开始。早先由于负的线程数目没有意义，因此咱们给那个 `size` 参数，选择了一个无符号整数类型。不过尽管零也是相当有效的 `usize`，但零个线程的线程池，同样是无意义的。咱们将在返回一个 `ThreadPool` 实例前，添加检查 `size` 大于零的代码，并在程序收到一个零时，通过使用 `assert!` 宏，让程序终止运行，如下面清单 20-13 中所示。

文件名：`src/lib.rs`

```rust
impl ThreadPool {
    /// 创建出一个新的 ThreadPool。
    ///
    /// 其中的 size 为线程池中线程的数目。
    ///
    /// # 终止运行
    ///
    /// 这个 `new` 函数将在 size 为零时终止运行。
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        ThreadPool
    }

    // --跳过代码--
}
```

*清单 20-13：将 `ThreadPool` 实现为在 `size` 为零时终止运行*

咱们还以一些文档注释，doc comments，给咱们的 `ThreadPool` 结构体添加了一些文档。请注意咱们通过添加如同第 14 章中曾讨论过的，一个会呼出咱们的函数可能终止运行时的那些情形的小节，而遵循了良好的文档实践。请尝试运行 `cargo doc --open` 并点击那个 `ThreadPool` 结构体，来看到为 `new` 生成的文档看起来是怎样的！

与其如咱们在这里所做的添加这个 `assert!` 宏，咱们则可把 `new` 改为 `build`，并像咱们曾在清单 12-9 中那个 I/O 项目里的 `Config::build` 下所做的那样，返回一个 `Result`。但咱们已经决定，在此示例中是在尝试创建一个，其中全部线程都不应是不可恢复错误的线程池。若你觉得信心满满，那就请编写一个名为 `build`，有着下面签名的函数，来与这个 `new` 函数相比较：

```rust
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### 创建空间来存储这些线程

**Creating Space to Store the Threads**

既然咱们有了获悉咱们有着要在线程池中存储线程有效数目的一种办法了，咱们便可以创建出这些线程，并在返回这个 `ThreadPool` 结构体前，将他们存储在该结构体中。但是咱们要怎么 “存储” 一个线程呢？下面又来看看那个 `thread::spawn` 签名：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` 函数返回了一个 `JoinHandle<T>`，其中的 `T` 为闭包所返回的类型。咱们也来尝试使用 `JoinHandle`，并观察会发生什么。在咱们的用例中，咱们传递给线程池的闭包，将处理 TCP 连接，而不会返回任何东西，因此其中的 `T` 将是单元类型 `()`。

下面清单 20-14 中的代码将会编译，但尚不会创建任何线程。咱们已将 `ThreadPool` 的定义，修改为保存了一个 `thread::JoinHandle<()>` 实例的矢量值，以 `size` 大小初始化了这个矢量值，还建立了一个将运行某些代码来创建出那些线程的 `for` 循环，并返回了一个包含着这些线程的 `ThreadPool` 实例。

文件名：`src/lib.rs`

```rust
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 创建出一些线程并将他们存储在那个矢量中
        }

        ThreadPool { threads }
    }
    // --跳过代码--
}
```

*清单 20-14：为保存那些线程而给 `ThreadPool` 创建一个矢量值*

由于咱们正使用 `thread::JoinHandle` 作为 `ThreadPool` 中那个矢量值条目的类型，因此咱们已将 `std::thread` 带入到这个库代码箱的作用域。

一旦收到有效的大小，咱们的 `ThreadPool` 就会创建出可保存 `size` 个条目的一个新矢量值。那个 `with_capacity` 函数，执行的是与 `Vec::new` 同意的任务，但有个重要的不同之处：他会预先分配那个矢量值中的空间。由于咱们清楚咱们需要在那个矢量值中存储 `size` 个元素，那么预先完成这种分配，相比使用在有元素插入时调整自身的 `Vec::new`，就会稍微更具效率。

在再度运行 `cargo check` 时，其应成功。

#### 负责将代码从 `ThreadPool` 发送给某个线程的 `Worker` 结构体

**A `Worker` Struct Responsible for Sending Code from the `ThreadPoll` to a Thread**

在清单 20-14 中的那个 `for` 循环里，咱们留了一个有关线程创建过程的注释。这里，咱们将看看咱们具体要怎么创建出那些线程来。标准库提供了 `thread::spawn` 作为创建线程的一种方式，而 `thread::spawn` 则期望得到一些线程在其一创建出来，就应立即运行的代码。然而，在咱们的示例中，咱们打算创建出这些线程，并让他们 *等待，wait* 咱们稍后将要发送的那些代码。标准库的线程实现，没有包含任何实现那样做法的方式；咱们必须亲自实现他。

咱们将通过引入介于 `ThreadPool` 与那些线程之间，将对这种新行为加以管理的一种新数据结构，来实现这样的行为。咱们将把这种数据结构称作 `Worker`，在线程池实现中，这是个常见的术语。`Worker` 会拾取需要运行的代码，并在该 `Worker` 的线程中运行那些代码。设想某家饭馆中工作的人们：工人们会一直等待，直到有顾客点的菜单进来，而随后他们就负责接下这些菜单，并让顾客们满意。

在线程池中存储的，不再是 `JoinHandle<()>` 实例的矢量值，咱们将存储这个 `Worker` 结构体的实例。每个 `Worker` 都将存储一个单独的 `JoinHandler<()>` 实例。随后咱们将在 `Worker` 上实现一个，将取得要运行代码的闭包，并将其发送到已经运行着的线程去执行的方法。咱们还将给到每个 `Worker` 一个 `id`，如此咱们就可以在日志记录或调试时，区分出线程池中那些不同的 `Worker`。


以下便是在咱们创建一个 `ThreadPool` 时，将发生的一个新过程。咱们将在以此方式建立起 `Worker` 结构体后，再实现把闭包发送给线程的那些代码：

1. 定义出一个保存了一个 `id` 与一个 `JoinHandler<()>` 的 `Worker` 结构体；
2. 把 `ThreadPool` 修改为保存一个 `Worker` 实例构成的矢量值；
3. 定义出会取一个 `id` 数字，并返回保存着这个 `id`，以及带有所生成的有着一个空闭包的线程的一个 `Worker` 实例，这样一个 `Worker::new` 函数；
4. 在 `Thread::new` 中，会使用那个 `for` 循环的计数器，来生成一个 `id`、用那个 `id` 创建出一个新的 `Worker`，并将该 `Worker` 存储在那个矢量值中。


若咱们准备挑战一下，那么请尝试在查看清单 20-15 中代码之前，自己实现这些修改。

准备好了吗？下面就是有着一种做出前面那些修改的一种方式的清单 20-15。

文件名：`src/lib.rs`

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --跳过代码--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
```

*清单 20-15：将 `ThreadPool` 修改为保存 `Worker` 实例而非直接保存线程*

由于 `ThreadPool` 现在保存的是一些 `Worker` 实例而非 `JoinHandle<()>` 实例，因此咱们已将其上那个字段的名字，从 `threads` 修改为了 `workers`。咱们将那个 `for` 循环中的计数器，用作给 `Worker::new` 的参数，同时咱们将每个新的 `Worker`，存储在那个名为 `workers` 的矢量值中。

外部代码（就像 `src/main.rs` 中咱们的服务器），无需知悉 `ThreadPool` 里某个 `Worker` 结构体使用方面的实现细节，因此咱们是将这个 `Worker` 结构体及其 `new` 函数，构造为了私有。`Worker::new` 函数使用了咱们给他的那个 `id`，并将经由使用空闭包而生成一个新线程，而创建出的一个 `JoinHandle<()>` 实例存储起来。


> 注意：若操作系统因没有足够系统资源而无法创建出一个线程，那么 `thread::spawn` 就将终止运行。那样的话，即使一些线程创建可能成功，也将导致咱们整个服务器终止运行。为简化起见，这种实现做法是无可厚非的，但在生产的线程池实现中，咱们就大概打算使用 [`std::thread::Builder`](https://doc.rust-lang.org/std/thread/struct.Builder.html) 与他的返回 `Result` 的 [`spawn`](https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn) 方法了。

这段代码将编译，并将咱们指定给 `ThreadPool::new` 数目的 `Worker` 实例存储起来。但咱们 *仍* 未处理咱们在 `execute` 中得到的闭包。接下来就要看看怎样完成那一步。

#### 经由通道把请求发送给线程

**Sending Requests to Threads via Channels**

接下来咱们将要解决的，便是所给到 `thread::spawn` 的那些闭包什么也没做的问题。当前，咱们在那个 `execute` 方法中，获取到了咱们打算执行的那个闭包。但咱们需要于那个 `ThreadPool` 创建期间，在咱们创建出各个 `Worker` 时，给到 `thread::spawn` 一个闭包。

咱们想要咱们刚创建出的那些 `Worker` 结构体，从一个保存在 `ThreadPool` 中的队列中获取要运行的代码，并把那些代码发送他的线程运行。

第 16 章中咱们学过的通道 -- 两个线程间通信的一种简单方式 -- 对于这个用例将是最佳的。咱们将使用一个函数的通道，作为作业队列，the queue of jobs，而 `execute` 将把来自 `ThreadPool` 的某项作业，发送到那些 `Worker` 实例，其将把该项作业，发送给他的线程。下面便是这个方案：

1. `ThreadPool` 将创建出一个通道，并保存于 `sender` 上；
2. 每个 `Worker` 实例，将保存于 `receiver` 上；
3. 咱们将创建出将保存那些咱们打算下发到通道上闭包的一个新 `Job` 结构体；
4. `execute` 方法将经由那个 `sender`，发送其打算执行的作业；
5. 在 `Worker` 实例的线程中，其将遍历其 `receiver` 并执行其所接收到的任何作业的闭包。

咱们来通过在 `ThreadPool::new` 中创建一个通道，并在 `ThreadPool` 实例中保存 `send` 开始，如下清单 20-16 中所示。其中的 `Job` 结构体现在没有保存任何东西，但将保存咱们下发到通道项目类型。

文件名：`src/lib.rs`

```rust
use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}
```

*清单 20-16：将 `ThreadPool` 修改为存储传递 `Job` 实例通道的 `sender`*

在 `ThreadPool::new` 中，咱们创建出来咱们的新通道，并让线程池保存了该通道的 `sender`。这段代码将成功编译。

下面就来尝试在这个线程池创建出该通道时，把其 `receiver` 传入各个 `worker`。咱们清楚咱们是要在那些 `workers` 生成的线程中使用这个 `receiver`，因此咱们将在那个闭包中，引用这个 `receiver` 参数。下面清单 20-17 中的代码尚不会很好地编译。

文件名：`src/lib.rs`

```rust
impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}

// --跳过代码--
impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
```

*清单 20-17：将 `receiver` 传递给 `workers`*

咱们作出了一些小而简单直接的修改：咱们把那个 `receiver` 传入到 `Worker::new`，并随后在那个闭包里使用了他。

当咱们尝试检查这段代码时，就会得到如下报错：

```console
$ cargo check
    Checking hello v0.1.0 (/home/peng/rust-lang-zh_CN/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
22 |         let (sender, receiver) = mpsc::channel();
   |                      -------- move occurs because `receiver` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
...
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here, in previous iteration of loop

For more information about this error, try `rustc --explain E0382`.
error: could not compile `hello` due to previous error
```

这段代码试图将 `receiver` 传递给多个 `Worker` 实例。正如回顾到第 16 章，这样做不会工作：Rust 所提供的通道实现，属于多 `producer`、单 `consumer` 的。这意味着咱们不能只克隆通道的消费端来修复这段代码。咱们也不打算将一条消息，多次发送给多个消费者；咱们是要一个带有多个 `worker` 的消息列表，如此每条消息，都将被一次性处理。

此外，从通道队列里取出一项作业，还涉及到令 `receiver` 可变，因此这些县城就需要一种共用与修改 `receiver` 的安全方式；否则，咱们就会面临竞争情形（如同第 16 章中所讲到的）。

回顾第 16 章中曾讨论过的线程安全灵巧指针：为在多个线程间共用所有权，以及实现这些线程修改值，咱们需要用到 `Arc<Mutex<T>>`。`Arc` 类型将实现多个 `worker` 都拥有那个 `receiver`，而 `Mutex` 将确保某个时刻只有一个 `worker` 从 `receiver` 获取一项作业。下面清单 20-18 给出了咱们需要作出的修改。

文件名：`src/lib.rs`

```rust
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
// --跳过代码--

impl ThreadPool {
    // --跳过代码--
    pub fn new(size: usize) -> ThreadPool {
        assert! (size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // --跳过代码--
}

// --跳过代码--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --跳过代码--
    }
}
```

*清单 20-18：运用 `Arc` 与 `Mutex` 在那些 `worker` 间共用 `receiver`*

在 `ThreadPool::new` 中，咱们把 `receiver` 放入到一个 `Arc` 与一个 `Mutex` 中。对于各个新 `worker`，咱们克隆了那个 `Arc`，从而增加了引用计数，这样那些 `worker` 就可以共用 `receiver` 的所有权。

有了这些修改，代码就会编译了！咱们就要达到目的了！


#### 实现 `execute` 方法

**Implementing the `execute` method**

咱们来最终实现那个 `ThreadPool` 上的 `execute` 方法。咱们还将把 `Job` 从结构体，修改为保存着 `execute` 接收到闭包类型的特质对象的类型别名。正如第 19 章 [“使用类型别名创建类型同义词”](Ch19_Advanced_Features.md#creating-type-synonyms-with-type-aliases) 小节中曾讨论过的，类型别名实现了为易于使用而将长类型构造缩短。请看看下面清单 20-19.

文件名：`src/lib.rs`

```rust
// --跳过代码--

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // --跳过代码--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
// --跳过代码--
```

*清单 20-19：为保存着各个闭包的 `Box` 创建出 `Job` 类型别名，并于随后把作业下发到通道*

使用咱们在 `execute` 中得到的闭包创建出一个新的 `Job` 实例后，咱们便把那项作业下发到通道的发送端。对于发送失败的情形，咱们调用了 `send` 上的 `unwrap` 方法。在比如咱们停止全部线程执行，即表示接收端已停止接收新消息时，发送失败就可能发生。在那个时刻，咱们是无法停止咱们的线程执行的：只要这个线程池存在，咱们的线程就会继续执行。咱们使用 `unwrap` 的原因，就是咱们清楚这样的失败情况不会发生，但编译器是不了解这点的。

但咱们还没有大功告成！在 `worker` 里，传递给 `thread::spawn` 的闭包，仍然只 *引用* 了通道的接收端。相反，咱们需要闭包一直循环，向通道接收端请求一项作业，并在其获取到一项作业时运行该项作业。下面咱们就来完成下面清单 20-20 中所给出的对 `Worker::new` 的修改。

文件名：`src/lib.rs`

```rust
// --跳过代码--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println! ("Worker {id} 获取到一项作业；执行中。");

            job();
        });

        Worker { id, thread }
    }
}
```

*清单 20-20：在 `worker` 的线程中接收并执行作业*

这里，咱们首选调用了 `receiver` 上的 `lock` 来请求那个互斥量，mutex，并于随后调用 `unwrap` 来在出现任何错误时终止运行。在互斥量处于 *中毒，poisoned* 状态时，请求锁就会失败，在有别的某个线程终止运行的同时，持有着而没有释放该锁时，这种情况便会发生。在这种情况下，调用 `unrap` 来让这个线程终止运行，便是要采取的正确措施。请放心地把这个 `unwrap`，修改为一个带有对咱们有意义报错信息的 `expect`。

当咱们获得了那个互斥量上的锁时，咱们就会调用 `recv` 来从通道接收一个 `Job`。最后的 `unwrap` 也会带过这里的任何错误，在持有 `sender` 的线程已关闭时就会发生这些错误，就跟 `receiver` 关闭时那个 `send` 方法会返回 `Err` 类似。

到 `recv` 的调用会阻塞，因此在尚无作业时，当前线程将等待，直到有某项作业可用。`Mutex<T>` 确保了一次只有一个 `Worker` 线程是在尝试请求作业。

咱们的线程池现在就处于工作状态了！给他一次 `cargo run` 并构造一些请求：


```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\hello.exe`
Worker 0 获取到一项作业；执行中。
Worker 1 获取到一项作业；执行中。
Worker 2 获取到一项作业；执行中。
Worker 3 获取到一项作业；执行中。
Worker 0 获取到一项作业；执行中。
Worker 1 获取到一项作业；执行中。
Worker 1 获取到一项作业；执行中。
Worker 2 获取到一项作业；执行中。
Worker 3 获取到一项作业；执行中。
Worker 0 获取到一项作业；执行中。
Worker 1 获取到一项作业；执行中。
```

成功了！咱们现在有了一个会异步执行 TCP 连接的线程池。绝不会有超过四个线程被创建出来，因此在服务器收到很多请求时，咱们的系统将不会过载。在咱们构造了一个到 `/sleep` 的请求时，服务器通过让另一线程运行别的一些请求，而将能服务这些请求。

> 注意：若咱们在多个窗口同时打开 `/sleep`，他们可能会在设置的时间间隔每次加载一个。有些 web 浏览器会出于缓存原因，而顺序执行同一请求的多个实例。这样的局限并不是由咱们的服务器导致的。



这段代码将会编译及运行，但不会产生所需的线程行为：慢速请求仍将导致别的请求等待被处理。至于原因则有点微妙：由于锁的所有权是基于 `lock` 方法返回的 `LockResult<MutexGuard<T>>` 中，`MutexGuard<T>` 的生命周期，因此这个 `Mutex` 结构体没有公开的 `unlock` 方法。在编译时，借用检查器可于随后，就除非咱们拿着 `Mutex` 所守卫的某项资源的锁，否则无法访问该项资源这一规矩强制加以检查。但是，若咱们没有注意到 `MutexGuard<T>` 的生命周期，那么这样的实现同样能导致锁相较预期被占用更长时间。

由于在 `let` 之下，等号右侧的表达式中用到的任何临时值，都会在 `let` 语句结束时被立即丢弃，因此使用了 `let job = receiver.lock().unwrap().recv().unwrap();` 的清单 20-20 中代码是工作的。但是，`while let`（以及 `if let` 与 `match`） 则是在相关代码块结束前，不会丢弃那些临时值。在清单 20-21 中，
