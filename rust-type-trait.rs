trait TypeTrait
{
    type Type;
}


struct UnrefStruct<T> (T);

impl<T> TypeTrait for UnrefStruct<&T>
{
    type Type = T;
}

type Unref<T> = <UnrefStruct::<T> as TypeTrait>::Type;


struct PassTypeStruct<T> (T);

impl<T> TypeTrait for PassTypeStruct<T>
{
    type Type = T;
}

type PassType<T> = <PassTypeStruct::<T> as TypeTrait>::Type;


fn main () -> ()
{
    type T = &'static u8;

    println!("{}", std::any::type_name::<T>()); // stdout: &u8
    println!("{}", std::any::type_name::<Unref::<T>>()); // stdout: u8
}
