extern crate envsubst;

use envsubst::env_subst;

use std::{
    env,
    io::{
        stdin,
        stdout,
    },
    error::Error,
};

fn main() {
    env_subst(stdin(), stdout()).unwrap();
}

