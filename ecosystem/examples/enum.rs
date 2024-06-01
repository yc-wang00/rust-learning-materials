use anyhow::Result;
use strum::{
    Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[derive(
    Debug,
    PartialEq,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    EnumIter,
    EnumString,
    IntoEnumIterator,
    IntoStaticStr,
    VariantNames,
    Display,
)]
enum MyEnum {
    A,
    B,
    C,
}

fn main() -> Result<()> {
    let a = MyEnum::A;
    let b = MyEnum::B;
    let c = MyEnum::C;

    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("MyEnum has {} variants", MyEnum::COUNT);

    for variant in MyEnum::iter() {
        println!("variant: {}", variant);
    }

    Ok(())
}
