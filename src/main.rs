// mod formatter;
// mod my;

// fn function() {
//     println!("called `function()`");
// }

// fn main() {
//     my::function();

//     function();

//     my::indirect_access();

//     my::nested::function();

//     formatter::formatter();
// }

// // 推导 `Structure` 的 `fmt::Debug` 实现。
// // `Structure` 是一个包含单个 `i32` 的结构体。
// #[derive(Debug)]
// struct Structure(i32);

// // 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//     // 使用 `{:?}` 打印和使用 `{}` 类似。
//     println!("{:?} months in a year.", 12);
//     println!(
//         "{1:?} {0:?} is the {actor:?} name.",
//         "Slater",
//         "Christian",
//         actor = "actor's"
//     );

//     // `Structure` 也可以打印！
//     println!("Now {:?} will print!", Structure(3));
//     // 使用 `derive` 的一个问题是不能控制输出的形式。
//     // 假如我只想展示一个 `7` 怎么办？
//     // println!("Now {:?} will print!", Deep(Structure(7)));
// }

// use std::fmt; // 导入 `fmt`

// // 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
// #[derive(Debug)]
// struct MinMax(i64, i64);

// // 实现 `MinMax` 的 `Display`。
// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 使用 `self.number` 来表示各个数据。
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// // 为了比较，定义一个含有具名字段的结构体。
// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// // 类似地对 `Point2D` 实现 `Display`
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 自定义格式，使得仅显示 `x` 和 `y` 的值。
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// fn main() {
//     let minmax = MinMax(0, 14);

//     println!("Compare structures:");
//     println!("Display: {}", minmax);
//     println!("Debug: {:?}", minmax);

//     let big_range = MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!(
//         "The big range is {big} and the small is {small}",
//         small = small_range,
//         big = big_range
//     );

//     let point = Point2D { x: 3.3, y: 7.2 };

//     println!("Compare points:");
//     println!("Display: {}", point);
//     println!("Debug: {:?}", point);

//     // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
//     // 得到实现。这语句不能运行。
//     // println!("What does Point2D look like in binary: {:b}?", point);
// }

// 一个名为 `my_mod` 的模块
// mod my_mod {
//     // 模块中的项默认具有私有的可见性
//     fn private_function() {
//         println!("called `my_mod::private_function()`");
//     }

//     // 使用 `pub` 修饰语来改变默认可见性。
//     pub fn function() {
//         println!("called `my_mod::function()`");
//     }

//     // 在同一模块中，项可以访问其它项，即使它是私有的。
//     pub fn indirect_access() {
//         print!("called `my_mod::indirect_access()`, that\n> ");
//         private_function();
//     }

//     // 模块也可以嵌套
//     pub mod nested {
//         pub fn function() {
//             println!("called `my_mod::nested::function()`");
//         }

//         #[allow(dead_code)]
//         fn private_function() {
//             println!("called `my_mod::nested::private_function()`");
//         }

//         // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
//         // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
//         // pub(in my_mod) fn public_function_in_my_mod() {
//         //     print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
//         //     public_function_in_nested()
//         // }

//         // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
//         pub(self) fn public_function_in_nested() {
//             println!("called `my_mod::nested::public_function_in_nested");
//         }

//         // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
//         pub(super) fn public_function_in_super_mod() {
//             println!("called my_mod::nested::public_function_in_super_mod");
//         }
//     }

//     pub fn call_public_function_in_my_mod() {
//         print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
//         nested::public_function_in_my_mod();
//         print!("> ");
//         nested::public_function_in_super_mod();
//     }

//     // `pub(crate)` 使得函数只在当前 crate 中可见
//     pub(crate) fn public_function_in_crate() {
//         println!("called `my_mod::public_function_in_crate()");
//     }

//     // 嵌套模块的可见性遵循相同的规则
//     mod private_nested {
//         #[allow(dead_code)]
//         pub fn function() {
//             println!("called `my_mod::private_nested::function()`");
//         }
//     }
// }

// fn function() {
//     println!("called `function()`");
// }

// fn main() {
//     // 模块机制消除了相同名字的项之间的歧义。
//     function();
//     my_mod::function();

//     // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
//     my_mod::indirect_access();
//     my_mod::nested::function();
//     my_mod::call_public_function_in_my_mod();

//     // pub(crate) 项可以在同一个 crate 中的任何地方访问
//     my_mod::public_function_in_crate();

//     // pub(in path) 项只能在指定的模块中访问
//     // 报错！函数 `public_function_in_my_mod` 是私有的
//     //my_mod::nested::public_function_in_my_mod();
//     // 试一试 ^ 取消该行的注释

