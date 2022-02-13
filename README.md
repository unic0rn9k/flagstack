
# tagstack

Create 'tagstacks', which are stacks of enums in a way that makes it easy for the compiler to figure out if it is constant.
This is useful for conditional code execution, where it is not possible to guarantee that the conditions are known at compile time.

# Assembly examples

The code used to generate the assembly shown bellow is found [here](./tests/src/main.rs).
It is compiled on the dev profile, so it is expected to be optimized further in release.

## Using constants

``` asm

tests::const_main:
pub fn const_main() {
  7dd0:    sub     rsp, 0x28                                                                          
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as u8 }>());
  7dd4:    lea     rdi, [rsp + 0x10]                                                                  
  7dd9:    lea     rsi, [rip + 0x415b0]                                                               
  7de0:    call    tagstack::ConstTypeTagStack<T,_>::pop_tag                                          
  7de5:    lea     rdi, [rsp + 0x10]                                                                  
  7dea:    add     rdi, 0x10                                                                          
  7dee:    call    <tests::TypeTagEnum as tagstack::ConstTaggable<tests::TypeTagEnum>>::const_contains
  7df3:    mov     byte ptr [rsp + 0xf], al                                                           
  7df7:    mov     al, byte ptr [rsp + 0xf]                                                           
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as u8 }>());
  7dfb:    xor     al, 0xff                                                                           
  7dfd:    xor     al, 0xff                                                                           
  7dff:    test    al, 1                                                                              
  7e01:    jne     0x7e08                                                                             
}
  7e03:    add     rsp, 0x28                                                                          
  7e07:    ret                                                                                        
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Tag as u8 }>());
  7e08:    lea     rdi, [rip + 0x342f2]                                                               
  7e0f:    lea     rdx, [rip + 0x4158a]                                                               
  7e16:    lea     rax, [rip - 0xcdd]                                                                 
  7e1d:    mov     esi, 0x48                                                                          
  7e22:    call    rax                                                                                
  7e24:    ud2                                                                                        

```

## Using variables

``` asm

tests::var_main:
pub fn var_main() {
  82b0:    sub      rsp, 0x78                                                                  
    let stack = tagstack!(const TypeTagEnum);
  82b4:    mov      rcx, qword ptr [rip + 0x41115]                                             
  82bb:    mov      rax, qword ptr [rip + 0x41116]                                             
  82c2:    mov      qword ptr [rsp + 8], rcx                                                   
  82c7:    mov      qword ptr [rsp + 0x10], rax                                                
  82cc:    lea      rdi, [rsp + 8]                                                             
    let stack = stack.push_tag::<{ Tag as u8 }>();
  82d1:    call     tagstack::ConstTypeTagStack<T,_>::push_tag                                 
  82d6:    mov      qword ptr [rsp + 0x20], rdx                                                
  82db:    mov      qword ptr [rsp + 0x18], rax                                                
  82e0:    lea      rdi, [rsp + 0x18]                                                          
    let stack = stack.push_tag::<{ Tag2 as u8 }>();
  82e5:    call     tagstack::ConstTypeTagStack<T,_>::push_tag                                 
  82ea:    mov      qword ptr [rsp + 0x30], rdx                                                
  82ef:    mov      qword ptr [rsp + 0x28], rax                                                
    assert!(stack.pop_tag().1.contains(Tag2));
  82f4:    lea      rdi, [rsp + 0x38]                                                          
  82f9:    lea      rsi, [rsp + 0x28]                                                          
  82fe:    call     tagstack::ConstTypeTagStack<T,_>::pop_tag                                  
  8303:    lea      rdi, [rsp + 0x38]                                                          
  8308:    add      rdi, 0x10                                                                  
  830c:    mov      byte ptr [rsp + 0x57], 1                                                   
  8311:    movzx    esi, byte ptr [rsp + 0x57]                                                 
  8316:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  831b:    mov      byte ptr [rsp + 7], al                                                     
  831f:    mov      al, byte ptr [rsp + 7]                                                     
    assert!(stack.pop_tag().1.contains(Tag2));
  8323:    xor      al, 0xff                                                                   
  8325:    test     al, 1                                                                      
  8327:    jne      0x833a                                                                     
    assert!(!stack.pop_tag().1.contains(Tag));
  8329:    lea      rdi, [rsp + 0x58]                                                          
  832e:    lea      rsi, [rsp + 0x28]                                                          
  8333:    call     tagstack::ConstTypeTagStack<T,_>::pop_tag                                  
  8338:    jmp      0x8358                                                                     
    assert!(stack.pop_tag().1.contains(Tag2));
  833a:    lea      rdi, [rip + 0x33e48]                                                       
  8341:    lea      rdx, [rip + 0x41098]                                                       
  8348:    lea      rax, [rip - 0x120f]                                                        
  834f:    mov      esi, 0x32                                                                  
  8354:    call     rax                                                                        
  8356:    ud2                                                                                 
    assert!(!stack.pop_tag().1.contains(Tag));
  8358:    lea      rdi, [rsp + 0x58]                                                          
  835d:    add      rdi, 0x10                                                                  
  8361:    mov      byte ptr [rsp + 0x77], 0                                                   
  8366:    movzx    esi, byte ptr [rsp + 0x77]                                                 
  836b:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  8370:    mov      byte ptr [rsp + 6], al                                                     
  8374:    mov      al, byte ptr [rsp + 6]                                                     
    assert!(!stack.pop_tag().1.contains(Tag));
  8378:    xor      al, 0xff                                                                   
  837a:    xor      al, 0xff                                                                   
  837c:    test     al, 1                                                                      
  837e:    jne      0x8385                                                                     
}
  8380:    add      rsp, 0x78                                                                  
  8384:    ret                                                                                 
    assert!(!stack.pop_tag().1.contains(Tag));
  8385:    lea      rdi, [rip + 0x33e2f]                                                       
  838c:    lea      rdx, [rip + 0x41065]                                                       
  8393:    lea      rax, [rip - 0x125a]                                                        
  839a:    mov      esi, 0x32                                                                  
  839f:    call     rax                                                                        
  83a1:    ud2                                                                                 

```

