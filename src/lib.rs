#![feature(const_trait_impl, const_fn_trait_bound)]

use std::{intrinsics::transmute, mem::transmute_copy};

pub trait TypeTag: Copy + PartialEq + From<u8> + Into<u8> {
    const REVOKED: Self;
    const MAX: u8;
    const IS_STACK: Self;
    const IS_CONST: Self;
    fn from_u8(t: u8) -> Self;
}

pub trait DynTaggable<T: TypeTag> {
    fn contains(&self, tag: T) -> bool;
    fn tag(&mut self, tag: T);
    fn untag(&mut self, tag: T);
}

pub trait ConstTaggable<T: TypeTag>: DynTaggable<T> {
    fn const_contains<const TAG: u8>(&self) -> bool
    where
        Self: ~const ConstTaggable<T>,
        Self: Sized;
}

impl<T: TypeTag> dyn DynTaggable<T> {
    pub fn as_const(&self) -> &dyn ConstTaggable<T> {
        if self.contains(T::IS_CONST) {
            unsafe { transmute(self) }
        } else {
            panic!("Failed to cast DynTaggable to ConstTaggable")
        }
    }
}

impl<T: TypeTag> dyn ConstTaggable<T> {
    pub fn as_dyn(&self) -> &dyn DynTaggable<T> {
        if !self.contains(T::IS_CONST) {
            unsafe { transmute(self) }
        } else {
            panic!("Failed to cast DynTaggable to ConstTaggable")
        }
    }
}

#[derive(Clone, Copy)]
pub struct ConstTypeTagStack<'a, T: TypeTag, const TAG: u8>(pub &'a dyn ConstTaggable<T>);

#[derive(Clone, Copy)]
pub struct DynTypeTagStack<'a, T: TypeTag> {
    pub child: &'a dyn DynTaggable<T>,
    pub tag: T,
}

impl<'a, T: TypeTag, const TAG: u8> DynTaggable<T> for ConstTypeTagStack<'a, T, TAG> {
    fn contains(&self, tag: T) -> bool {
        TAG == T::IS_STACK.into() || tag.into() == TAG || self.0.contains(tag) || tag == T::IS_CONST
    }

    fn tag(&mut self, _tag: T) {
        panic!("Please use LinkedTypeTagList::push_tag instead")
    }
    fn untag(&mut self, _tag: T) {
        panic!("Please use LinkedTypeTagList::pop_tag instead")
    }
}

impl<'a, T: TypeTag> DynTaggable<T> for DynTypeTagStack<'a, T> {
    fn contains(&self, tag: T) -> bool {
        tag != T::IS_CONST
            && (self.tag == T::IS_STACK || tag == self.tag || self.child.contains(tag))
    }

    fn tag(&mut self, _tag: T) {
        panic!("Please use DynTypeTagStack::push_tag instead")
    }

    fn untag(&mut self, tag: T) {
        if self.child.contains(tag) {
            unsafe { *self = transmute_copy(&self.child) }
        } else {
            panic!("Tried to revoke tag, not at top of stack")
        }
    }
}

impl<'a, T: TypeTag> DynTypeTagStack<'a, T> {
    pub const fn push_tag(&'a self, tag: T) -> DynTypeTagStack<'a, T> {
        Self { child: self, tag }
    }
    pub fn pop_tag(&mut self) -> T {
        let tag = self.tag;
        self.untag(tag);
        tag
    }
}

impl<'a, T: TypeTag, const TAG: u8> ConstTaggable<T> for ConstTypeTagStack<'a, T, TAG> {
    fn const_contains<const TAG2: u8>(&self) -> bool {
        TAG == T::IS_STACK.into()
            || TAG2 == TAG
            || self.0.contains(TAG2.into())
            || TAG2 == T::IS_CONST.into()
    }
}

impl<'a, T: TypeTag, const TAG: u8> ConstTypeTagStack<'a, T, TAG> {
    pub const fn push_tag<const TAG2: u8>(&'a self) -> ConstTypeTagStack<'a, T, TAG2> {
        ConstTypeTagStack(self)
    }
    pub const fn pop_tag(&'a self) -> (&'a dyn ConstTaggable<T>, T)
    where
        T: ~const TypeTag,
    {
        (self.0, T::from_u8(TAG))
    }
}

impl<'a, T: TypeTag, const TAG: u8> Iterator for ConstTypeTagStack<'a, T, TAG> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let (child, tag) = self.pop_tag();
        if child.contains(T::REVOKED) {
            None
        } else {
            if child.contains(T::IS_STACK) {
                unsafe { *self = std::mem::transmute_copy(&child) }
            }
            Some(tag)
        }
    }
}
impl<'a, T: TypeTag> Iterator for DynTypeTagStack<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let tag = self.pop_tag();
        if tag == T::REVOKED {
            None
        } else {
            Some(tag)
        }
    }
}

#[macro_export]
macro_rules! tagstack {
    (const $TypeTag: ty) => {{
        const TMP: ConstTypeTagStack<'static, $TypeTag, { <$TypeTag>::REVOKED as u8 }> =
            ConstTypeTagStack::<'static, $TypeTag, { <$TypeTag>::REVOKED as u8 }>(
                &<$TypeTag>::REVOKED,
            );
        TMP
    }};
    ($TypeTag: ty) => {{
        DynTypeTagStack::<'static, $TypeTag> {
            child: &<$TypeTag>::REVOKED,
            tag: <$TypeTag>::REVOKED,
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn dynamic() {
        let stack = tagstack!(const TypeTagEnum);
        let stack = stack.push_tag::<{ Tag as u8 }>();
        let stack = stack.push_tag::<{ Tag2 as u8 }>();

        assert!(stack.pop_tag().1.contains(Tag2));
        assert!(!stack.pop_tag().1.contains(Tag));
        for tag in stack {
            assert!(tag.contains(Tag) || tag.contains(Tag2))
        }
    }

    #[test]
    #[should_panic]
    fn cast_dyn_to_const() {
        let stack: &dyn DynTaggable<TypeTagEnum> = &tagstack!(TypeTagEnum);
        stack.as_const();
    }

    #[test]
    fn cast_const_to_const() {
        let stack: &dyn DynTaggable<TypeTagEnum> = &tagstack!(const TypeTagEnum);
        stack.as_const();
    }

    #[test]
    fn constant() {
        const STACK: ConstTypeTagStack<'static, TypeTagEnum, { Revoked.into() }> =
            ConstTypeTagStack(&Revoked);

        const STACK_2: ConstTypeTagStack<TypeTagEnum, { Tag.into() }> =
            STACK.push_tag::<{ Tag as u8 }>();

        const STACK_3: ConstTypeTagStack<TypeTagEnum, { Tag2.into() }> =
            STACK_2.push_tag::<{ Tag2 as u8 }>();

        const IS_TRUE: bool = STACK_3.pop_tag().1.const_contains::<{ Tag2 as u8 }>();
        assert!(IS_TRUE);
        assert!(STACK_2.pop_tag().1.const_contains::<{ Tag as u8 }>());
        assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as u8 }>());

        for tag in STACK_3 {
            assert!(tag.contains(Tag) || tag.contains(Tag2))
        }
    }
}
