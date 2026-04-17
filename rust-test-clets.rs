macro_rules! clet
{
    ($t:ty : $name:ident) => { let mut $name : $t; };
    ($t:ty : $id:ident = $exp:expr) => { let mut $id : $t = $exp; };
    (const $t:ty : $id:ident) => { let $id : $t; };
    (const $t:ty : $id:ident = $exp:expr) => { let $id : $t = $exp; };
    ($t:ty : * $name:ident) => { let $name : &mut $t; };
    ($t:ty : * $id:ident = $exp:expr) => { let $id : &mut $t = $exp; };
    (const $t:ty : * $id:ident) => { let $id : &$t; };
    (const $t:ty : * $id:ident = $exp:expr) => { let $id : &$t = $exp; };
    ($t:ty : $name:ident [$n:expr]) => { let mut $name : [$t;$n]; };
    ($t:ty : $id:ident [$n:expr] = $exp:expr) => { let mut $id : [$t;$n] = $exp; };
    (const $t:ty : $id:ident [$n:expr]) => { let $id : [$t;$n]; };
    (const $t:ty : $id:ident [$n:expr] = $exp:expr) => { let $id : [$t;$n] = $exp; };
}

macro_rules! clets
{
    ($(#[$($l:tt)+])*)
    =>
    {
        $(
            clet!($($l)+);
        )*
    };
}

fn main()
{
    clets!{
        #[ i8 : x = 7 ]
        #[ const i8 : *p = &x ]
        #[ i8 : arr[32] = [0;32] ]
    };

    println!("*p = {p}");
   
    for ai in &mut arr
    { *ai = x; x += 1; }
   
    println!("{arr:?}");
}
