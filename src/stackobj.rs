//the StackObj type represents the different types that can be pushed onto the stack

use std::fmt;
use std::ops;
use std::cmp;

#[derive(Copy, Clone)]
pub enum StackObj {
    Num(f32),
    Ref(u32),
}

impl fmt::Display for StackObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match *self {
            StackObj::Num(arg) => {
                if arg % 1.0 == 0.0 {
                    arg.to_string() + ".0"
                } else {
                    arg.to_string()
                }
            },
            StackObj::Ref(_arg) => "Reference".to_string(),
        };
        write!(f, "{}", res)
    }
}


//TODO: Add support for operations on references

impl ops::Add for StackObj {
    type Output = StackObj;
    fn add(self, right: StackObj) -> StackObj{
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => StackObj::Num(arg1 + arg2),
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}

impl ops::Sub for StackObj {
    type Output = StackObj;
    fn sub(self, right: StackObj) -> StackObj{
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => StackObj::Num(arg1 - arg2),
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}

impl ops::Mul for StackObj {
    type Output = StackObj;
    fn mul(self, right: StackObj) -> StackObj{
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => StackObj::Num(arg1 * arg2),
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}

impl ops::Div for StackObj {
    type Output = StackObj;
    fn div(self, right: StackObj) -> StackObj{
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => StackObj::Num(arg1 / arg2),
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}

impl cmp::PartialEq for StackObj {
    fn eq(&self, right: &StackObj) -> bool {
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => (arg1 == arg2),
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}

impl cmp::PartialOrd for StackObj {
    fn partial_cmp(&self, right: &StackObj) -> Option<cmp::Ordering> {
        match self {
            StackObj::Num(arg1) => {
                match right {
                    StackObj::Num(arg2) => {
                        arg1.partial_cmp(&arg2)
                    },
                    _ => panic!("Cannot perform operations on refs (yet)"),
                }
            }
            _ => panic!("Cannot perform operations on refs (yet)"),
        }
    }
}