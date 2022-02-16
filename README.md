
# Flagstack

Create 'Flagstacks', which are stacks of enums in a way that makes it easy for the compiler to figure out if it is constant.
This is useful for conditional code execution, where it is not possible to guarantee that the conditions are known at compile time.

# Assembly examples

The code used to generate the assembly shown bellow is found [here](./tests/src/main.rs).
It is compiled on the dev profile, so it is expected to be optimized further in release.

## Release mode
In release mode both the constant and variable tests will be compiled to just `ret` unles `black_box` are used'.


### Using constants

``` asm

tests::main:
  7b20:    push    rax                      
  7b21:    mov     byte ptr [rsp + 6], 1    
  7b26:    lea     rax, [rsp + 6]           
  7b2b:    cmp     byte ptr [rsp + 6], 0    
  7b30:    je      0x7b45                   
  7b32:    mov     byte ptr [rsp + 7], 1    
  7b37:    lea     rax, [rsp + 7]           
  7b3c:    cmp     byte ptr [rsp + 7], 0    
  7b41:    je      0x7b60                   
  7b43:    pop     rax                      
  7b44:    ret                              
  7b45:    lea     rdi, [rip + 0x334c5]     
  7b4c:    lea     rdx, [rip + 0x40885]     
  7b53:    mov     esi, 0x54                
  7b58:    call    qword ptr [rip + 0x4347a]
  7b5e:    ud2                              
  7b60:    lea     rdi, [rip + 0x334fe]     
  7b67:    lea     rdx, [rip + 0x40882]     
  7b6e:    mov     esi, 0x55                
  7b73:    call    qword ptr [rip + 0x4345f]
  7b79:    ud2                              

```

### Using variables

``` asm

tests::main:
  7b20:    push    rax                      
  7b21:    mov     byte ptr [rsp + 6], 1    
  7b26:    lea     rax, [rsp + 6]           
  7b2b:    cmp     byte ptr [rsp + 6], 0    
  7b30:    je      0x7b45                   
  7b32:    mov     byte ptr [rsp + 7], 1    
  7b37:    lea     rax, [rsp + 7]           
  7b3c:    cmp     byte ptr [rsp + 7], 0    
  7b41:    je      0x7b60                   
  7b43:    pop     rax                      
  7b44:    ret                              
  7b45:    lea     rdi, [rip + 0x334c5]     
  7b4c:    lea     rdx, [rip + 0x40885]     
  7b53:    mov     esi, 0x3e                
  7b58:    call    qword ptr [rip + 0x4347a]
  7b5e:    ud2                              
  7b60:    lea     rdi, [rip + 0x334e8]     
  7b67:    lea     rdx, [rip + 0x40882]     
  7b6e:    mov     esi, 0x3f                
  7b73:    call    qword ptr [rip + 0x4345f]
  7b79:    ud2                              

```

### Using a dynamic flagstack

``` asm

tests::main:
  7b60:    push    rax                      
  7b61:    mov     byte ptr [rsp + 6], 0    
  7b66:    lea     rax, [rsp + 6]           
  7b6b:    cmp     byte ptr [rsp + 6], 0    
  7b70:    je      0x7b85                   
  7b72:    mov     byte ptr [rsp + 7], 1    
  7b77:    lea     rax, [rsp + 7]           
  7b7c:    cmp     byte ptr [rsp + 7], 0    
  7b81:    je      0x7ba0                   
  7b83:    pop     rax                      
  7b84:    ret                              
  7b85:    lea     rdi, [rip + 0x33485]     
  7b8c:    lea     rdx, [rip + 0x40875]     
  7b93:    mov     esi, 0x3c                
  7b98:    call    qword ptr [rip + 0x4343a]
  7b9e:    ud2                              
  7ba0:    lea     rdi, [rip + 0x334a6]     
  7ba7:    lea     rdx, [rip + 0x40872]     
  7bae:    mov     esi, 0x3d                
  7bb3:    call    qword ptr [rip + 0x4341f]
  7bb9:    ud2                              

```


## Debug mode

