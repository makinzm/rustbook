// Eq を実装するために PartialEq が必要
#[derive(Eq, PartialEq)]
struct A(i32);

impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A({})", self.0)
    }
}

// PartialOrd を実装するために PartialEq が必要
#[derive(PartialEq, PartialOrd)]
struct B(f32);

// Copy を実装するために Clone が必要
#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;

fn main() {
    // A は一致比較可能
    println!("{:?}", A(0) == A(1));

    let a1 = A(1);
    let a2 = A(2);
    if a1 != a2 {
        eprintln!("Error: {} is not equal to {}", a1, a2);
    }

    // B は大小比較可能
    println!("{:?}", B(1.0) > B(0.0));

    // C はムーブではなくコピーされる
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // Cがムーブならc0はc1へムーブしているのでここでコンパイルエラー

    // D はclone可能
    let d0 = D;
    let _d1 = d0.clone();

    // E はデバッグプリント{:?}可能
    println!("{:?}", E);

    // F はdefault可能
    let _f = F::default();

    panic!("PANIC!!");

    unreachable!("You cannot come here!!!!");
    
}
