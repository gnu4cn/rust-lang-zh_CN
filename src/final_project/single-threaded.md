# 构建单线程的 Web 服务器

我们将从让单线程的 web 服务器运行开始。在开始之前，我们来先快速概述一下构建 web 服务器所涉及的协议。这些协议的细节超出了本书的范围，但简要概述就将给予咱们所需的信息。

Web 服务器涉及的两个主要协议，分别是 *超文本传输协议 HTTP* 和 *传输控制协议 TCP*。这两个协议都属于 *请求-响应* 协议，这意味着 *客户端* 发起请求，*服务器* 监听请求并向客户端提供响应。这些请求和响应的内容由协议定义。

TCP 属于底层协议，描述了信息如何从一台服务器传输到另一服务器的具体细节，但并未规定这些信息的内容。HTTP 通过定义请求和响应的内容，而构建于 TCP 之上。从技术上讲，于其他协议一起使用 HTTP 是可行的，但在绝大多数情况下，HTTP 都时通过 TCP 发送数据。我们将使用 TCP 和 HTTP 请求和响应的原始字节。


## 监听 TCP 连接

我们的 web 服务器需要监听 TCP 连接，因此这是我们将要处理的第一部分。标准库提供了 `std::net` 模组，让我们可以做到这一点。我们来以惯常方式构造一个新项目：

```console
$ cargo new hello
    Creating binary (application) `hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello
```

现在，请在 `src/main.rs` 中输入下面清单 21-1 中的代码来开始。这段代码将在本地地址 `127.0.0.1:7878` 上监听传入的 TCP 流。当他获取到传入流时，他将打印 `连接已建立！`。

<a name="listing_21-1"></a>
文件名：`projects/hello/src/main.rs`

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

**清单 21-1**：监听传入流，并在接收到流时打印消息

使用 `TcpListener`，我们可以监听地址 `127.0.0.1:7878` 上的 TCP 连接。在地址中，冒号之前的部分是个 IP 地之，代表咱们的计算机（这在每台计算机上都是相同的，并不特指本书作者的计算机），而 `7878` 是端口。我们选择这个端口有两个原因：HTTP 在这个端口上通常是不接受的，因此我们的服务器不太可能于咱们机器上运行的任何其他 web 服务器冲突，并且 `7878` 在电话上输入时听起来像 *rust*。

在这一情形下的 `bind` 函数的作用与 `new` 函数相似，将返回一个新的 `TcpListener` 实例。该函数称为 `bind`，因为在网络通信中，连接到某个要监听的端口称为 “绑定到端口”。

`bind` 函数返回 `Result<T, E>`，这表明绑定可能失败。例如，当我们运行了程序的两个实例，而因此让两个程序监听同一个端口时；由于我们只是出于学习目的而编写一个基础服务器，因此无需担心处理此类错误；相反，我们使用 `unwrap` 来在错误发生时停止程序。

`TcpListener` 上的 `incoming` 方法返回一个迭代器，给予我们一个流的序列（更具体地说，是 `TcpStream` 类型的流）。单个 *流* 代表客户端与服务器之间的开放连接。所谓 *连接，conneciton*，是指完整的请求和响应过程，其中客户端连接到服务器，服务器生成响应，然后服务器关闭连接。因此，我们将从 `TcpStream` 中读取以查看客户端发送的内容，然后将响应写入流以将数据发送回客户端。总的来说，这个 `for` 循环将依次处理每个连接，并生成一系列供我们处理的流。

目前，我们对流的处理包括

- 当流出现任何错误时，调用 `unwrap` 来终止程序；
- 当没有任何错误时，程序则打印一条消息。

我们将在下一代码清单中为流成功的情形添加更多功能。当客户端连接到服务器时，我们可能会从 `incoming` 方法收到错误的原因在于，我们实际上并不是在遍历连接。而是在遍历 *连接尝试*。连接可能因多种原因而失败，其中许多原因都是特定于操作系统的。例如，许多操作系统对可支持的同时打开连接的数量有限制；超过该数量的新连接尝试将引发错误，知道部分已打开的连接被关闭为止。

我们来尝试运行这段代码！在终端中执行 `cargo run`，然后在 web 浏览器中加载 `127.0.0.1:7878`。由于服务器当前尚未返回任何数据，浏览器应该显示 `Connection reset,` 的错误消息。但当咱们查看终端时，应该看到浏览器连接到服务器时打印的几条消息！

> **译注**：可使用 `curl 127.0.0.1:7878` 命令进行调试，且使用 `curl` 也是网络编程调试中常用的方法。

```console
   Compiling hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/hello`

连接已建立！
连接已建立！
连接已建立！
```

