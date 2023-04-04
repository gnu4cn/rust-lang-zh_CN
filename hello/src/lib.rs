#![allow(warnings)]
pub struct ThreadPool;

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

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
