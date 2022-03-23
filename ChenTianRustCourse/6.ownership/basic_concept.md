对值的引用，Rust 给出了如下规则：
1. 一个值只能被一个变量所拥有，这个变量称为所有者（Each value in Rust has a variable that's called its owner）。
2. 一个值同一时刻只能有一个人所有者（There can only be one owner at that timr），也就是说不能有两个变量同时拥有同一个值。
3. 当所有者离开作用域，其拥有的值将被回收。（When the owner goes out of scope, the value will be dropped），内存被释放。

---

owership 规则让堆上的数据同一时刻只存在一份引用，消除了多重引用，这是它最大的优势。

如果不希望所有权被转移，可以采用 Copy 语义，如果一个数据结构实现了 Copy trait，在赋值或传参时，值会自动按位拷贝（浅拷贝）。   

除此之外，还可以选择“借用”数据。  
