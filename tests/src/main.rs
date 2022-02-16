#![feature(const_trait_impl, const_fn_trait_bound, const_mut_refs)]

use flagstack::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExampleFlags {
    Flag1,
    Flag2,
    IsStack,
    IsConst,
    Revoked,
    Max,
}

pub use ExampleFlags::*;

impl const ConstFlaggable<Self> for ExampleFlags {
    fn const_contains<const TAG: u8>(&self) -> bool {
        *self as u8 == TAG
    }
}

impl const Into<u8> for ExampleFlags {
    fn into(self) -> u8 {
        self as u8
    }
}

impl const From<u8> for ExampleFlags {
    fn from(t: u8) -> Self {
        Self::from_u8(t)
    }
}

impl const Flag for ExampleFlags {
    const REVOKED: Self = Revoked;
    const MAX: u8 = Max as u8;
    const IS_STACK: Self = IsStack;
    const IS_CONST: Self = IsConst;
    fn from_u8(t: u8) -> Self {
        if t > Max as u8 {
            panic!("Tried to convert u8 bigger than TypeTag::Max to TypeTag")
        } else {
            unsafe { std::mem::transmute(t as u8) }
        }
    }
}

impl const DynFlaggable<Self> for ExampleFlags {
    fn contains(&self, tag: ExampleFlags) -> bool {
        *self as u8 == tag as u8
    }

    fn tag(&mut self, tag: ExampleFlags) {
        *self = tag
    }

    fn untag(&mut self, tag: ExampleFlags) {
        if *self as u8 == tag as u8 {
            *self = Revoked
        } else {
            panic!("Untag missmatch")
        }
    }
}

pub fn var_main() {
    let stack = flagstack!(const ExampleFlags);

    let stack = stack.push_tag::<{ Flag1 as u8 }>();
    let stack = stack.push_tag::<{ Flag2 as u8 }>();

    assert!(stack.pop_tag().1.contains(Flag2));
    assert!(!stack.pop_tag().1.contains(Flag1));
}

pub fn const_main() {
    const STACK: ConstFlagStack<'static, ExampleFlags, { Revoked.into() }> =
        ConstFlagStack(&Revoked);

    const STACK_2: ConstFlagStack<ExampleFlags, { Flag1.into() }> =
        STACK.push_tag::<{ Flag1 as u8 }>();

    const STACK_3: ConstFlagStack<ExampleFlags, { Flag2.into() }> =
        STACK_2.push_tag::<{ Flag2 as u8 }>();

    const IS_TRUE: bool = STACK_3.pop_tag().1.const_contains::<{ Flag2 as u8 }>();
    assert!(IS_TRUE);
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>());
}

pub fn dyn_main() {
    let mut stack = flagstack!(ExampleFlags);

    stack.push_tag(Flag1);
    stack.push_tag(Flag2);

    assert!(stack.pop_tag().contains(Flag2));
    assert!(!stack.pop_tag().contains(Flag1));
}

fn main() {
    #[cfg(feature = "constant")]
    const_main();
    #[cfg(feature = "dynamic")]
    dyn_main();
    #[cfg(feature = "variable")]
    var_main();
}
