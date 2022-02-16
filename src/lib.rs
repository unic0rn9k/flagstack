#![feature(const_trait_impl, const_fn_trait_bound)]

use std::{intrinsics::transmute, mem::transmute_copy};

pub trait Flag: Copy + PartialEq + From<u8> + Into<u8> {
    const REVOKED: Self;
    const MAX: u8;
    const IS_STACK: Self;
    const IS_CONST: Self;
    fn from_u8(t: u8) -> Self;
}

pub trait DynFlaggable<T: Flag> {
    fn contains(&self, tag: T) -> bool;
    fn tag(&mut self, tag: T);
    fn untag(&mut self, tag: T);
}

pub trait ConstFlaggable<T: Flag>: DynFlaggable<T> {
    fn const_contains<const TAG: u8>(&self) -> bool
    where
        Self: ~const ConstFlaggable<T>,
        Self: Sized;
}

impl<T: Flag> dyn DynFlaggable<T> {
    pub fn as_const(&self) -> &dyn ConstFlaggable<T> {
        if self.contains(T::IS_CONST) {
            unsafe { transmute(self) }
        } else {
            panic!("Failed to cast DynTaggable to ConstTaggable")
        }
    }
}

impl<T: Flag> dyn ConstFlaggable<T> {
    pub fn as_dyn(&self) -> &dyn DynFlaggable<T> {
        if !self.contains(T::IS_CONST) {
            unsafe { transmute(self) }
        } else {
            panic!("Failed to cast DynTaggable to ConstTaggable")
        }
    }
}

#[derive(Clone, Copy)]
pub struct ConstFlagStack<'a, T: Flag, const TAG: u8>(pub &'a dyn ConstFlaggable<T>);

#[derive(Clone, Copy)]
pub struct DynFlagStack<'a, T: Flag> {
    pub child: &'a dyn DynFlaggable<T>,
    pub tag: T,
}

impl<'a, T: Flag, const TAG: u8> DynFlaggable<T> for ConstFlagStack<'a, T, TAG> {
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

impl<'a, T: Flag> DynFlaggable<T> for DynFlagStack<'a, T> {
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

impl<'a, T: Flag> DynFlagStack<'a, T> {
    pub const fn push_tag(&'a self, tag: T) -> DynFlagStack<'a, T> {
        Self { child: self, tag }
    }
    pub fn pop_tag(&mut self) -> T {
        let tag = self.tag;
        self.untag(tag);
        tag
    }
}

impl<'a, T: Flag, const TAG: u8> ConstFlaggable<T> for ConstFlagStack<'a, T, TAG> {
    fn const_contains<const TAG2: u8>(&self) -> bool {
        TAG == T::IS_STACK.into()
            || TAG2 == TAG
            || self.0.contains(TAG2.into())
            || TAG2 == T::IS_CONST.into()
    }
}

impl<'a, T: Flag, const TAG: u8> ConstFlagStack<'a, T, TAG> {
    pub const fn push_tag<const TAG2: u8>(&'a self) -> ConstFlagStack<'a, T, TAG2> {
        ConstFlagStack(self)
    }
    pub const fn pop_tag(&'a self) -> (&'a dyn ConstFlaggable<T>, T)
    where
        T: ~const Flag,
    {
        (self.0, T::from_u8(TAG))
    }
}

impl<'a, T: Flag, const TAG: u8> Iterator for ConstFlagStack<'a, T, TAG> {
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
impl<'a, T: Flag> Iterator for DynFlagStack<'a, T> {
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
macro_rules! flagstack {
    (const $TypeTag: ty) => {{
        const TMP: ConstFlagStack<'static, $TypeTag, { <$TypeTag>::REVOKED as u8 }> =
            ConstFlagStack::<'static, $TypeTag, { <$TypeTag>::REVOKED as u8 }>(
                &<$TypeTag>::REVOKED,
            );
        TMP
    }};
    ($TypeTag: ty) => {{
        DynFlagStack::<'static, $TypeTag> {
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

    impl const ConstFlaggable<Self> for TypeTagEnum {
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

    impl const Flag for TypeTagEnum {
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

    impl DynFlaggable<Self> for TypeTagEnum {
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
        let stack = flagstack!(const TypeTagEnum);
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
        let stack: &dyn DynFlaggable<TypeTagEnum> = &flagstack!(TypeTagEnum);
        stack.as_const();
    }

    #[test]
    fn cast_const_to_const() {
        let stack: &dyn DynFlaggable<TypeTagEnum> = &flagstack!(const TypeTagEnum);
        stack.as_const();
    }

    #[test]
    fn constant() {
        const STACK: ConstFlagStack<'static, TypeTagEnum, { Revoked.into() }> =
            ConstFlagStack(&Revoked);

        const STACK_2: ConstFlagStack<TypeTagEnum, { Tag.into() }> =
            STACK.push_tag::<{ Tag as u8 }>();

        const STACK_3: ConstFlagStack<TypeTagEnum, { Tag2.into() }> =
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
