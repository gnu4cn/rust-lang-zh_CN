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

第一行是 *状态行*，包含响应中使用的 HTTP 版本、概括请求结果的数字状态码，以及提供状态码文本描述的原因短语。CRLF 序列之后是任何的头部、另一个 CRLF 序列，以及响应正文。

下面是一个示例响应，使用 HTTP 版本 1.1，状态码为 `200`、`OK` 原因短语、没有头部，也没有正文。

```text
HTTP/1.1 200 OK\r\n\r\n
```

状态代码 `200` 是标准的成功响应。这段文本便是个简短的成功 HTTP 响应。我们来写入这个响应到流，作为对成功请求的响应！在 `handle_connection` 函数中，移除原本用于打印请求数据的 `println!`，并以下面清单 21-3 中的代码替换他。

<a name="listing_21-3"></a>
文件名：`src/main.rs`

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
```

**清单 21-3**：写入一个极小的成功 HTTP 响应到流

第一个新行定义了 `response` 变量，包含成功消息的数据。然后，我们对 `response` 调用 `as_bytes`，转换字符串数据为字节。`stream` 上的 `write_all` 方法取一个 `&[u8]`，并直接发送这些字节到连接。由于 `write_all` 操作可能失败，因此我们像之前一样对任何错误结果使用 `unwrap`。同样，在真实应用中，咱们将在这里添加错误处理。

在这些修改下，我们来运行代码并发出请求。我们不再打印任何数据到终端，因此除了 Cargo 的输出外，我们不会看到其他任何输出。当咱们在 web 浏览器中加载 `127.0.0.1:7878` 时，应得到一个空白页而不是报错。咱们刚刚亲手编写了接收 HTTP 请求并发送响应的代码！

> **译注**：使用 `curl 127.0.0.1:7878 -v` 命令测试现在的 web 服务器，输出如下。
>
> ```console
> $ curl 127.0.0.1:7878 -v
> *   Trying 127.0.0.1:7878...
> * Established connection to 127.0.0.1 (127.0.0.1 port 7878) from 127.0.0.1 port 51618
> * using HTTP/1.x
> > GET / HTTP/1.1
> > Host: 127.0.0.1:7878
> > User-Agent: curl/8.19.0
> > Accept: */*
> >
> * Request completely sent off
> < HTTP/1.1 200 OK
> * no chunk, no close, no size. Assume close to signal end
> <
> * shutting down connection #0
> ```


## 返回真实的 HTML

我们来实现返回非空白页面的功能。请在项目目录的根目录下，而非 `src` 目录中，创建新文件 `hello.html`。咱们可输入任何咱们想要的 HTML；下面清单 21-4 展示了一种可能。

<a name="listing_21-4"></a>
文件名：`projects/hello/hello.html`

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

**清单 21-4**：在响应中返回的示例 HTML 文件

这是个最小的 HTML5 文档，带有标题和一些文本。为了在收到请求时从服务器返回这个文档，我们将按照下面清单 21-5 中所示，修改 `handle_connection` 以读取这个 HTML 文件，作为响应正文添加到响应，然后发送他。

<a name="listing_21-5"></a>
文件名：`projects/hello/src/main.rs`

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
// -- 跳过代码 --

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

**清单 21-5**：作为响应正文发送 `hello.html` 的内容

我们已添加 `fs` 到 `use` 语句中，以带入标准库的文件系统模组到作用域。读取文件内容到字符串中的代码应该看起来很熟悉；我们在 [清单 12-4](../io_project/reading_a_file.md#listing_12-4) 中为 I/O 项目读取文件内容时就用过他。

接下来，我们使用 `format!` 宏，以添加文件内容为成功响应的正文。为了确保有效的 HTTP 响应，我们要添加 `Content-Length` 头部，其被设置为响应正文的大小 -- 在这一情形下，就是 `hello.html` 的大小。

通过 `cargo run` 运行这段代码，并在浏览器中加载 `127.0.0.1:7878`；咱们应能看到咱们的 HTML 被渲染了！

目前，我们忽略了 `http_request` 中的请求数据，而仅无条件地发回 HTML 文件的内容。这就意味着，当咱们在浏览器中尝试请求 `127.0.0.1:7878/something-else` 时，将仍然得到这一相同的 HTML 响应。目前，我们的服务器非常有限，不会实现大多数 web 服务器能做的事情。我们希望根据请求自定义响应，进而只针对到 `/` 的格式正确的请求，发回这个 HTML 文件。


## 验证请求并有选择地响应

目前，无论客户端请求什么，我们的 web 服务器都会返回该文件中的 HTML。我们来添加功能，在返回 HTML 文件之前先检查浏览器是否请求了 `/`，并在浏览器请求了其他路径时返回错误。为此，我们需要 `handle_connection`，如下清单 21-6 中所示。这段新代码会将收到的请求，与我们已知的 `/` 请求格式对比，并添加 `if` 和 `else` 代码块来以不同方式处理请求。

<a name="listing_21-6"></a>
文件名：`src/main.rs`

```rust
// -- 跳过代码 --

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}
```

**清单 21-6**：以不同于其他请求方式，处理到 `/` 的请求

我们只会查看 HTTP 请求的第一行，因此我们不会读取整个请求到矢量值中，而是调用 `next` 获取迭代器中的第一个项目。第一个 `unwrap` 处理 `Option`，并在迭代器没有项目时停止程序。第二个 `unwrap` 处理 `Result`，并与清单 21-2 中添加的 `map` 里的 `unwrap` 有着相同效果。

接下来，我们检查 `request_line` 是否等于到 `/` 路径的 `GET` 请求的请求行。当等于时，`if` 代码块就返回 HTML 文件的内容。

当 `request_line` *不* 等于到 `/` 路径的 `GET` 请求时，则意味着我们收到了其他请求。稍后，我们将添加代码到 `else` 代码块，以响应所有其他请求。

现在请运行这段代码并请求 `127.0.0.1:7878`；咱们应该获取到 `hello.html` 中的 HTML。当咱们发出任何其他请求时，比如 `127.0.0.1:7878/something-else`，咱们将得到连接错误，就像咱们在运行清单 21-1 及清单 21-2 中的代码时看到的那样。

> **译注**：当通过命令 `curl 127.0.0.1:7878/something-else` 连接现在的服务器时，输出如下。
>
> ```console
> $ curl 127.0.0.1:7878/something-else
> curl: (52) Empty reply from server
> ```

现在，我们来将添加下面清单 21-7 中的代码到 `else` 代码块，以返回状态代码为 `404` 的响应，这表示请求的内容未找到。我们还将返回一些 HTML 代码，以便在浏览器中渲染页面，向最终用户表示响应。

<a name="listing_21-7"></a>
文件名：`projects/hello/src/main.rs`

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

**清单 21-7**：当请求除 `/` 外的任何路径时，响应以状态代码 `404` 和错误页面

在这里，我们的响应包含一个状态行，有着状态代码 `404` 以及原因短语 `NOT FOUND`。响应正文是文件 `404.html` 中的 HTML。咱们需要在 `hello.html` 旁边创建一个 `404.html` 文件作为错误页面； 同样，咱们可以随意使用任何 HTML，或者使用下面清单 21-8 中的示例 HTML。

<a name="listing_21-8"></a>
文件名：`projects/hello/404.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>你好！</title>
  </head>
  <body>
    <h1>哎呀！</h1>
    <p>抱歉，我不明白你请求的是什么。</p>
  </body>
</html>
```

**清单 21-8**：与任何 404 响应一起发回的页面的示例内容

在这些修改下，再次运行咱们的服务器。请求 `127.0.0.1:7878` 应返回 `hello.html` 的内容，而任何其他请求，比如 `127.0.0.1:7878/foo`，都应返回 `404.html` 中的报错 HTML。

> **译注**：命令 `curl 127.0.0.1:7878/foo -v` 的输出如下。
>
> ```console
> $ curl 127.0.0.1:7878/foo -v
> *   Trying 127.0.0.1:7878...
> * Established connection to 127.0.0.1 (127.0.0.1 port 7878) from 127.0.0.1 port 50990
> * using HTTP/1.x
> > GET /foo HTTP/1.1
> > Host: 127.0.0.1:7878
> > User-Agent: curl/8.19.0
> > Accept: */*
> >
> * Request completely sent off
> < HTTP/1.1 404 NOT FOUND
> < Content-Length: 215
> <
> <!DOCTYPE html>
> <html lang="en">
>   <head>
>     <meta charset="utf-8">
>     <title>你好！</title>
>   </head>
>   <body>
>     <h1>哎呀！</h1>
>     <p>抱歉，我不明白你请求的是什么。</p>
>   </body>
> </html>
> * Connection #0 to host 127.0.0.1:7878 left intact
> ```

## 重构

目前，`if` 与 `else` 两个代码块有着很多重复：他们都在读取文件并写入文件内容到流。唯二区别就是状态行和文件名。我们来提取这些差异到单独的 `if` 和 `else` 行，把状态行和文件名指派给变量；这样，我们就可以在代码中，无条件地使用这两个变量来读取文件并写入响应。下面清单 21-9 展示了替换大段的 `if` 与 `else` 代码块后的最终代码。

<a name="listing_21-9"></a>
文件名：`projects/hello/src/main.rs`

```rust
// -- 跳过代码 --

fn handle_connection(mut stream: TcpStream) {
    // -- 跳过代码 --

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

**清单 21-9**：重构 `if` 和 `else` 代码块，以仅包含两种情形之间不同的代码

现在，`if` 和 `else` 代码块仅返回元组中的状态行与文件名的相应值；然后，我们使用正如第 19 章讨论的 [`let` 语句中的模式](../patterns/all_places.md#let-语句)，使用解构特性将这两个值指派给 `status_line` 与 `filename`。

之前重复的代码现在位于 `if` 与 `else` 代码块之外，并使用了 `status_line` 与 `filename` 两个变量。这样更容易看出两种情况之间的区别，而且这意味着当我们打算更改文件读取和响应写入的方式时，只有一个地方要更新代码。清单 21-9 中的代码的行为将与清单 21-8 中的相同。

太棒了！现在我们仅以大约 40 行 Rust 代码，就实现了个简单的 web 服务器，他会以一个内容页面响应一次请求，并以一个 404 的响应，响应所有其他请求。

目前，我们的服务器在单线程下运行，这意味着他一次只能服务一个请求。我们来通过模拟一些缓慢请求，来看看这会如何成为一个问题。然后，我们将修复他，以便我们的服务器可以同时处理多个请求。


