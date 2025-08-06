use super::tokens::Type;

#[derive(PartialEq)]
pub enum WrappedVal {
    CallStack(u32),
    Int(i32),
    Dcml(f64),
    Bool(bool),
    String(u16),
}
impl WrappedVal {
    pub fn type_enum(&self) -> Type {
        match self {
            WrappedVal::CallStack(_) => Type::CallStack,
            WrappedVal::Int(_) => Type::Int,
            WrappedVal::Dcml(_) => Type::Dcml,
            WrappedVal::Bool(_) => Type::Bool,
            WrappedVal::String(_) => Type::String,
        }
    }
}
impl std::ops::Add for WrappedVal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(rhs)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs + rhs)
            }
            WrappedVal::Dcml(lhs) if matches!(rhs, WrappedVal::Dcml(rhs)) => {
                let WrappedVal::Dcml(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Dcml(lhs + rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::Sub for WrappedVal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(rhs)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs - rhs)
            }
            WrappedVal::Dcml(lhs) if matches!(rhs, WrappedVal::Dcml(rhs)) => {
                let WrappedVal::Dcml(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Dcml(lhs - rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::Mul for WrappedVal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(rhs)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs * rhs)
            }
            WrappedVal::Dcml(lhs) if matches!(rhs, WrappedVal::Dcml(rhs)) => {
                let WrappedVal::Dcml(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Dcml(lhs * rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::Div for WrappedVal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs / rhs)
            }
            WrappedVal::Dcml(lhs) if matches!(rhs, WrappedVal::Dcml(_)) => {
                let WrappedVal::Dcml(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Dcml(lhs / rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::Rem for WrappedVal {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let WrappedVal::Int(lhs) = self else {
            unreachable!();
        };
        let WrappedVal::Int(rhs) = rhs else {
            unreachable!();
        };
        WrappedVal::Int(lhs % rhs)
    }
}
impl std::ops::BitAnd for WrappedVal {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs & rhs)
            }
            WrappedVal::Bool(lhs) if matches!(rhs, WrappedVal::Bool(_)) => {
                let WrappedVal::Bool(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Bool(lhs & rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::BitOr for WrappedVal {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs | rhs)
            }
            WrappedVal::Bool(lhs) if matches!(rhs, WrappedVal::Bool(_)) => {
                let WrappedVal::Bool(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Bool(lhs | rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::ops::BitXor for WrappedVal {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            WrappedVal::Int(lhs) if matches!(rhs, WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Int(lhs ^ rhs)
            }
            WrappedVal::Bool(lhs) if matches!(rhs, WrappedVal::Bool(_)) => {
                let WrappedVal::Bool(rhs) = rhs else {
                    unreachable!();
                };
                WrappedVal::Bool(lhs ^ rhs)
            }
            _ => unreachable!(),
        }
    }
}
impl std::cmp::PartialOrd for WrappedVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lt(other) {
            Some(std::cmp::Ordering::Less)
        } else if self.gt(other) {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self {
            WrappedVal::Int(lhs) if matches!(other, &WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = other else {
                    unreachable!();
                };
                lhs < rhs
            }
            WrappedVal::Dcml(lhs) if matches!(other, &WrappedVal::Dcml(_)) => {
                let WrappedVal::Dcml(rhs) = other else {
                    unreachable!();
                };
                lhs < rhs
            }
            _ => unreachable!(),
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self {
            WrappedVal::Int(lhs) if matches!(other, &WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = other else {
                    unreachable!();
                };
                lhs <= rhs
            }
            WrappedVal::Dcml(lhs) if matches!(other, &WrappedVal::Dcml(_)) => {
                let WrappedVal::Dcml(rhs) = other else {
                    unreachable!();
                };
                lhs <= rhs
            }
            _ => unreachable!(),
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self {
            WrappedVal::Int(lhs) if matches!(other, &WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = other else {
                    unreachable!();
                };
                lhs > rhs
            }
            WrappedVal::Dcml(lhs) if matches!(other, &WrappedVal::Dcml(_)) => {
                let WrappedVal::Dcml(rhs) = other else {
                    unreachable!();
                };
                lhs > rhs
            }
            _ => unreachable!(),
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match self {
            WrappedVal::Int(lhs) if matches!(other, &WrappedVal::Int(_)) => {
                let WrappedVal::Int(rhs) = other else {
                    unreachable!();
                };
                lhs >= rhs
            }
            WrappedVal::Dcml(lhs) if matches!(other, &WrappedVal::Dcml(_)) => {
                let WrappedVal::Dcml(rhs) = other else {
                    unreachable!();
                };
                lhs >= rhs
            }
            _ => unreachable!(),
        }
    }
}
