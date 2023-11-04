use crate::runner::one_er;
pub use runner::one_er::private_ee; //可用pub将use导出

//使用外部包
use rand::Rng;
//标准库std也被视为外部库，但是不需要手动添加

//使用嵌套路径
use std::{cmp::Ordering, io};
use std::{self, write};// == use std::io & use std::io::write

use std::collections::* //全部引用，谨慎使用

//声明其他文件的模块
mod house;

fn main() {
    //cargo的文件系统分为package crate module三类，package就是项目，有一个cargo.toml crate分为二进制binary crate（可编译为执行文件）
    //以及library crate。 main.re and lib.rs分别是两种crate的crate root

    let e = private_ee {
        long: 828,
    }
}

//module
mod runner {
    pub mod one_yi {
        // use super::all;
        fn get_one() {
            //调用子模块是private的，而所偶祖先模块可以随便调用
            crate::runner::all();
            super::all();
        }
        pub fn set_one() {}
    }

    pub mod one_er {
        fn get_two() {}

        pub struct erer {
            name: String, //struct内部的东西也默认是私有的
            pub age: i32,
        }

        impl erer {
            pub fn new() -> erer {
                erer {
                    name: String::from("value"), //此处为面向对象的封装理念。私有的name被封装了
                    age: 2,
                }
            }
        }

        pub struct private_ee {
            // int: i32,
            pub long: u64,
        }
    }

    fn all() {}
}

mod meal {
    pub enum Dinner { //enum里面的元素不需要Pub
        Apple,
        Juice,
    }
}

use runner::one_yi::{self, set_one}; //use关键字引用到当前作用域
//use一般习惯指定到父级
//struct enum 指定到本身

pub fn run() {
    

    crate::runner::one_yi::set_one(); //绝对路径（当前crate开始）
    //error: module `one_yi` is private
    //mod内默认是私有的
    let er = one_er::erer::new(); //封装

    set_one(); //use的效果

    //同名条目指定到父级
    use std::fmt::Result;
    use std::io::Result as IoResult;//为同名但是不同路径的struct指定一个别名
    //如下
    // fn f1() -> Result {}
    // fn f2() -> IoResult {}
}
