//! This crate includes more parser combinators to use with [`nom`].

pub use {
    find::{find_all, find_all_into, find_first},
    recognize_separated::{recognize_separated0, recognize_separated1},
    simple::{alphanumdot0, alphanumdot1},
    take_all::{take_all, take_all_into},
    weblink::{link_char, weblink},
};

mod find;
mod recognize_separated;
mod simple;
mod take_all;
mod weblink;
