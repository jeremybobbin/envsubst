extern crate envsubst;

use envsubst::env_subst;

use std::{
    io::{
        stdin,
        stdout,
    },
};

fn main() {
    env_subst(stdin(), stdout()).unwrap();
}

