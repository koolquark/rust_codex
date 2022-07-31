## What-for-What 
- Shared ownership of a value allocated in the heap : Use ```Rc<T>```
- Mutability inside an Rc : Use ```Cell``` or ```RefCell```inside an ```Rc```. 
- Atomic Reference counting type : Use ```Rc```
- Multi threaded atomic reference count : Use ```Arc```

