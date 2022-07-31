## Online Notes 
- Cloning an Rc produces new pointer which points to same allocation on the heap. 
- Rc does not allow mutable reference , since Rc is a shared reference. 
- Rc uses atomic reference counting
- Weak Reference does not count towards ownership. 
- Weak Reference will not prevent value store in allocatin from being dropped. 
- Weak reference does not gurantee about value still being present. 
- 
