//! vreui: 威惹 框架部分 (用户侧, wasm, vue+react+flutter/dart)

#![deny(unsafe_code)]

// 模块
#[path = "响应/mod.rs"]
mod 响应;
#[path = "接口/mod.rs"]
mod 接口;
#[path = "时间/mod.rs"]
mod 时间;
#[path = "核心/mod.rs"]
mod 核心;
#[path = "桥/mod.rs"]
mod 桥;

#[path = "布局/mod.rs"]
mod 布局;
#[path = "渲染/mod.rs"]
mod 渲染;
#[path = "组件/mod.rs"]
mod 组件;

#[path = "内部_文本/mod.rs"]
mod 内部_文本;
#[path = "动画/mod.rs"]
mod 动画;
#[path = "图片/mod.rs"]
mod 图片;
#[path = "异步/mod.rs"]
mod 异步;
#[path = "线程/mod.rs"]
mod 线程;
#[path = "输入/mod.rs"]
mod 输入;

// 导出
pub use 响应::{响应实现, 状态类型};
pub use 接口::{
    h, 取引擎, 威惹, 威惹引擎, 威惹接口, 威惹组件, 引擎初始化错误, 环境接口, 生命周期接口,
    组件属性, H,
};
pub use 时间::{定时器, 时间接口};

pub use 组件::{下级, 文本};

// TODO

// 全局测试
#[cfg(test)]
mod test;
