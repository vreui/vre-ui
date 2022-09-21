//! 响应: 响应式数据依赖功能, 类似 vue
use std::rc::Rc;

pub struct 状态类型<T: Clone> {
    内部_值: T,
}

// TODO
impl<T: Clone> 状态类型<T> {
    pub fn new(初始值: T) -> Self {
        Self {
            内部_值: 初始值
        }
    }

    pub fn 值(&self) -> T {
        self.内部_值.clone()
    }

    pub fn 设(&mut self, 值: T) {
        self.内部_值 = 值;
    }

    pub fn 更新(&mut self, 函数: impl Fn(T) -> T) {
        self.内部_值 = 函数(self.内部_值.clone());
    }
}

pub trait 响应接口<'a> {
    /// 类似 vue 3: ref()
    fn 状态<T: Clone>(&mut self, 初值: impl Fn() -> T) -> Rc<状态类型<T>>;

    /// 类似 vue 3: compute()
    fn 计算<T: Clone>(&mut self, 函数: impl Fn() -> T + 'a) -> Rc<状态类型<T>>;

    /// 类似 vue 3: watch()
    fn 监视<T: Clone>(&mut self, 数据: Rc<状态类型<T>>, 函数: impl Fn() -> () + 'a);
}

pub struct 响应实现 {
    //  TODO
}

impl 响应实现 {
    // TODO
}

// TODO
impl<'a> 响应接口<'a> for 响应实现 {
    fn 状态<T: Clone>(&mut self, 初值: impl Fn() -> T) -> Rc<状态类型<T>> {
        Rc::new(状态类型::new(初值()))
    }

    fn 计算<T: Clone>(&mut self, 函数: impl Fn() -> T + 'a) -> Rc<状态类型<T>> {
        Rc::new(状态类型::new(函数()))
    }

    fn 监视<T: Clone>(&mut self, _数据: Rc<状态类型<T>>, _函数: impl Fn() -> () + 'a) {
        // TODO
    }
}
