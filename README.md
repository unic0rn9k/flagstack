# tagstack

``` asm 
tests::const_main:
pub fn const_main() {
  8040:    sub     rsp, 0x28
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as usize }>());
  8044:    lea     rdi, [rsp + 0x10]
  8049:    lea     rsi, [rip + 0x41368]
  8050:    call    tagstack::ConstTypeTagStack<T,_>::pop_tag
  8055:    lea     rdi, [rsp + 0x10]
  805a:    add     rdi, 0x10
  805e:    call    <tests::TypeTagEnum as tagstack::ConstTaggable<tests::TypeTagEnum>>::const_contains
  8063:    mov     byte ptr [rsp + 0xf], al
  8067:    mov     al, byte ptr [rsp + 0xf]
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as usize }>());
  806b:    xor     al, 0xff
  806d:    xor     al, 0xff
  806f:    test    al, 1
  8071:    jne     0x8078
}
  8073:    add     rsp, 0x28
  8077:    ret
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as usize }>());
  8078:    lea     rdi, [rip + 0x340b2]
  807f:    lea     rdx, [rip + 0x41342]
  8086:    lea     rax, [rip - 0xf4d]
  808d:    mov     esi, 0x4b
  8092:    call    rax
  8094:    ud2
```
