use paste::paste;
use seq_macro::seq;

seq!(N in 1..=12{
    paste!{
        pub(crate) mod [<d_ N>];
    }
});