```asm

tests::main:
  7b60:    push    rax                      
  7b61:    mov     byte ptr [rsp + 6], 0    
  7b66:    lea     rax, [rsp + 6]           
  7b6b:    cmp     byte ptr [rsp + 6], 0    
  7b70:    je      0x7b85                   
  7b72:    mov     byte ptr [rsp + 7], 1    
  7b77:    lea     rax, [rsp + 7]           
  7b7c:    cmp     byte ptr [rsp + 7], 0    
  7b81:    je      0x7ba0                   
  7b83:    pop     rax                      
  7b84:    ret                              
  7b85:    lea     rdi, [rip + 0x33485]     
  7b8c:    lea     rdx, [rip + 0x40875]     
  7b93:    mov     esi, 0x3c                
  7b98:    call    qword ptr [rip + 0x4343a]
  7b9e:    ud2                              
  7ba0:    lea     rdi, [rip + 0x334a6]     
  7ba7:    lea     rdx, [rip + 0x40872]     
  7bae:    mov     esi, 0x3d                
  7bb3:    call    qword ptr [rip + 0x4341f]
  7bb9:    ud2                              

```

### Using constants

``` asm

tests::const_main:
pub fn const_main() {
  8230:    sub      rsp, 0x38                                                                              
        STACK_3.pop_tag().1.const_contains::<{ Flag2 as u8 }>()
  8234:    lea      rdi, [rsp + 8]                                                                         
  8239:    lea      rsi, [rip + 0x411c0]                                                                   
  8240:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                                
  8245:    lea      rdi, [rsp + 8]                                                                         
  824a:    add      rdi, 0x10                                                                              
  824e:    call     <tests::ExampleFlags as flagstack::ConstFlaggable<tests::ExampleFlags>>::const_contains
  8253:    mov      byte ptr [rsp + 7], al                                                                 
  8257:    mov      al, byte ptr [rsp + 7]                                                                 
    assert!(black_box(
  825b:    movzx    edi, al                                                                                
  825e:    and      edi, 1                                                                                 
  8261:    call     core::hint::black_box                                                                  
  8266:    mov      byte ptr [rsp + 6], al                                                                 
  826a:    mov      al, byte ptr [rsp + 6]                                                                 
    assert!(black_box(
  826e:    xor      al, 0xff                                                                               
  8270:    test     al, 1                                                                                  
  8272:    jne      0x8287                                                                                 
        !STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>()
  8274:    lea      rdi, [rsp + 0x20]                                                                      
  8279:    lea      rsi, [rip + 0x41180]                                                                   
  8280:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                                
  8285:    jmp      0x82a5                                                                                 
    assert!(black_box(
  8287:    lea      rdi, [rip + 0x33eb3]                                                                   
  828e:    lea      rdx, [rip + 0x4117b]                                                                   
  8295:    lea      rax, [rip - 0x115c]                                                                    
  829c:    mov      esi, 0x54                                                                              
  82a1:    call     rax                                                                                    
  82a3:    ud2                                                                                             
        !STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>()
  82a5:    lea      rdi, [rsp + 0x20]                                                                      
  82aa:    add      rdi, 0x10                                                                              
  82ae:    call     <tests::ExampleFlags as flagstack::ConstFlaggable<tests::ExampleFlags>>::const_contains
  82b3:    mov      byte ptr [rsp + 5], al                                                                 
  82b7:    mov      al, byte ptr [rsp + 5]                                                                 
        !STACK_3.pop_tag().1.const_contains::<{ Flag1 as u8 }>()
  82bb:    xor      al, 0xff                                                                               
    assert!(black_box(
  82bd:    movzx    edi, al                                                                                
  82c0:    and      edi, 1                                                                                 
  82c3:    call     core::hint::black_box                                                                  
  82c8:    mov      byte ptr [rsp + 4], al                                                                 
  82cc:    mov      al, byte ptr [rsp + 4]                                                                 
    assert!(black_box(
  82d0:    xor      al, 0xff                                                                               
  82d2:    test     al, 1                                                                                  
  82d4:    jne      0x82db                                                                                 
}
  82d6:    add      rsp, 0x38                                                                              
  82da:    ret                                                                                             
    assert!(black_box(
  82db:    lea      rdi, [rip + 0x33eb3]                                                                   
  82e2:    lea      rdx, [rip + 0x4113f]                                                                   
  82e9:    lea      rax, [rip - 0x11b0]                                                                    
  82f0:    mov      esi, 0x55                                                                              
  82f5:    call     rax                                                                                    
  82f7:    ud2                                                                                             

```

### Using variables

