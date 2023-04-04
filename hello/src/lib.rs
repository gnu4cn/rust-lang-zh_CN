#![allow(warnings)]
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 创建出一些线程并将他们存储在那个矢量中
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
