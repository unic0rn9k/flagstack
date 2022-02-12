# tagstack

The code used to generate the assembly shown bellow is found [here](./tests/src/main.rs).
It is compiled on the dev profile, so it is expected to be optimized further in release.

## Using constants
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

## Using variables
``` asm
tests::dyn_main:
pub fn dyn_main() {
  80a0:    sub      rsp, 0x78                                                                  
    let stack = tagstack!(const TypeTagEnum);
  80a4:    lea      rax, [rip + 0x34085]                                                       
  80ab:    mov      qword ptr [rsp + 8], rax                                                   
  80b0:    lea      rax, [rip + 0x412d1]                                                       
  80b7:    mov      qword ptr [rsp + 0x10], rax                                                
  80bc:    lea      rdi, [rsp + 8]                                                             
    let stack = stack.push_tag::<{ Tag as usize }>();
  80c1:    call     tagstack::ConstTypeTagStack<T,_>::push_tag                                 
  80c6:    mov      qword ptr [rsp + 0x20], rdx                                                
  80cb:    mov      qword ptr [rsp + 0x18], rax                                                
  80d0:    lea      rdi, [rsp + 0x18]                                                          
    let stack = stack.push_tag::<{ Tag2 as usize }>();
  80d5:    call     tagstack::ConstTypeTagStack<T,_>::push_tag                                 
  80da:    mov      qword ptr [rsp + 0x30], rdx                                                
  80df:    mov      qword ptr [rsp + 0x28], rax                                                
    assert!(stack.pop_tag().1.contains(Tag2));
  80e4:    lea      rdi, [rsp + 0x38]                                                          
  80e9:    lea      rsi, [rsp + 0x28]                                                          
  80ee:    call     tagstack::ConstTypeTagStack<T,_>::pop_tag                                  
  80f3:    lea      rdi, [rsp + 0x38]                                                          
  80f8:    add      rdi, 0x10                                                                  
  80fc:    mov      byte ptr [rsp + 0x57], 1                                                   
  8101:    movzx    esi, byte ptr [rsp + 0x57]                                                 
  8106:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  810b:    mov      byte ptr [rsp + 7], al                                                     
  810f:    mov      al, byte ptr [rsp + 7]                                                     
    assert!(stack.pop_tag().1.contains(Tag2));
  8113:    xor      al, 0xff                                                                   
  8115:    test     al, 1                                                                      
  8117:    jne      0x812a                                                                     
    assert!(!stack.pop_tag().1.contains(Tag));
  8119:    lea      rdi, [rsp + 0x58]                                                          
  811e:    lea      rsi, [rsp + 0x28]                                                          
  8123:    call     tagstack::ConstTypeTagStack<T,_>::pop_tag                                  
  8128:    jmp      0x8148                                                                     
    assert!(stack.pop_tag().1.contains(Tag2));
  812a:    lea      rdi, [rip + 0x34000]                                                       
  8131:    lea      rdx, [rip + 0x41280]                                                       
  8138:    lea      rax, [rip - 0xfff]                                                         
  813f:    mov      esi, 0x32                                                                  
  8144:    call     rax                                                                        
  8146:    ud2                                                                                 
    assert!(!stack.pop_tag().1.contains(Tag));
  8148:    lea      rdi, [rsp + 0x58]                                                          
  814d:    add      rdi, 0x10                                                                  
  8151:    mov      byte ptr [rsp + 0x77], 0                                                   
  8156:    movzx    esi, byte ptr [rsp + 0x77]                                                 
  815b:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  8160:    mov      byte ptr [rsp + 6], al                                                     
  8164:    mov      al, byte ptr [rsp + 6]                                                     
    assert!(!stack.pop_tag().1.contains(Tag));
  8168:    xor      al, 0xff                                                                   
  816a:    xor      al, 0xff                                                                   
  816c:    test     al, 1                                                                      
  816e:    jne      0x8175                                                                     
}
  8170:    add      rsp, 0x78                                                                  
  8174:    ret                                                                                 
    assert!(!stack.pop_tag().1.contains(Tag));
  8175:    lea      rdi, [rip + 0x33fe7]                                                       
  817c:    lea      rdx, [rip + 0x4124d]                                                       
  8183:    lea      rax, [rip - 0x104a]                                                        
  818a:    mov      esi, 0x32                                                                  
  818f:    call     rax                                                                        
  8191:    ud2                                                                                 
```
