pub mod city;
pub mod connection;
pub mod walk;

#[cfg(test)]
mod test {
    use crate::{
        city::City,
        connection::{Connection, Symmetric},
        walk::Walk,
    };

    use super::*;

    use city::{NewYork, Odense, Rome, Sydney, Tokyo};
    use walk::{Composite, Direct};

    #[test]
    fn connections_work() {
        const _: () = connection::assert::<NewYork, Rome, (NewYork, Rome)>();
        const _: () = connection::assert::<Odense, Rome, (Odense, Rome)>();
        const _: () = connection::assert::<Rome, Sydney, (Rome, Sydney)>();
        const _: () = connection::assert::<Sydney, Tokyo, (Sydney, Tokyo)>();
        const _: () = connection::assert::<Tokyo, NewYork, (Tokyo, NewYork)>();
    }

    #[test]
    fn walks_work() {
        const _: () = walk::assert::<NewYork, Rome, Direct<(NewYork, Rome)>>();

        const _: () = walk::assert::<
            NewYork,
            Sydney,
            Composite<Direct<(NewYork, Rome)>, Direct<(Rome, Sydney)>>,
        >();

        const _: () = walk::assert::<
            NewYork,
            Tokyo,
            Composite<
                Direct<(NewYork, Rome)>,
                Composite<Direct<(Rome, Sydney)>, Direct<(Sydney, Tokyo)>>,
            >,
        >();

        const _: () = walk::assert::<
            NewYork,
            Tokyo,
            Composite<
                Composite<Direct<(NewYork, Rome)>, Direct<(Rome, Sydney)>>,
                Direct<(Sydney, Tokyo)>,
            >,
        >();
    }

    #[test]
    fn exercise_1_1() {
        const _: () = walk::assert::<
            NewYork,
            Sydney,
            Composite<Direct<(NewYork, Rome)>, Direct<(Rome, Sydney)>>,
        >();
    }

    #[test]
    fn exercise_1_2() {
        const _: () = walk::assert::<
            Odense,
            NewYork,
            Composite<
                Composite<Direct<(Odense, Rome)>, Direct<(Rome, Sydney)>>,
                Composite<Direct<(Sydney, Tokyo)>, Direct<(Tokyo, NewYork)>>,
            >,
        >();
    }

    #[test]
    fn exercise_1_3() {
        const _: () = walk::assert::<
            NewYork,
            NewYork,
            Composite<
                Direct<(NewYork, Rome)>,
                Composite<
                    Composite<Direct<(Rome, Sydney)>, Direct<(Sydney, Tokyo)>>,
                    Direct<(Tokyo, NewYork)>,
                >,
            >,
        >();
    }

    #[test]
    fn exercise_1_4() {
        const _: () = walk::assert::<
            Odense,
            Odense,
            Composite<Direct<(Odense, Rome)>, Direct<Symmetric<(Odense, Rome)>>>,
        >();
    }

    #[test]
    fn exercise_1_5() {
        trait Roundtrip: City {
            type Through<Stop: City>: Walk<From = Self, To = Self>
            where
                (Self, Stop): Connection<From = Self, To = Stop>;
        }

        impl<From: City> Roundtrip for From {
            type Through<Stop: City> = Composite<Direct<(From, Stop)>, Direct<Symmetric<(From, Stop)>>>
            where
                (From, Stop): Connection<From=From, To=Stop>
        ;
        }

        #[allow(dead_code)]
        const fn verify<A: City>()
        where
            A: Roundtrip,
        {
        }

        const _: () = verify::<NewYork>();
        const _: () = walk::assert::<NewYork, NewYork, <NewYork as Roundtrip>::Through<Rome>>();

        const _: () = verify::<Rome>();
        const _: () = walk::assert::<Rome, Rome, <Rome as Roundtrip>::Through<Sydney>>();

        const _: () = verify::<Tokyo>();
        const _: () = walk::assert::<Tokyo, Tokyo, <Tokyo as Roundtrip>::Through<NewYork>>();
    }
}
