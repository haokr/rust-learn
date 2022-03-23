see [impl_copy_trait.rs](./impl_copy_trait.rs).

实现了 Copy 的类型：
- 原生类型，包括函数、不可变引用和裸指针都实现了 Copy；
- 数组和元组，如果其内部的数据结构实现了 Copy，那么它们也实现了 Copy；
- 可变引用没有实现 Copy；
- 非固定大小的数据结构，没有实现 Copy。

[Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html) 文档介绍了 Rust 标准库中实现了 Copy trait 的所有数据结构。

---

__问：在 Rust 下，分配在堆上的数据结构可以引用栈上的数据么？为什么?__  
答：可以，只要保证在堆上数据被回收前，栈上数据一致存在即可，避免悬垂指针。贴评论区大佬见解：  
> 那么来看第一题，问的是堆上数据是否可以引用栈上的数据，我选择抛开堆栈不谈，因为不管分配到堆栈上都是分配到了内存上
在所有权机制的限制之下，可不可以引用这个问题其实就变成了如何避免悬垂引用，那么如何避免呢？使用生命周期(老师在抛砖引玉xdm)

__问：main() 函数传递给 find_pos() 函数的另一个参数 v，也会被移动吧？为什么图上并没有将其标灰？__  
答：因为参数 v 是 `i32` 类型，实现了 Copy trait，在传入方法时，会拷贝一份传给方法参数，因此方法参数和调用方作用域内变量的引用的值分别是两个存在栈上的值。当方法执行结束时，方法参数引用的值会被回收，方法调用方作用域内变量引用的值不受影响。

---

__补充（来自评论区）__

(1)  
常见的内存安全问题: 内存泄漏(非内存安全问题) , 堆栈溢出(迭代器/运行时检查), 重复释放, 悬垂指针;
所有权先解决重复释放的问题.
rust中,为了处理内存管理问题,要求每个内存对象(无论堆上或者栈上)仅有一个所有者,也就是所有权.
当所有者超出其作用域时,其管理的内存对象将会被释放, 这里分两种: 栈上内存由编译器自动管理,无需额外释放. 堆上内存会调用内存对象的Drop Trait. 这里就保证了不会发生重复释放.
rust中为了保证一块内存仅有一个所有者, 在所有权转移时(赋值，函数调用，函数返回)默认使用move语义, 也就是转移对象所有权. 除非对象实现了copy语义,那么会优先使用copy语义.
copy语义的作用类似于浅拷贝,仅拷贝栈上的内存.如基础类型, 裸指针,组合类型(其成员全部实现copy语义), 引用等.此时还是一块内存仅有一个所有者,只是内存被复制了一份. 因为栈上通常内存不大,那么此时发生了消耗较少的拷贝.
在rust语言机制上,不允许copy trait和drop trait同时实现,因为允许copy的,都在栈上. 栈上的内存管理是不需要开发者操心的,只有堆上的内存需要, 类似于C++的析构函数.
在rust语言机制上,clone trait是copy trait的supertait,也就是基类. copy trait的调用是由编译器默认调用的, 而clone trait则是开发者通过clone方法调用的.在了解了copy语义的作用后,clone语义也比较好理解,基本就是深拷贝了.那么深拷贝后的堆内存,通常也需要实现Drop Trait以保证内存不泄漏. clone相较栈消耗要大得多,因此为了避免拷贝,就引入了*borrow*的概念,类似C++的引用. 但引用又会带来悬垂指针的问题,这就需要通过*生命周期*来解决.
以上就是目前对所有权的理解.   

(2)  
另外对按位复制补充一点：

按位复制，等同于 C 语言里的 memcpy。

C 语言中的 memcpy 会从源所指的内存地址的起始位置开始拷贝 n 个字节，直到目标所指的内存地址的结束位置。但如果要拷贝的数据中包含指针，该函数并*不会*连同指针指向的数据一起拷贝。

因此如果是不包含指针的原生类型，那么按位复制(浅拷贝)等同于 clone，可如果是 Vec 这种在堆上开辟，在栈上存储胖指针的数据就不一样了，因为按位复制会拷贝胖指针本身，而其指向的堆中数据则不会拷贝，因此堆上的数据仍然只有一份。

最后，最好不用去实现 Copy。

__作者回复__: 对。不过实现 Copy 并不会影响程序的正确性。不会出现拷贝可能会被释放的内存的指针的问题。

Rust 在设计时就已经保证了你无法为一个在堆上分配内存的结构实现 Copy。所以 Vec / String 等结构是不能实现 Copy 的。因为这条路已经被堵死了：Copy trait 和 Drop trait 不能共存。一旦你实现了 Copy trait，就无法实现 Drop trait。反之亦然。

有同学看到裸指针 *const T/ *mut T 实现了 Copy，就会想如果我用 unsafe 把 Vec<T> 的指针取出来，组成一个数据结构，到处 Copy，然后其中一个 drop 后，岂不就造成 use after free，破坏了 Rust 的安全性保证？很遗憾，Rust 并不允许你这么做。因为你无法实现 Drop。

我写了一段代码，感兴趣的同学可以看一下：

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=4828e734f6f161dfce32098333a1aaa5

```rust
use std::{fmt, slice};

#[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释掉，看看会出什么问题
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();

    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(buf);

    // buf 还能用
    println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    drop(buf)
}
```