//     // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

//     // 报错！`private_function` 是私有的
//     //my_mod::private_function();
//     // 试一试 ^ 取消此行注释

//     // 报错！`private_function` 是私有的
//     //my_mod::nested::private_function();
//     // 试一试 ^ 取消此行的注释

//     // Error! `private_nested` is a private module
//     // my_mod::private_nested::function();
//     // 试一试 ^ 取消此行的注释
// }

// mod my {
//     // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
//     pub struct OpenBox<T> {
//         pub contents: T,
//     }

//     // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
//     #[allow(dead_code)]
//     pub struct ClosedBox<T> {
//         contents: T,
//     }

//     impl<T> ClosedBox<T> {
//         // 一个公有的构造器方法
//         pub fn new(contents: T) -> ClosedBox<T> {
//             ClosedBox { contents: contents }
//         }
//     }
// }

// fn main() {
//     // 带有公有字段的公有结构体，可以像平常一样构造
//     let open_box = my::OpenBox {
//         contents: "public information",
//     };

//     // 并且它们的字段可以正常访问到。
//     println!("The open box contains: {}", open_box.contents);

//     // 带有私有字段的公有结构体不能使用字段名来构造。
//     // 报错！`ClosedBox` 含有私有字段。
//     //let closed_box = my::ClosedBox { contents: "classified information" };
//     // 试一试 ^ 取消此行注释

//     // 不过带有私有字段的结构体可以使用公有的构造器来创建。
//     let _closed_box = my::ClosedBox::new("classified information");

//     // 并且一个结构体中的私有字段不能访问到。
//     // 报错！`content` 字段是私有的。
//     //println!("The closed box contains: {}", _closed_box.contents);
//     // 试一试 ^ 取消此行注释
// }

// 将 `deeply::nested::function` 路径绑定到 `other_function`。
// use deeply::nested::function as other_function;

// fn function() {
//     println!("called `function()`");
// }

// mod deeply {
//     pub mod nested {
//         pub fn function() {
//             println!("called `deeply::nested::function()`")
//         }
//     }
// }

// fn main() {
//     // 更容易访问 `deeply::nested::funcion`
//     other_function();

//     println!("Entering block");
//     {
//         // 这和 `use deeply::nested::function as function` 等价。
//         // 此 `function()` 将掩蔽外部的同名函数。
//         use deeply::nested::function;
//         function();

//         // `use` 绑定拥有局部作用域。在这个例子中，`function()`
//         // 的掩蔽只存在在这个代码块中。
//         println!("Leaving block");
//     }

//     function();
// }

// fn function() {
//     println!("called `function()`");
// }

// mod cool {
//     pub fn function() {
//         println!("called `cool::function()`");
//     }
// }

// mod my {
//     fn function() {
//         println!("called `my::function()`");
//     }
//     mod cool {
//         pub fn function() {
//             println!("called `my::cool::function()`");
//         }
//     }
//     pub fn indirect_call() {
//         // 让我们从这个作用域中访问所有名为 `function` 的函数！
//         print!("called `my::indirect_call()`, that\n> ");
//         // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
//         // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
//         // 因为他们表示相同的函数。
//         self::function();
//         function();
//         // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
//         self::cool::function();
//         // `super` 关键字表示父作用域（在 `my` 模块外面）。
//         super::function();
//         // 这将在 *crate* 作用域内绑定 `cool::function` 。
//         // 在这个例子中，crate 作用域是最外面的作用域。
//         {
//             use crate::cool::function as root_function;
//             root_function();
//         }
//     }
// }

// fn main() {
//     my::indirect_call();
// }

// // 此函数取得堆分配的内存的所有权
// fn destroy_box(c: Box<i32>) {
//     println!("Destroying a box that contains {}", c);

//     // `c` 被销毁且内存得到释放
// }

// fn main() {
//     // 栈分配的整型
//     let x = 5u32;

//     // 将 `x` *复制*到 `y`——不存在资源移动
//     let y = x;

//     // 两个值各自都可以使用
//     println!("x is {}, and y is {}", x, y);

//     // `a` 是一个指向堆分配的整数的指针
//     let a = Box::new(5i32);

//     println!("a contains: {}", a);

//     // *移动* `a` 到 `b`
//     let b = a;
//     // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
//     // 同一个堆分配的数据，但是现在是 `b` 拥有它。
//     // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
//     println!("a contains: {}", a);
//     // 试一试 ^ 去掉此行注释

//     // 此函数从 `b` 中取得堆分配的内存的所有权
//     destroy_box(b);

