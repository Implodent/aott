use super::*;
use crate::MaybeUninit;
use crate::MaybeUninitExt;

/// See [`group`].
#[derive(Copy, Clone)]
pub struct Tuple<T> {
        parsers: T,
}

/// Parse using a tuple of many parsers, producing a tuple of outputs if all successfully parse,
/// otherwise returning an error if any parsers fail.
///
/// This parser is to [`Parser::then`] as [`choice`] is to [`Parser::or`]
pub const fn tuple<T>(parsers: T) -> Tuple<T> {
        Tuple { parsers }
}

// impl<I, O, E, P, const N: usize> Parser<I, [O; N], E> for Tuple<[P; N]>
// where
//         I: InputType,
//         E: ParserExtras<I>,
//         P: Parser<I, O, E>,
// {
//         #[inline]
//         fn explode<'parse, M: Mode>(
//                 &self,
//                 inp: Input<'parse, I, E>,
//         ) -> PResult<'parse, I, E, M, [O; N]> {
//                 let mut arr: [MaybeUninit<_>; N] = MaybeUninitExt::uninit_array();
//             let mut i = inp;
//                 for (p, res) in self.parsers
//                         .iter()
//                         .zip(arr.iter_mut()) {
//                             let (input, out) = match p.explode::<M>(inp) {
//                                 (input)
//                             };

//                         }
//                         // .try_for_each(|(p, res)| {
//                         //         res.write(p.explode::<M>(inp)?);
//                         //         Ok(())
//                         // })?;
//                 // SAFETY: We guarantee that all parers succeeded and as such all items have been initialized
//                 //         if we reach this point
//                 Ok(M::array(unsafe { MaybeUninitExt::array_assume_init(arr) }))
//         }

//         explode_extra!([O; N]);
// }

macro_rules! flatten_map {
    // map a single element into a 1-tuple
    (<$M:ident> $head:ident) => {
        $M::map(
            $head,
            |$head| ($head,),
        )
    };
    // combine two elements into a 2-tuple
    (<$M:ident> $head1:ident $head2:ident) => {
        $M::combine(
            $head1,
            $head2,
            |$head1, $head2| ($head1, $head2),
        )
    };
    // combine and flatten n-tuples from recursion
    (<$M:ident> $head:ident $($X:ident)+) => {
        $M::combine(
            $head,
            flatten_map!(
                <$M>
                $($X)+
            ),
            |$head, ($($X),+)| ($head, $($X),+),
        )
    };
}

macro_rules! impl_group_for_tuple {
    () => {};
    ($head:ident $ohead:ident $($X:ident $O:ident)*) => {
        impl_group_for_tuple!($($X $O)*);
        impl_group_for_tuple!(~ $head $ohead $($X $O)*);
    };
    (~ $($X:ident $O:ident)*) => {
        #[allow(unused_variables, non_snake_case)]
        impl<I, E, $($X),*, $($O),*> Parser<I, ($($O,)*), E> for Tuple<($($X,)*)>
        where
            I: InputType,
            E: ParserExtras<I>,
            $($X: Parser<I, $O, E>),*
        {
            #[inline]
            fn explode<'parse, M: Mode>(&self, inp: Input<'parse, I, E>) -> PResult<'parse, I, E, M, ($($O,)*)> {
                let Tuple { parsers: ($($X,)*) } = self;

                $(
                    let (inp, $X) = match $X.explode::<M>(inp) {
                        (inp, Ok(ok)) => (inp, ok),
                        (inp, Err(())) => return (inp, Err(()))
                    };
                )*

                (inp, Ok(flatten_map!(<M> $($X)*)))
            }

            explode_extra!(($($O,)*));
        }
    };
}

impl_group_for_tuple! {
    A_ OA
    B_ OB
    C_ OC
    D_ OD
    E_ OE
    F_ OF
    G_ OG
    H_ OH
    I_ OI
    J_ OJ
    K_ OK
    L_ OL
    M_ OM
    N_ ON
    O_ OO
    P_ OP
    Q_ OQ
    R_ OR
    S_ OS
    T_ OT
    U_ OU
    V_ OV
    W_ OW
    X_ OX
    Y_ OY
    Z_ OZ
}
