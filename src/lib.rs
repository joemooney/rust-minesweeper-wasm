mod log;
mod random;
mod minesweeper;

use minesweeper::*;
// use log::*;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(15, 15, 15));
}

// in javascript use camelCase not snake_case
#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().toggle_flag((x, y));
    });
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().open((x, y));
    });
}