//     // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
//     // 报错！和前面出错的原因一样。
//     //println!("b contains: {}", b);
//     // 试一试 ^ 去掉此行注释
// }

// fn main() {
//     let immutable_box = Box::new(5u32);

//     println!("immutable_box contains {}", immutable_box);

//     // 可变性错误
//     //*immutable_box = 4;

//     // *移动* box，改变所有权（和可变性）
//     let mut mutable_box = immutable_box;

//     println!("mutable_box contains {}", mutable_box);

//     // 修改 box 的内容
//     *mutable_box = 4;

//     println!("mutable_box now contains {}", mutable_box);
// }

// // 此函数取得一个 box 的所有权并销毁它
// fn eat_box_i32(boxed_i32: Box<i32>) {
//     println!("Destroying box that contains {}", boxed_i32);
// }

// // 此函数借用了一个 i32 类型
// fn borrow_i32(borrowed_i32: &i32) {
//     println!("This int is: {}", borrowed_i32);
// }

// fn main() {
//     // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
//     let boxed_i32 = Box::new(5_i32);
//     let stacked_i32 = 6_i32;

//     // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
//     // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
//     // 在函数中临时创建的引用也就不复存在了。
//     borrow_i32(&boxed_i32);
//     borrow_i32(&stacked_i32);

//     {
//         // 取得一个对 box 中数据的引用
//         let _ref_to_i32: &i32 = &boxed_i32;

//         // 报错！
//         // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
//         eat_box_i32(boxed_i32);
//         // 改正 ^ 注释掉此行

//         // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
//         //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
//         borrow_i32(_ref_to_i32);
//         // `_ref_to_i32` 离开作用域且不再被借用。
//     }

//     // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
//     //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
//     eat_box_i32(boxed_i32);
// }

// #[allow(dead_code)]
// #[derive(Clone, Copy)]
// struct Book {
//     // `&'static str` 是一个对分配在只读内存区的字符串的引用
//     author: &'static str,
//     title: &'static str,
//     year: u32,
// }

// // 此函数接受一个对 Book 类型的引用
// fn borrow_book(book: &Book) {
//     println!("I immutably borrowed {} - {} edition", book.title, book.year);
// }

// // 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
// fn new_edition(book: &mut Book) {
//     book.year = 2014;
//     println!("I mutably borrowed {} - {} edition", book.title, book.year);
// }

// fn main() {
//     // 创建一个名为 `immutabook` 的不可变的 Book 实例
//     let immutabook = Book {
//         // 字符串字面量拥有 `&'static str` 类型
//         author: "Douglas Hofstadter",
//         title: "Gödel, Escher, Bach",
//         year: 1979,
//     };

//     // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
//     let mut mutabook = immutabook;
//     // 不可变地借用一个不可变对象
//     borrow_book(&immutabook);

//     // 不可变地借用一个可变对象
//     borrow_book(&mutabook);
//     // 可变地借用一个可变对象
//     new_edition(&mut mutabook);
//     // 报错！不能可变地借用一个不可变对象
//    // new_edition(&mut immutabook);
//     // 改正 ^ 注释掉此行
// }

// fn main() {
//     let mut _mutable_integer = 7i32;

//     {
//         // 借用 `_mutable_integer`
//         let large_integer = &_mutable_integer;

//         // 报错！`_mutable_integer` 在本作用域被冻结
//         _mutable_integer = 50;
//         // 改正 ^ 注释掉此行

//         println!("Immutably borrowed {}", large_integer);

//         // `large_integer` 离开作用域
//     }

//     // 正常运行！`_mutable_integer` 在这作用域没有冻结
//     _mutable_integer = 3;
// }

// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// fn main() {
//     let mut point = Point { x: 0, y: 0, z: 0 };

//     {
//         let borrowed_point = &point;
//         let another_borrow = &point;

//         // 通过引用和原始所有者来访问数据
//         println!(
//             "Point has coordinates: ({}, {}, {})",
//             borrowed_point.x, another_borrow.y, point.z
//         );

//         // 报错！不能可变地借用 `point` ，因为现在它有不可变的借用。
//         //let mutable_borrow = &mut point;
//         // 试一试 ^ 取消此行注释。

//         // 此处再次使用被借用的值
//         println!(
//             "Point has coordinates: ({}, {}, {})",
//             borrowed_point.x, another_borrow.y, point.z
//         );

//         // 不可变引用离开作用域
//     }

//     {
//         let mutable_borrow = &mut point;

