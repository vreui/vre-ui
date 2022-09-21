//! vreui: 威惹 框架部分 (用户侧, wasm, vue+react+flutter/dart)
#![deny(unsafe_code)]

// 模块
#[cfg(feature = "np")]
mod np;

#[cfg(feature = "proxy")]
mod proxy;

// 导出 (np)
#[cfg(feature = "np")]
pub use np::响应::{响应实现, 状态类型};
#[cfg(feature = "np")]
pub use np::接口::{
    h, 取引擎, 威惹, 威惹引擎, 威惹接口, 威惹组件, 引擎初始化错误, 环境接口, 生命周期接口,
    组件属性, H,
};
#[cfg(feature = "np")]
pub use np::时间::{定时器, 时间接口};

#[cfg(feature = "np")]
pub use np::组件::{下级, 文本};

// TODO

// 全局测试
#[cfg(test)]
mod test;