## Using a dynamic typestack

``` asm

tests::dyn_main:
pub fn dyn_main() {
  7ff0:    sub      rsp, 0x68                                                                  
    let mut stack = tagstack!(TypeTagEnum);
  7ff4:    lea      rax, [rip + 0x34149]                                                       
  7ffb:    mov      qword ptr [rsp + 0x10], rax                                                
  8000:    lea      rax, [rip + 0x413a1]                                                       
  8007:    mov      qword ptr [rsp + 0x18], rax                                                
  800c:    mov      byte ptr [rsp + 0x20], 4                                                   
    stack.push_tag(Tag);
  8011:    mov      byte ptr [rsp + 0x47], 0                                                   
  8016:    lea      rdi, [rsp + 0x28]                                                          
  801b:    lea      rsi, [rsp + 0x10]                                                          
  8020:    movzx    edx, byte ptr [rsp + 0x47]                                                 
  8025:    call     tagstack::DynTypeTagStack<T>::push_tag                                     
    stack.push_tag(Tag2);
  802a:    mov      byte ptr [rsp + 0x63], 1                                                   
  802f:    lea      rdi, [rsp + 0x48]                                                          
  8034:    lea      rsi, [rsp + 0x10]                                                          
  8039:    movzx    edx, byte ptr [rsp + 0x63]                                                 
  803e:    call     tagstack::DynTypeTagStack<T>::push_tag                                     
    assert!(stack.pop_tag().contains(Tag2));
  8043:    lea      rdi, [rsp + 0x10]                                                          
  8048:    call     tagstack::DynTypeTagStack<T>::pop_tag                                      
  804d:    mov      byte ptr [rsp + 0x64], al                                                  
  8051:    mov      byte ptr [rsp + 0x65], 1                                                   
  8056:    lea      rdi, [rsp + 0x64]                                                          
  805b:    movzx    esi, byte ptr [rsp + 0x65]                                                 
  8060:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  8065:    mov      byte ptr [rsp + 0xf], al                                                   
  8069:    mov      al, byte ptr [rsp + 0xf]                                                   
    assert!(stack.pop_tag().contains(Tag2));
  806d:    xor      al, 0xff                                                                   
  806f:    test     al, 1                                                                      
  8071:    jne      0x8083                                                                     
    assert!(!stack.pop_tag().contains(Tag));
  8073:    lea      rdi, [rsp + 0x10]                                                          
  8078:    call     tagstack::DynTypeTagStack<T>::pop_tag                                      
  807d:    mov      byte ptr [rsp + 0x66], al                                                  
  8081:    jmp      0x80a1                                                                     
    assert!(stack.pop_tag().contains(Tag2));
  8083:    lea      rdi, [rip + 0x340bb]                                                       
  808a:    lea      rdx, [rip + 0x41347]                                                       
  8091:    lea      rax, [rip - 0xf48]                                                         
  8098:    mov      esi, 0x30                                                                  
  809d:    call     rax                                                                        
  809f:    ud2                                                                                 
    assert!(!stack.pop_tag().contains(Tag));
  80a1:    mov      byte ptr [rsp + 0x67], 0                                                   
  80a6:    lea      rdi, [rsp + 0x66]                                                          
  80ab:    movzx    esi, byte ptr [rsp + 0x67]                                                 
  80b0:    call     <tests::TypeTagEnum as tagstack::DynTaggable<tests::TypeTagEnum>>::contains
  80b5:    mov      byte ptr [rsp + 0xe], al                                                   
  80b9:    mov      al, byte ptr [rsp + 0xe]                                                   
    assert!(!stack.pop_tag().contains(Tag));
  80bd:    xor      al, 0xff                                                                   
  80bf:    xor      al, 0xff                                                                   
  80c1:    test     al, 1                                                                      
  80c3:    jne      0x80ca                                                                     
}
  80c5:    add      rsp, 0x68                                                                  
  80c9:    ret                                                                                 
    assert!(!stack.pop_tag().contains(Tag));
  80ca:    lea      rdi, [rip + 0x340a4]                                                       
  80d1:    lea      rdx, [rip + 0x41318]                                                       
  80d8:    lea      rax, [rip - 0xf8f]                                                         
  80df:    mov      esi, 0x30                                                                  
  80e4:    call     rax                                                                        
  80e6:    ud2                                                                                 

```

