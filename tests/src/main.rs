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
    fn const_contains<const TAG: usize>(&self) -> bool {
        *self as usize == TAG
    }
}

impl const Into<usize> for TypeTagEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl const From<usize> for TypeTagEnum {
    fn from(t: usize) -> Self {
        Self::from_usize(t)
    }
}

impl const TypeTag for TypeTagEnum {
    const REVOKED: Self = Revoked;
    const MAX: usize = Max as usize;
    const IS_STACK: Self = IsTagList;
    const IS_CONST: Self = IsConst;
    fn from_usize(t: usize) -> Self {
        if t > Max as usize {
            panic!("Tried to convert usize bigger than TypeTag::Max to TypeTag")
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

pub fn dyn_main() {
    let stack = tagstack!(const TypeTagEnum);

    let stack = stack.push_tag::<{ Tag as usize }>();
    let stack = stack.push_tag::<{ Tag2 as usize }>();

    assert!(stack.pop_tag().1.contains(Tag2));
    assert!(!stack.pop_tag().1.contains(Tag));
}

pub fn const_main() {
    const STACK: ConstTypeTagStack<'static, TypeTagEnum, { Revoked.into() }> =
        ConstTypeTagStack(&Revoked);

    const STACK_2: ConstTypeTagStack<TypeTagEnum, { Tag.into() }> =
        STACK.push_tag::<{ Tag as usize }>();

    const STACK_3: ConstTypeTagStack<TypeTagEnum, { Tag2.into() }> =
        STACK_2.push_tag::<{ Tag2 as usize }>();

    const IS_TRUE: bool = STACK_3.pop_tag().1.const_contains::<{ Tag2 as usize }>();
    assert!(IS_TRUE);
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as usize }>());
}

fn main() {
    dyn_main()
}