有时咱们会看到针对一次浏览器请求打印多条消息；原因可能是浏览器在请求页面时，同时也请求了其他资源，例如显示在浏览器标签页中的 `favicon.ico` 图标。

也可能是因为服务器没有响应任何数据，导致浏览器尝试多次连接到服务器。当 `stream` 超出作用域并在循环结束时被启用时，连接将作为 `drop` 实现的一部分而被关闭。浏览器有时会通过重试来处理关闭的连接，因为问题可能是暂时的。

浏览器有时也会在不发送任何请求的情况下，打开与服务器的多个连接，以便稍后 *真正* 发送请求时，这些请求可以更快地发生。当这种情况发生时，我们的服务器将发现到每个连接，无论连接上是否有请求。例如，许多基于 Chrome 的浏览器版本都会这样做；咱们可以通过使用隐私浏览模式，或使用其他浏览器来禁用这一优化。

重要的是我们已经成功获得了 TCP 连接的句柄！

请记住在运行完特定版本的代码后按下 `Ctrl-c` 来停止程序。然后，在完成每套代码修改后，要通过运行 `cargo run` 命令重启程序，以确保运行的是最新版本的代码。


## 读取请求

我们来实现读取来自浏览器的请求的功能！为了将首先获取连接，然后对连接采取一些操作的关注点分离，我们将开启一个新函数来处理连接。在这个新的 `handle_connection` 函数中，我们将读取 TCP 流中的数据并将其打印出来，以便我们可以查看浏览器发送的数据。请修改代码为下面清单 21-2 这样。

<a name="listing_21-2"></a>
文件名：`src/main.rs`

```rust
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
```

**清单 21-2**：从 `TcpStream` 读取并打印数据

我们带入 `std::io::prelude` 和 `std::io::BufReader` 到作用域，以访问允许我们从流中读取和向流中写入的特质与类型。在 `main` 函数的 `for` 循环中，我们不再打印表示我们已建立连接的消息，而是调用新的 `handle_connection` 函数，并传递 `stream` 给他。

在 `handle_connection` 函数中，我们创建了一个新的 `BufReader` 实例，封装了到 `stream` 的引用。`BufReader` 通过管理到 `std::io::Read` 特质方法的调用，为我们带来缓冲功能。

我们创建了一个名为 `http_request` 的变量，以收集浏览器发送到我们服务器的请求行。通过添加 `Vec<_>` 的类型注解，我们表明希望收集这些行到一个矢量值中。

`BufReader` 实现了 `std::io::BufRead` 特质，该特质提供了 `lines` 方法。`lines` 方法通过在遇到新行字节时分割数据流，返回一个 `Result<String, std::io::Error>` 类型的迭代器。为了获取每个 `String`，我们 `map` 并 `unwrap` 每个 `Result`。当数据不是有效的 UTF-8 编码，或者从流读取存在问题时，`Result` 可能就是错误。同样，生产程序应该更优雅地处理这些错误，但为了简单起见，我们选择了在错误情形下停止程序。

浏览器通过在一行中连续发送两个换行符来表示 HTTP 请求的结束。因此，为了从流中获取一次请求，我们会不断取得行，直到遇到空字符串的行为止。一旦我们收集这些行到矢量值中，我们就会使用美观的调试格式将他们打印出来，以便查看 web 浏览器发送给服务器的指令。

