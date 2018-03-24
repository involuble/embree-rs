macro_rules! into_primitive {
    ($enum_name:ty, $prim:ty) => (
        impl Into<$prim> for $enum_name {
            fn into(self) -> $prim {
                unsafe { ::std::mem::transmute::<$enum_name, $prim>(self) }
            }
        }
    )
}