//         // 通过可变引用来改变数据
//         mutable_borrow.x = 5;
//         mutable_borrow.y = 2;
//         mutable_borrow.z = 1;

//         // 报错！不能不可变地借用 `point`，因为现在它有可变的借用。
//         //let y = &point.y;
//         // 试一试 ^ 取消此行注释。

//         // 报错！不能打印，因为 `println!` 会创建一个不可变引用。
//         //println!("Point Z coordinate is {}", point.z);
//         // 试一试 ^ 取消此行注释。

//         // 可以工作！可变引用可以作为不可变的传给 `println!`。
//         println!(
//             "Point has coordinates: ({}, {}, {})",
//             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
//         );

//         // 可变引用离开作用域
//     }

//     // 现在又可以不可变地借用 `point` 了。
//     let borrowed_point = &point;
//     println!(
//         "Point now has coordinates: ({}, {}, {})",
//         borrowed_point.x, borrowed_point.y, borrowed_point.z
//     );
// }

// fn main() {
//     let a1 = 5;
//     let a2: i32 = 5;
//     assert_eq!(a1, a2);
//     //let 绑定 整数变量默认类型推断是 i32

//     let b1: u32 = 5;
//     //assert_eq!(a1, b1);
//     //去掉上面的注释会报错，因为类型不匹配
//     //errer: mismatched types
// }

// fn main() {
//     // let mut a: f64 = 1.0;

//     // println!("{:?}", a);
//     // a = 2.0;
//     // println!("{:?}", a);

//     // let a: f64 = 1.0;
//     // println!("{:?}", a);

//     // 报错
//     // let mut a: f64 = 1.0;
//     // a = 2.0;
//     // println!("{:?}", a);
// }

// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     println!("a = {:?}, b = {:?}", a, b);
//     //a 不可变绑定
//     //a = false;
//     //b 可变绑定
//     b = true;
//     assert_eq!(a, b);
// }

// fn main() {
//     let mut array = [0; 3];

//     array[1] = 1;
//     array[2] = 2;

//     assert_eq!([1, 2], &array[1..]);

//     // This loop prints: 0 1 2
//     for x in &array {
//         println!("{} ", x);
//     }
// }

// fn main() {
//     //创建空Vec
//     let v: Vec<i32> = Vec::new();
//     //使用宏创建空Vec
//     let v: Vec<i32> = vec![];
//     //创建包含5个元素的Vec
//     let v = vec![1, 2, 3, 4, 5];
//     //创建十个零
//     let v = vec![0; 10];
//     //创建可变的Vec，并压入元素3
//     let mut v = vec![1, 2];
//     v.push(3);
//     //创建拥有两个元素的Vec，并弹出一个元素
//     let mut v = vec![1, 2];
//     let two = v.pop();
//     //创建包含三个元素的可变Vec，并索引一个值和修改一个值
//     let mut v = vec![1, 2, 3];
//     let three = v[2];
//     v[1] = v[1] + 5;
// }

// fn main() {
//     // 字符串字面值
//     //let hello = "Hello, world!";
//     // 附带显式类型标识
//     let hello: &'static str = "Hello, world!";
//     println!("{}", hello)
// }

// fn main() {
//     // 创建一个空的字符串
//     //let mut s = String::new();
//     // 从 `&str` 类型转化成 `String` 类型
//     let mut hello = String::from("Hello, ");
//     // 压入字符和压入字符串切片
//     hello.push('w');
//     hello.push_str("orld!");

//     // 弹出字符。
//     let mut s = String::from("foo");
//     assert_eq!(s.pop(), Some('o'));
//     assert_eq!(s.pop(), Some('o'));
//     assert_eq!(s.pop(), Some('f'));
//     assert_eq!(s.pop(), None);
// }

// fn main() {
//     // structs
//     struct Point {
//         x: i32,
//         y: i32,
//     }
//     let point = Point { x: 0, y: 0 };
//     // tuple structs
//     struct Color(u8, u8, u8);
//     let android_green = Color(0xa4, 0xc6, 0x39);
//     let Color(red, green, blue) = android_green;
// }

// fn main() {
//     // // structs
//     // struct Point {
//     //     x: i32,
//     //     y: i32,
//     // }
//     // let point = Point { x: 0, y: 0 };

//     // // tuple structs
//     // struct Color(u8, u8, u8);
//     // let android_green = Color(0xa4, 0xc6, 0x39);
//     // let Color(red, green, blue) = android_green;

//     // // A tuple struct’s constructors can be used as functions.
//     // struct Digit(i32);
//     // let v = vec![0, 1, 2];
//     // let d: Vec<Digit> = v.into_iter().map(Digit).collect();