我们来试试这段代码！启动程序并再次在浏览器中发出请求。请注意，我们仍然会在浏览器中得到错误页面，但终端中的程序输出现在将类似于下面这样：

```console
$ cargo run
   Compiling hello v0.1.0 (/home/hector/rust-lang-zh_CN/projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "Connection: keep-alive",
    "sec-ch-ua: \"Google Chrome\";v=\"147\", \"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"147\"",
    "sec-ch-ua-mobile: ?0",
    "sec-ch-ua-platform: \"Linux\"",
    "DNT: 1",
    "Upgrade-Insecure-Requests: 1",
    "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-User: ?1",
    "Sec-Fetch-Dest: document",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Accept-Language: en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7,ko;q=0.6,zh-TW;q=0.5,ja;q=0.4,it;q=0.3,ro;q=0.2,de;q=0.1",
]
```

> **译注**：使用 `curl 127.0.0.1:7878` 的输出，如下面这样：
>
> ```console
> $ cargo run
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
>      Running `target/debug/hello`
> Request: [
>     "GET / HTTP/1.1",
>     "Host: 127.0.0.1:7878",
>     "User-Agent: curl/8.19.0",
>     "Accept: */*",
> ]
> ```

根据咱们的浏览器，咱们可能会得到略有不同的输出。现在我们正在打印请求数据，通过查看请求第一行中 `GET` 之后的路径，我们可以发现为何我们会从一次浏览器请求得到多个连接。当重复的连接都在请求 `/`，我们就知道浏览器正在重复获取 `/`，因为他没有从我们的程序得到响应。

我们来分写这一请求数据，以了解浏览器向我们的程序请求什么。


## 仔细观察 HTTP 请求

HTTP 属于基于文本的协议，请求采用以下格式：

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

第一行是 *请求行*，包含有关客户端请求的信息。请求行的第一部分指明了所使用的方法，比如 `GET` 或 `POST`，这描述了客户端是发出请求的方式。我们的客户端使用了 `GET` 请求，这意味着他正在请求信息。

请求行的下一部分是 `/`，表示客户端正在请求的 *同一资源标识符，URI*：URI 与 *统一资源定位符，URL* 几乎相同。URI 与 URL 之间的区别对于我们这一章的目的来说不重要，但 HTTP 规范使用 URI 这一术语，因此我们在这里可以将 *URL* 理解为 *URI*。

最后一部分是客户端使用的 HTTP 版本，随后请求行以 CRLF 序列结束。（CRLF 代表 *回车，carriage return* 和 *换行，line fedd*，这是打字机时代的术语！）CRLF 序列也可以写成 `\r\n`，其中的 `\r` 是回车符，`\n` 是换行符。CRLF 序列将请求行与请求数据的其余部分分开。请注意，当打印 CRLF 时，我们会看到一个新行开始，而不是 `\r\n`。

查看到目前为止运行程序所接收的请求行数据，我们可以看到，`GET` 是请求方法，`/` 是请求 URI，`HTTP/1.1` 是版本。

在请求行之后，从 `Host:` 开始的其余行都属于头部。`GET` 请求没有请求体。

请尝试从不同浏览器发出请求，或是请求不同的地址，比如 `127.0.0.1:7878/test`，以查看请求数据如何变化。

> **译注**：运行 `curl 127.0.0.1:7878/test` 时，请求数据如下所示：
>
> ```console
> Request: [
>     "GET /test HTTP/1.1",
>     "Host: 127.0.0.1:7878",
>     "User-Agent: curl/8.19.0",
>     "Accept: */*",
> ]
> ```

现在我们知道浏览器请求了什么，我们就来发回一些数据！


## 编写响应

我们将实现发送数据以响应客户端请求。响应有着以下格式：

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


## 返回真正的 HTML

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


## 对请求加以验证并有选择地进行响应

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


## 初试重构

**A Touch of Refactoring**


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





（End）


