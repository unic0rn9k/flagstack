#![feature(const_trait_impl, const_fn_trait_bound)]

use tagstack::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeTagEnum {
    Tag,
    Tag2,
    IsTagList,
    IsConst,
    Revoked,
    Max,
}

pub use TypeTagEnum::*;

impl const ConstTaggable<Self> for TypeTagEnum {
    fn const_contains<const TAG: u8>(&self) -> bool {
        *self as u8 == TAG
    }
}

impl const Into<u8> for TypeTagEnum {
    fn into(self) -> u8 {
        self as u8
    }
}

impl const From<u8> for TypeTagEnum {
    fn from(t: u8) -> Self {
        Self::from_u8(t)
    }
}

impl const TypeTag for TypeTagEnum {
    const REVOKED: Self = Revoked;
    const MAX: u8 = Max as u8;
    const IS_STACK: Self = IsTagList;
    const IS_CONST: Self = IsConst;
    fn from_u8(t: u8) -> Self {
        if t > Max as u8 {
            panic!("Tried to convert u8 bigger than TypeTag::Max to TypeTag")
        } else {
            unsafe { std::mem::transmute(t as u8) }
        }
    }
}

impl DynTaggable<Self> for TypeTagEnum {
    fn contains(&self, tag: TypeTagEnum) -> bool {
        *self == tag
    }

    fn tag(&mut self, tag: TypeTagEnum) {
        *self = tag
    }

    fn untag(&mut self, tag: TypeTagEnum) {
        if *self == tag {
            *self = Revoked
        } else {
            panic!(
                "Cannot revoke TypeTag::{:?} that doesn't match tag found in self ({:?})",
                tag, self
            )
        }
    }
}

pub fn var_main() {
    let stack = tagstack!(const TypeTagEnum);

    let stack = stack.push_tag::<{ Tag as u8 }>();
    let stack = stack.push_tag::<{ Tag2 as u8 }>();

    assert!(stack.pop_tag().1.contains(Tag2));
    assert!(!stack.pop_tag().1.contains(Tag));
}

pub fn const_main() {
    const STACK: ConstTypeTagStack<'static, TypeTagEnum, { Revoked.into() }> =
        ConstTypeTagStack(&Revoked);

    const STACK_2: ConstTypeTagStack<TypeTagEnum, { Tag.into() }> =
        STACK.push_tag::<{ Tag as u8 }>();

    const STACK_3: ConstTypeTagStack<TypeTagEnum, { Tag2.into() }> =
        STACK_2.push_tag::<{ Tag2 as u8 }>();

    const IS_TRUE: bool = STACK_3.pop_tag().1.const_contains::<{ Tag2 as u8 }>();
    assert!(IS_TRUE);
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as u8 }>());
}

pub fn dyn_main() {
    let mut stack = tagstack!(TypeTagEnum);

    stack.push_tag(Tag);
    stack.push_tag(Tag2);

    assert!(stack.pop_tag().contains(Tag2));
    assert!(!stack.pop_tag().contains(Tag));
}

fn main() {
    #[cfg(feature = "constant")]
    const_main();
    #[cfg(feature = "dynamic")]
    dyn_main();
    #[cfg(feature = "variable")]
    var_main();
}
