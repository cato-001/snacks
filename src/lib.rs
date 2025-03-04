pub use {
    find_all::{find_all, find_all_into},
    recognize_separated::{recognize_separated0, recognize_separated1},
    simple::{alphanumdot0, alphanumdot1},
    take_all::{take_all, take_all_into},
    weblink::weblink,
};

mod find_all;
mod recognize_separated;
mod simple;
mod take_all;
mod weblink;