//     // // newtype: a tuple struct with only one element
//     // struct Inches(i32);
//     // let length = Inches(10);
//     // let Inches(integer_length) = length;

//     // // unit-like structs
//     // struct EmptyStruct;
//     // let empty = EmptyStruct;

//     // #[derive(Default)]
//     // struct Point3d {
//     //     x: i32,
//     //     y: i32,
//     //     z: i32,
//     // }

//     // let origin = Point3d::default();
//     // let point = Point3d { y: 1, ..origin };
//     // let Point3d { x: x0, y: y0, .. } = point;
// }

// fn main() {
//     // let x = 5;
//     // let mut y = 5;

//     // match x {
//     //     // the `r` inside the match has the type `&i32`
//     //     ref r => println!("Got a reference to {}", r),
//     // }

//     // match y {
//     //     // the `mr` inside the match has the type `&i32` and is mutable
//     //     ref mut mr => println!("Got a mutable reference to {}", mr),
//     // }

//     // let pair = (0, -2);

//     // match pair {
//     //     (0, y) => println!("x is `0` and `y` is `{:?}`", y),
//     //     (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
//     //     _ => println!("It doesn't matter what they are"),
//     // }

//     // struct Point {
//     //     x: i32,
//     //     y: i32,
//     // }
//     // let origin = Point { x: 0, y: 0 };
//     // match origin {
//     //     Point { x, .. } => println!("x is {}", x),
//     // }

//     // enum OptionalInt {
//     //     Value(i32),
//     //     Missing,
//     // }
//     // let x = OptionalInt::Value(5);
//     // match x {
//     //     // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
//     //     OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
//     //     OptionalInt::Value(..) => println!("Got an int!"),
//     //     OptionalInt::Missing => println!("No such luck."),
//     // }

//     let number = Some(7);
//     let mut optional = Some(0);

//     // If `let` destructures `number` into `Some(i)`, evaluate the block.
//     if let Some(i) = number {
//         println!("Matched {:?}!", i);
//     } else {
//         println!("Didn't match a number!");
//     }

//     // While `let` destructures `optional` into `Some(i)`, evaluate the block.
//     while let Some(i) = optional {
//         if i > 9 {
//             println!("Greater than 9, quit!");
//             optional = None;
//         } else {
//             println!("`i` is `{:?}`. Try again.", i);
//             optional = Some(i + 1);
//         }
//     }
// }

// struct Circle {
//     x: f64,
//     y: f64,
//     radius: f64,
// }
// impl Circle {
//     fn new(x: f64, y: f64, radius: f64) -> Circle {
//         Circle {
//             x: x,
//             y: y,
//             radius: radius,
//         }
//     }
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * (self.radius * self.radius)
//     }}
// fn main() {
//     let c = Circle {
//         x: 0.0,
//         y: 0.0,
//         radius: 2.0,
//     };
//     println!("{}", c.area());
//     // use associated function and method chaining
//     println!("{}", Circle::new(0.0, 0.0, 2.0).area());
// }

// fn main() {
//     struct D {};
// }

// struct Person {
//     name: String,
// }

// impl Person {
//     fn new(n: &str) -> Person {
//         Person {
//             name: n.to_string(),
//         }
//     }

//     fn greeting(&self) {
//         println!("{} say hello .", self.name);
//     }
// }

// fn main() {
//     let peter = Person::new("Peter");
//     peter.greeting();
// }

// fn main() {
//     // let y = "Hello";
//     // let x: &'static str = "Hello";
//     // println!("{}", x == y)

//     // //没有转义序列
//     // let d: &'static str = r"abc \n abc";
//     // //等价于
//     // let c: &'static str = "abc \\n abc";
//     // println!("{}{}", c, d)

//     // let x: &'static str = "hello";

//     // let mut y: String = x.to_string();
//     // println!("{}", y);
//     // y.push_str(", world");
//     // println!("{}", y);

//     let x = "哎哟我去".to_string();
//     for i in x.as_bytes() {
//         print!("{} ", i);
//     }

//     println!("");

//     for i in x.chars() {
//         print!("{}", i);
//     }

//     x.chars().nth(2);
// }

// fn main() {
//     let xm = "xiaoming";
//     let xh = "xiaohong";
//     say_what(xm, hi);
//     say_what(xh, hello);
// }

// fn hi(name: &str) {
//     println!("Hi, {}.", name);
// }

// fn hello(name: &str) {
//     println!("Hello, {}.", name);
// }

// fn say_what(name: &str, func: fn(&str)) {
//     func(name)
// }
