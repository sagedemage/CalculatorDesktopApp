/* Calculator implementations */

use gtk::Entry;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use crate::operator_symbols::*;

pub struct Values {
    pub num1: Rc<Cell<f64>>,
    pub num2: Rc<Cell<f64>>,
}

impl Values {
    pub fn new() -> Values {
        Values {
            num1: Rc::new(Cell::new(0.0)),
            num2: Rc::new(Cell::new(0.0)),
        }
    }
}

pub struct Operators {
    pub current: Rc<Cell<u8>>,
    pub previous: Rc<Cell<u8>>,
}

impl Operators {
    pub fn new() -> Operators {
        Operators {
            current: Rc::new(Cell::new(NONE)),
            previous: Rc::new(Cell::new(NONE)),
        }
    }
}

pub fn set_value(num_counter: i32, vals: &Rc<RefCell<Values>>, num: f64) {
    /* Set the first or second value */
    if num_counter == 0 {
        vals.borrow().num1.set(vals.borrow().num1.get() * 10.0 + num); 
    }
    if num_counter == 1 {
        vals.borrow().num2.set(vals.borrow().num2.get() * 10.0 + num); 
    }
}

pub fn calculation(operation: u8, vals: &Rc<RefCell<Values>>) {
    /* Calculate the 2 values */
    match operation {
        ADD => vals.borrow().num1.set(vals.borrow().num1.get() + vals.borrow().num2.get()),
        SUBTRACT => vals.borrow().num1.set(vals.borrow().num1.get() - vals.borrow().num2.get()),
        MULTIPLY => vals.borrow().num1.set(vals.borrow().num1.get() * vals.borrow().num2.get()),
        _=> ()
    }
    if operation == DIVIDE && vals.borrow().num2.get() != 0.0 {
        vals.borrow().num1.set(vals.borrow().num1.get() / vals.borrow().num2.get());
    }
}

pub fn check_divison_by_zero(ops: u8, val2: f64, divide_zero: &Rc<Cell<bool>>) {
    /* Set division by zero status */
    if ops == DIVIDE && val2 == 0.0 {
        divide_zero.set(true);
    }
}

pub fn operation(symbol_operator: u8, num_counter: &Rc<Cell<i32>>, ops: &Rc<RefCell<Operators>>,
                 vals: &Rc<RefCell<Values>>, divide_zero: &Rc<Cell<bool>>) {
    /* Do the operation when two values are received for calucaltion */ 
    if num_counter.get() == 2 {
        // Set previous and current operation
        ops.borrow().previous.set(ops.borrow().current.get());
        ops.borrow().current.set(symbol_operator);
        
        // Do calculation
        calculation(ops.borrow().previous.get(), vals);

        // Check divison by zero
        check_divison_by_zero(ops.borrow().previous.get(), vals.borrow().num2.get(), divide_zero);
        
        // reset variables
        num_counter.set(num_counter.get() - 1);
        vals.borrow().num2.set(0.0);
    }
    else {
        ops.borrow().current.set(symbol_operator);
    }
}

pub fn equation_result(ops: &Rc<RefCell<Operators>>, vals: &Rc<RefCell<Values>>, divide_zero: &Rc<Cell<bool>>) 
    -> std::string::String {
    let mut result = String::from("= ");

    // Check divison by zero
    check_divison_by_zero(ops.borrow().current.get(), vals.borrow().num2.get(), divide_zero);

    // Do calculation
    calculation(ops.borrow().current.get(), vals);

    if divide_zero.get() {
        // The result stores Divsion by Zero Error Message
        result =  String::from("Divide by 0 error");
    }
    else {
        // append the first value to the result
        // in other words, it just appends the result after the equals sign
        result.push_str(&vals.borrow().num1.get().to_string());
    }
    result
}

pub fn reset_variables(vals: &Rc<RefCell<Values>>, ops: &Rc<RefCell<Operators>>,
                       num_counter: &Rc<Cell<i32>>, divide_zero: &Rc<Cell<bool>>) {
    /* reset variables */
    vals.borrow().num1.set(0.0);
    vals.borrow().num2.set(0.0);
    ops.borrow().previous.set(NONE);
    ops.borrow().current.set(NONE);
    num_counter.set(0);
    divide_zero.set(false);
}

pub fn equality(num_counter: &Rc<Cell<i32>>, ops: &Rc<RefCell<Operators>>,
                 vals: &Rc<RefCell<Values>>, divide_zero: &Rc<Cell<bool>>,
                 entry: &Entry, initiate_equals: &Rc<Cell<bool>>) {
    if num_counter.get() == 2 {
        let result = equation_result(
            ops,
            vals,
            divide_zero,
        );

        // Show the result on the entry
        entry.set_text(&result);

        // Notify the progam the user initated the equals button
        initiate_equals.set(true);

        // reset variables
        reset_variables(vals, ops, num_counter, divide_zero);
    }
}

pub fn clear_entry_before_calculation(initiate_equals: &Rc<Cell<bool>>, entry: &Entry) {
    /* Clears the entry once the user clicks a number after getting the 
     * result of the calculation */
    if initiate_equals.get() {
        entry.set_text("");
        initiate_equals.set(false);
    }
}

