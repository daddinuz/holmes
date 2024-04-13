use crate::city::City;
use crate::connection::Connection;

pub trait Walk {
    type From: City;
    type To: City;
}

pub const fn assert<From: City, To: City, T: Walk<From = From, To = To>>() {}

pub struct Direct<Route: Connection>(Route);

impl<Route: Connection> Walk for Direct<Route> {
    type From = Route::From;
    type To = Route::To;
}

pub struct Composite<Head: Walk, Tail: Walk<From = Head::To>>(Head, Tail);

impl<Head: Walk, Tail: Walk<From = Head::To>> Walk for Composite<Head, Tail> {
    type From = Head::From;
    type To = Tail::To;
}
