
# Flagstack

Create 'Flagstacks', which are stacks of enums in a way that makes it easy for the compiler to figure out if it is constant.
This is useful for conditional code execution, where it is not possible to guarantee that the conditions are known at compile time.

# Assembly examples

The code used to generate the assembly shown bellow is found [here](./tests/src/main.rs).
It is compiled on the dev profile, so it is expected to be optimized further in release.

## Release mode
In release mode both the constant and variable tests will be compiled to just `ret`.
The dynamic example will however be compiled to:

```asm

tests::main:
  7b60:    push    rax                      
  7b61:    lea     rdi, [rip + 0x334a9]     
  7b68:    lea     rdx, [rip + 0x408b9]     
  7b6f:    mov     esi, 0x31                
  7b74:    call    qword ptr [rip + 0x43466]
  7b7a:    ud2                              

```

## Using constants

``` asm

tests::const_main:
pub fn const_main() {
  8210:    sub     rsp, 0x28                                                                              
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>());
  8214:    lea     rdi, [rsp + 0x10]                                                                      
  8219:    lea     rsi, [rip + 0x411f0]                                                                   
  8220:    call    flagstack::ConstFlagStack<T,_>::pop_tag                                                
  8225:    lea     rdi, [rsp + 0x10]                                                                      
  822a:    add     rdi, 0x10                                                                              
  822e:    call    <tests::ExampleFlags as flagstack::ConstFlaggable<tests::ExampleFlags>>::const_contains
  8233:    mov     byte ptr [rsp + 0xf], al                                                               
  8237:    mov     al, byte ptr [rsp + 0xf]                                                               
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>());
  823b:    xor     al, 0xff                                                                               
  823d:    xor     al, 0xff                                                                               
  823f:    test    al, 1                                                                                  
  8241:    jne     0x8248                                                                                 
}
  8243:    add     rsp, 0x28                                                                              
  8247:    ret                                                                                            
    assert!(!STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>());
  8248:    lea     rdi, [rip + 0x33ef2]                                                                   
  824f:    lea     rdx, [rip + 0x411ca]                                                                   
  8256:    lea     rax, [rip - 0x111d]                                                                    
  825d:    mov     esi, 0x4a                                                                              
  8262:    call    rax                                                                                    
  8264:    ud2                                                                                            

```

## Using variables

``` asm

tests::var_main:
pub fn var_main() {
  80f0:    sub      rsp, 0x78                                                                      
    let stack = flagstack!(const ExampleFlags);
  80f4:    mov      rcx, qword ptr [rip + 0x412dd]                                                 
  80fb:    mov      rax, qword ptr [rip + 0x412de]                                                 
  8102:    mov      qword ptr [rsp + 8], rcx                                                       
  8107:    mov      qword ptr [rsp + 0x10], rax                                                    
  810c:    lea      rdi, [rsp + 8]                                                                 
    let stack = stack.push_tag::<{ Flag1 as u8 }>();
  8111:    call     flagstack::ConstFlagStack<T,_>::push_tag                                       
  8116:    mov      qword ptr [rsp + 0x20], rdx                                                    
  811b:    mov      qword ptr [rsp + 0x18], rax                                                    
  8120:    lea      rdi, [rsp + 0x18]                                                              
    let stack = stack.push_tag::<{ Flag2 as u8 }>();
  8125:    call     flagstack::ConstFlagStack<T,_>::push_tag                                       
  812a:    mov      qword ptr [rsp + 0x30], rdx                                                    
  812f:    mov      qword ptr [rsp + 0x28], rax                                                    
    assert!(stack.pop_tag().1.contains(Flag2));
  8134:    lea      rdi, [rsp + 0x38]                                                              
  8139:    lea      rsi, [rsp + 0x28]                                                              
  813e:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                        
  8143:    lea      rdi, [rsp + 0x38]                                                              
  8148:    add      rdi, 0x10                                                                      
  814c:    mov      byte ptr [rsp + 0x57], 1                                                       
  8151:    movzx    esi, byte ptr [rsp + 0x57]                                                     
  8156:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  815b:    mov      byte ptr [rsp + 7], al                                                         
  815f:    mov      al, byte ptr [rsp + 7]                                                         
    assert!(stack.pop_tag().1.contains(Flag2));
  8163:    xor      al, 0xff                                                                       
  8165:    test     al, 1                                                                          
  8167:    jne      0x817a                                                                         
    assert!(!stack.pop_tag().1.contains(Flag1));
  8169:    lea      rdi, [rsp + 0x58]                                                              
  816e:    lea      rsi, [rsp + 0x28]                                                              
  8173:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                        
  8178:    jmp      0x8198                                                                         
    assert!(stack.pop_tag().1.contains(Flag2));
  817a:    lea      rdi, [rip + 0x33f68]                                                           
  8181:    lea      rdx, [rip + 0x41260]                                                           
  8188:    lea      rax, [rip - 0x104f]                                                            
  818f:    mov      esi, 0x33                                                                      
  8194:    call     rax                                                                            
  8196:    ud2                                                                                     
    assert!(!stack.pop_tag().1.contains(Flag1));
  8198:    lea      rdi, [rsp + 0x58]                                                              
  819d:    add      rdi, 0x10                                                                      
  81a1:    mov      byte ptr [rsp + 0x77], 0                                                       
  81a6:    movzx    esi, byte ptr [rsp + 0x77]                                                     
  81ab:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  81b0:    mov      byte ptr [rsp + 6], al                                                         
  81b4:    mov      al, byte ptr [rsp + 6]                                                         
    assert!(!stack.pop_tag().1.contains(Flag1));
  81b8:    xor      al, 0xff                                                                       
  81ba:    xor      al, 0xff                                                                       
  81bc:    test     al, 1                                                                          
  81be:    jne      0x81c5                                                                         
}
  81c0:    add      rsp, 0x78                                                                      
  81c4:    ret                                                                                     
    assert!(!stack.pop_tag().1.contains(Flag1));
  81c5:    lea      rdi, [rip + 0x33f50]                                                           
  81cc:    lea      rdx, [rip + 0x4122d]                                                           
  81d3:    lea      rax, [rip - 0x109a]                                                            
  81da:    mov      esi, 0x34                                                                      
  81df:    call     rax                                                                            
  81e1:    ud2                                                                                     

```

