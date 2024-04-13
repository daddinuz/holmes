use crate::city::{City, NewYork, Odense, Rome, Sydney, Tokyo};

pub trait Connection {
    type From: City;
    type To: City;
}

pub const fn assert<From: City, To: City, T: Connection<From = From, To = To>>() {}

impl Connection for (NewYork, Rome) {
    type From = NewYork;
    type To = Rome;
}

impl Connection for (Odense, Rome) {
    type From = Odense;
    type To = Rome;
}

impl Connection for (Rome, Sydney) {
    type From = Rome;
    type To = Sydney;
}

impl Connection for (Sydney, Tokyo) {
    type From = Sydney;
    type To = Tokyo;
}

impl Connection for (Tokyo, NewYork) {
    type From = Tokyo;
    type To = NewYork;
}

pub struct Symmetric<T: Connection>(T);

impl<T: Connection> Connection for Symmetric<T> {
    type From = T::To;
    type To = T::From;
}
