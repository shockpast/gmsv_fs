mod api;

use gmodx::{gmod13_close, gmod13_open, lua};

#[gmod13_open]
fn gmod13_open(state: lua::State) {
    let fs = state.create_table();

    api::on_gmod_open(&state, &fs);

    state.set_global("fs", &fs)
        .expect("failed to set 'fs' table");
}

#[gmod13_close]
fn gmod13_close(state: lua::State) {}