## Using a dynamic typestack

``` asm

tests::dyn_main:
pub fn dyn_main() {
  7be0:    sub      rsp, 0x68                                                                      
    let mut stack = flagstack!(ExampleFlags);
  7be4:    lea      rax, [rip + 0x34436]                                                           
  7beb:    mov      qword ptr [rsp + 0x10], rax                                                    
  7bf0:    lea      rax, [rip + 0x41709]                                                           
  7bf7:    mov      qword ptr [rsp + 0x18], rax                                                    
  7bfc:    mov      byte ptr [rsp + 0x20], 4                                                       
    stack.push_tag(Flag1);
  7c01:    mov      byte ptr [rsp + 0x47], 0                                                       
  7c06:    lea      rdi, [rsp + 0x28]                                                              
  7c0b:    lea      rsi, [rsp + 0x10]                                                              
  7c10:    movzx    edx, byte ptr [rsp + 0x47]                                                     
  7c15:    call     flagstack::DynFlagStack<T>::push_tag                                           
    stack.push_tag(Flag2);
  7c1a:    mov      byte ptr [rsp + 0x63], 1                                                       
  7c1f:    lea      rdi, [rsp + 0x48]                                                              
  7c24:    lea      rsi, [rsp + 0x10]                                                              
  7c29:    movzx    edx, byte ptr [rsp + 0x63]                                                     
  7c2e:    call     flagstack::DynFlagStack<T>::push_tag                                           
    assert!(stack.pop_tag().contains(Flag2));
  7c33:    lea      rdi, [rsp + 0x10]                                                              
  7c38:    call     flagstack::DynFlagStack<T>::pop_tag                                            
  7c3d:    mov      byte ptr [rsp + 0x64], al                                                      
  7c41:    mov      byte ptr [rsp + 0x65], 1                                                       
  7c46:    lea      rdi, [rsp + 0x64]                                                              
  7c4b:    movzx    esi, byte ptr [rsp + 0x65]                                                     
  7c50:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  7c55:    mov      byte ptr [rsp + 0xf], al                                                       
  7c59:    mov      al, byte ptr [rsp + 0xf]                                                       
    assert!(stack.pop_tag().contains(Flag2));
  7c5d:    xor      al, 0xff                                                                       
  7c5f:    test     al, 1                                                                          
  7c61:    jne      0x7c73                                                                         
    assert!(!stack.pop_tag().contains(Flag1));
  7c63:    lea      rdi, [rsp + 0x10]                                                              
  7c68:    call     flagstack::DynFlagStack<T>::pop_tag                                            
  7c6d:    mov      byte ptr [rsp + 0x66], al                                                      
  7c71:    jmp      0x7c91                                                                         
    assert!(stack.pop_tag().contains(Flag2));
  7c73:    lea      rdi, [rip + 0x343a8]                                                           
  7c7a:    lea      rdx, [rip + 0x416af]                                                           
  7c81:    lea      rax, [rip - 0xb38]                                                             
  7c88:    mov      esi, 0x31                                                                      
  7c8d:    call     rax                                                                            
  7c8f:    ud2                                                                                     
    assert!(!stack.pop_tag().contains(Flag1));
  7c91:    mov      byte ptr [rsp + 0x67], 0                                                       
  7c96:    lea      rdi, [rsp + 0x66]                                                              
  7c9b:    movzx    esi, byte ptr [rsp + 0x67]                                                     
  7ca0:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  7ca5:    mov      byte ptr [rsp + 0xe], al                                                       
  7ca9:    mov      al, byte ptr [rsp + 0xe]                                                       
    assert!(!stack.pop_tag().contains(Flag1));
  7cad:    xor      al, 0xff                                                                       
  7caf:    xor      al, 0xff                                                                       
  7cb1:    test     al, 1                                                                          
  7cb3:    jne      0x7cba                                                                         
}
  7cb5:    add      rsp, 0x68                                                                      
  7cb9:    ret                                                                                     
    assert!(!stack.pop_tag().contains(Flag1));
  7cba:    lea      rdi, [rip + 0x34392]                                                           
  7cc1:    lea      rdx, [rip + 0x41680]                                                           
  7cc8:    lea      rax, [rip - 0xb7f]                                                             
  7ccf:    mov      esi, 0x32                                                                      
  7cd4:    call     rax                                                                            
  7cd6:    ud2                                                                                     

```