``` asm

tests::var_main:
pub fn var_main() {
  8120:    sub      rsp, 0x78                                                                      
    let stack = flagstack!(const ExampleFlags);
  8124:    mov      rcx, qword ptr [rip + 0x412ad]                                                 
  812b:    mov      rax, qword ptr [rip + 0x412ae]                                                 
  8132:    mov      qword ptr [rsp + 8], rcx                                                       
  8137:    mov      qword ptr [rsp + 0x10], rax                                                    
  813c:    lea      rdi, [rsp + 8]                                                                 
    let stack = stack.push_tag::<{ Flag1 as u8 }>();
  8141:    call     flagstack::ConstFlagStack<T,_>::push_tag                                       
  8146:    mov      qword ptr [rsp + 0x20], rdx                                                    
  814b:    mov      qword ptr [rsp + 0x18], rax                                                    
  8150:    lea      rdi, [rsp + 0x18]                                                              
    let stack = stack.push_tag::<{ Flag2 as u8 }>();
  8155:    call     flagstack::ConstFlagStack<T,_>::push_tag                                       
  815a:    mov      qword ptr [rsp + 0x30], rdx                                                    
  815f:    mov      qword ptr [rsp + 0x28], rax                                                    
    assert!(black_box(stack.pop_tag().1.contains(Flag2)));
  8164:    lea      rdi, [rsp + 0x38]                                                              
  8169:    lea      rsi, [rsp + 0x28]                                                              
  816e:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                        
  8173:    lea      rdi, [rsp + 0x38]                                                              
  8178:    add      rdi, 0x10                                                                      
  817c:    mov      byte ptr [rsp + 0x57], 1                                                       
  8181:    movzx    esi, byte ptr [rsp + 0x57]                                                     
  8186:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  818b:    mov      byte ptr [rsp + 7], al                                                         
  818f:    mov      al, byte ptr [rsp + 7]                                                         
    assert!(black_box(stack.pop_tag().1.contains(Flag2)));
  8193:    movzx    edi, al                                                                        
  8196:    and      edi, 1                                                                         
  8199:    call     core::hint::black_box                                                          
  819e:    mov      byte ptr [rsp + 6], al                                                         
  81a2:    mov      al, byte ptr [rsp + 6]                                                         
    assert!(black_box(stack.pop_tag().1.contains(Flag2)));
  81a6:    xor      al, 0xff                                                                       
  81a8:    test     al, 1                                                                          
  81aa:    jne      0x81bd                                                                         
    assert!(black_box(!stack.pop_tag().1.contains(Flag1)));
  81ac:    lea      rdi, [rsp + 0x58]                                                              
  81b1:    lea      rsi, [rsp + 0x28]                                                              
  81b6:    call     flagstack::ConstFlagStack<T,_>::pop_tag                                        
  81bb:    jmp      0x81db                                                                         
    assert!(black_box(stack.pop_tag().1.contains(Flag2)));
  81bd:    lea      rdi, [rip + 0x33f25]                                                           
  81c4:    lea      rdx, [rip + 0x4121d]                                                           
  81cb:    lea      rax, [rip - 0x1092]                                                            
  81d2:    mov      esi, 0x3e                                                                      
  81d7:    call     rax                                                                            
  81d9:    ud2                                                                                     
    assert!(black_box(!stack.pop_tag().1.contains(Flag1)));
  81db:    lea      rdi, [rsp + 0x58]                                                              
  81e0:    add      rdi, 0x10                                                                      
  81e4:    mov      byte ptr [rsp + 0x77], 0                                                       
  81e9:    movzx    esi, byte ptr [rsp + 0x77]                                                     
  81ee:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  81f3:    mov      byte ptr [rsp + 5], al                                                         
  81f7:    mov      al, byte ptr [rsp + 5]                                                         
    assert!(black_box(!stack.pop_tag().1.contains(Flag1)));
  81fb:    xor      al, 0xff                                                                       
  81fd:    movzx    edi, al                                                                        
  8200:    and      edi, 1                                                                         
  8203:    call     core::hint::black_box                                                          
  8208:    mov      byte ptr [rsp + 4], al                                                         
  820c:    mov      al, byte ptr [rsp + 4]                                                         
    assert!(black_box(!stack.pop_tag().1.contains(Flag1)));
  8210:    xor      al, 0xff                                                                       
  8212:    test     al, 1                                                                          
  8214:    jne      0x821b                                                                         
}
  8216:    add      rsp, 0x78                                                                      
  821a:    ret                                                                                     
    assert!(black_box(!stack.pop_tag().1.contains(Flag1)));
  821b:    lea      rdi, [rip + 0x33f05]                                                           
  8222:    lea      rdx, [rip + 0x411d7]                                                           
  8229:    lea      rax, [rip - 0x10f0]                                                            
  8230:    mov      esi, 0x3f                                                                      
  8235:    call     rax                                                                            
  8237:    ud2                                                                                     

```

