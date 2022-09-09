//! 接口: vreui 主要 API
use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use super::响应::{响应实现, 响应接口, 状态类型};
use super::时间::{定时器, 时间实现, 时间接口};

// TODO 保存闭包  Fn() -> ()
pub struct 闭包容器<'a> {
    闭包: Box<dyn Fn() -> () + 'a>,
}

impl<'a> 闭包容器<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> () + 'a,
    {
        Self {
            闭包: Box::new(f)
        }
    }
}

/// 威惹 的组件 (代码) 是一个函数 (类似 react)
pub type 威惹组件<T> = fn(v: &mut 威惹, p: &组件属性<T>) -> ();

pub trait 威惹接口<'a> {
    /// 组件的渲染输出
    fn 渲染(&mut self, 函数: impl Fn() -> H<'a> + 'a);

    /// 请求帧: 类似 requestAnimationFrame()
    fn 请求帧(&mut self, 函数: impl Fn() -> () + 'a);
}

// TODO
pub trait 环境接口 {
    /// 类似 vue 3: provide()
    fn 提供(&mut self);

    /// 类似 vue 3: inject()
    fn 环境(&self);
}

/// 组件的生命周期
pub trait 生命周期接口<'a> {
    /// 类似 vue 3: onMounted()
    fn 已挂载(&mut self, 函数: impl Fn() -> () + 'a);

    /// 类似 vue 3: onBeforeUnmount()
    fn 将卸载(&mut self, 函数: impl Fn() -> () + 'a);

    /// 类似 vue 3: onUnmounted()
    fn 已卸载(&mut self, 函数: impl Fn() -> () + 'a);
}

struct 功能集 {
    pub 时间: RefCell<时间实现>,
    pub 响应: RefCell<响应实现>,
}

/// 威惹 的主要 API
pub struct 威惹<'a> {
    // 上级节点
    上级: Option<Rc<Self>>,

    // 通用功能
    功能: Rc<功能集>,

    // 闭包容器
    渲染闭包: Option<Box<dyn Fn() -> H<'a> + 'a>>,
    请求帧闭包: Option<Box<dyn Fn() -> () + 'a>>,
    // TODO
}

impl<'a> 威惹<'a> {
    // TODO
}

impl<'a> 威惹接口<'a> for 威惹<'a> {
    fn 渲染(&mut self, 函数: impl Fn() -> H<'a> + 'a) {
        // TODO
        self.渲染闭包 = Some(Box::new(函数));
    }

    fn 请求帧(&mut self, 函数: impl Fn() -> () + 'a) {
        // TODO
        self.请求帧闭包 = Some(Box::new(函数));
    }
}

impl<'a> 环境接口 for 威惹<'a> {
    fn 提供(&mut self) {
        // TODO
    }

    fn 环境(&self) {
        // TODO
    }
}

impl<'a> 生命周期接口<'a> for 威惹<'a> {
    fn 已挂载(&mut self, _函数: impl Fn() -> () + 'a) {
        // TODO
    }

    fn 将卸载(&mut self, _函数: impl Fn() -> () + 'a) {
        // TODO
    }

    fn 已卸载(&mut self, _函数: impl Fn() -> () + 'a) {
        // TODO
    }
}

impl<'a> 时间接口<'a> for 威惹<'a> {
    fn 取时间(&self) -> f32 {
        self.功能.时间.borrow().取时间()
    }

    fn 设超时(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a> {
        let mut 时间 = self.功能.时间.borrow_mut();
        时间.设超时(秒, 回调)
    }

    fn 设周期(&mut self, 秒: f32, 回调: impl Fn() -> () + 'a) -> 定时器<'a> {
        let mut 时间 = self.功能.时间.borrow_mut();
        时间.设周期(秒, 回调)
    }
}

impl<'a> 响应接口<'a> for 威惹<'a> {
    fn 状态<T: Clone>(&mut self, 初值: impl Fn() -> T) -> Rc<状态类型<T>> {
        let mut 响应 = self.功能.响应.borrow_mut();
        响应.状态(初值)
    }

    fn 计算<T: Clone>(&mut self, 函数: impl Fn() -> T + 'a) -> Rc<状态类型<T>> {
        let mut 响应 = self.功能.响应.borrow_mut();
        响应.计算(函数)
    }

    fn 监视<T: Clone>(&mut self, 数据: Rc<状态类型<T>>, 函数: impl Fn() -> () + 'a) {
        let mut 响应 = self.功能.响应.borrow_mut();
        响应.监视(数据, 函数)
    }
}

/// 传递给威惹组件的属性数据 (props)
pub struct 组件属性<T> {
    // TODO 属性的值
    d: T,
}

impl<T> 组件属性<T> {
    pub fn new(d: T) -> Self {
        Self { d }
    }

    pub fn 值(&self) -> &T {
        &self.d
    }
}

/// h() 函数的返回值
pub struct H<'a> {
    // 闭包容器
    闭包: Box<dyn FnOnce(&mut 威惹) -> () + 'a>,
}

impl<'a> H<'a> {
    pub fn new(调用: impl FnOnce(&mut 威惹) -> () + 'a) -> Self {
        Self {
            闭包: Box::new(调用),
        }
    }
}

/// 用于创建组件的辅助函数
pub fn h<'a, T: 'a>(组件: 威惹组件<T>, 属性: T) -> H<'a> {
    let 调用 = move |v: &mut 威惹| {
        // TODO
        let 属性数据 = 组件属性::new(属性);
        // TODO
        组件(v, &属性数据);
    };
    H::new(调用)
}

/// 使用引擎的入口
pub struct 威惹引擎 {
    // TODO
}

impl 威惹引擎 {
    /// 启动界面渲染
    pub fn 渲染<T>(&mut self, _组件: 威惹组件<T>, _属性: T) {
        // TODO
    }
}

#[derive(Debug)]
pub struct 引擎初始化错误 {
    /// 错误描述
    pub m: String,
}

impl Display for 引擎初始化错误 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.m)?;
        Ok(())
    }
}

impl Error for 引擎初始化错误 {}

/// 初始化引擎
pub fn 取引擎() -> Result<威惹引擎, 引擎初始化错误> {
    // TODO
    Ok(威惹引擎 {})
}
