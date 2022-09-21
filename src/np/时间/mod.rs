//! 时间: 时间相关功能

pub struct 定时器<'a> {
    // 单位: 秒 (s)
    时间: f32,

    // 闭包容器
    回调: Box<dyn Fn() -> () + 'a>,
}

impl<'a> 定时器<'a> {
    pub fn new<F>(时间: f32, 回调: F) -> Self
    where
        F: Fn() -> () + 'a,
    {
        Self {
            时间,
            回调: Box::new(回调),
        }
    }

    pub fn 调用(&self) {
        (self.回调)();
    }

    pub fn 取消(&mut self) {
        // TODO
    }
}

// 自动取消
impl<'a> Drop for 定时器<'a> {
    fn drop(&mut self) {
        self.取消();
    }
}

pub trait 时间接口<'a> {
    fn 取时间(&self) -> f32;

    /// setTimeout()
    fn 设超时(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a>;

    /// setInterval()
    fn 设周期(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a>;
}

pub struct 时间实现 {
    // TODO
}

impl 时间实现 {
    // TODO

    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> 时间接口<'a> for 时间实现 {
    fn 取时间(&self) -> f32 {
        // TODO
        0.0
    }

    fn 设超时(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a> {
        // TODO
        定时器::new(秒, 回调)
    }

    fn 设周期(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a> {
        // TODO
        定时器::new(秒, 回调)
    }
}