### Using a dynamic flagstack

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
    assert!(black_box(stack.pop_tag().contains(Flag2)));
  7c33:    lea      rdi, [rsp + 0x10]                                                              
  7c38:    call     flagstack::DynFlagStack<T>::pop_tag                                            
  7c3d:    mov      byte ptr [rsp + 0x64], al                                                      
  7c41:    mov      byte ptr [rsp + 0x65], 1                                                       
  7c46:    lea      rdi, [rsp + 0x64]                                                              
  7c4b:    movzx    esi, byte ptr [rsp + 0x65]                                                     
  7c50:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  7c55:    mov      byte ptr [rsp + 0xf], al                                                       
  7c59:    mov      al, byte ptr [rsp + 0xf]                                                       
    assert!(black_box(stack.pop_tag().contains(Flag2)));
  7c5d:    movzx    edi, al                                                                        
  7c60:    and      edi, 1                                                                         
  7c63:    call     core::hint::black_box                                                          
  7c68:    mov      byte ptr [rsp + 0xe], al                                                       
  7c6c:    mov      al, byte ptr [rsp + 0xe]                                                       
    assert!(black_box(stack.pop_tag().contains(Flag2)));
  7c70:    xor      al, 0xff                                                                       
  7c72:    test     al, 1                                                                          
  7c74:    jne      0x7c86                                                                         
    assert!(black_box(!stack.pop_tag().contains(Flag1)));
  7c76:    lea      rdi, [rsp + 0x10]                                                              
  7c7b:    call     flagstack::DynFlagStack<T>::pop_tag                                            
  7c80:    mov      byte ptr [rsp + 0x66], al                                                      
  7c84:    jmp      0x7ca4                                                                         
    assert!(black_box(stack.pop_tag().contains(Flag2)));
  7c86:    lea      rdi, [rip + 0x34395]                                                           
  7c8d:    lea      rdx, [rip + 0x4169c]                                                           
  7c94:    lea      rax, [rip - 0xb4b]                                                             
  7c9b:    mov      esi, 0x3c                                                                      
  7ca0:    call     rax                                                                            
  7ca2:    ud2                                                                                     
    assert!(black_box(!stack.pop_tag().contains(Flag1)));
  7ca4:    mov      byte ptr [rsp + 0x67], 0                                                       
  7ca9:    lea      rdi, [rsp + 0x66]                                                              
  7cae:    movzx    esi, byte ptr [rsp + 0x67]                                                     
  7cb3:    call     <tests::ExampleFlags as flagstack::DynFlaggable<tests::ExampleFlags>>::contains
  7cb8:    mov      byte ptr [rsp + 0xd], al                                                       
  7cbc:    mov      al, byte ptr [rsp + 0xd]                                                       
    assert!(black_box(!stack.pop_tag().contains(Flag1)));
  7cc0:    xor      al, 0xff                                                                       
  7cc2:    movzx    edi, al                                                                        
  7cc5:    and      edi, 1                                                                         
  7cc8:    call     core::hint::black_box                                                          
  7ccd:    mov      byte ptr [rsp + 0xc], al                                                       
  7cd1:    mov      al, byte ptr [rsp + 0xc]                                                       
    assert!(black_box(!stack.pop_tag().contains(Flag1)));
  7cd5:    xor      al, 0xff                                                                       
  7cd7:    test     al, 1                                                                          
  7cd9:    jne      0x7ce0                                                                         
}
  7cdb:    add      rsp, 0x68                                                                      
  7cdf:    ret                                                                                     
    assert!(black_box(!stack.pop_tag().contains(Flag1)));
  7ce0:    lea      rdi, [rip + 0x34377]                                                           
  7ce7:    lea      rdx, [rip + 0x4165a]                                                           
  7cee:    lea      rax, [rip - 0xba5]                                                             
  7cf5:    mov      esi, 0x3d                                                                      
  7cfa:    call     rax                                                                            
  7cfc:    ud2                                                                                     

```

