#![allow(non_ascii_idents)]
#![allow(uncommon_codepoints)]

#[rustfmt::skip]
fn transmute<ㅡ,ㅣ>(ㆍ:ㅡ)->ㅣ{
    trait ㅗ:ㅿ<ㅡ=<Self as ㅿ>::ㅣ>{}
    trait ㅿ{type ㅡ;type ㅣ;}
    fn ㅜ<ㅡ,ㅣ,T:?Sized+ㅗ<ㅣ=ㅣ>>(ㆍ:T::ㅡ)->ㅣ{ㆍ}
    ㅜ::<ㅡ,ㅣ,dyn ㅗ<ㅡ=ㅡ,ㅣ=ㅣ>>(ㆍ)
}

fn main() {
    let a: &[u8] = &[
        69, 97, 116, 32, 97, 115, 115, 44, 32, 115, 109, 111, 107, 101, 32, 103, 114, 97, 115, 115,
        44, 32, 115, 108, 101, 100, 32, 102, 97, 115, 116,
    ];
    let b: &str = transmute(a);
    println!("{}", b);
}
