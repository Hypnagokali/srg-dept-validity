use chrono::{NaiveDateTime, NaiveDate};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

pub struct Prozedur {
    pub code: String,
    pub date: Option<NaiveDateTime>,
}

pub struct Dept {
    pub name: String,
    pub aufnahme: Option<NaiveDateTime>,
}

pub struct InvalidIntervall {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

pub struct FabMitGueltigkeit {
    pub dept: Rc<Dept>,
    pub is_pseudo: Cell<bool>,
    pub is_invalid: Cell<bool>,
    pub aufnahme_calc: Cell<Option<NaiveDateTime>>,
    pub entlass_calc: Cell<Option<NaiveDateTime>>,
    pub aufnahme: Option<NaiveDateTime>,
    pub entlass: Option<NaiveDateTime>,
}

pub struct ProzedurMitGueltigkeit {
    pub prozedur: Rc<Prozedur>,
    // belongs_to: Rc<Option<FabMitGueltigkeit>>,
    pub validity: Cell<i32>,
    pub days_validity_vector: RefCell<Vec<Cell<bool>>>,
    pub invalid_ranges: Vec<InvalidIntervall>,
    pub validity_set: String,
    pub treatment_type: String,
    pub validity_group: String,
}

pub struct Fall {
    pub dept: RefCell<Vec<Rc<Dept>>>,
    pub srg: RefCell<Vec<Rc<Prozedur>>>,
    pub adt: Option<NaiveDateTime>,
    pub sdt: Option<NaiveDateTime>,
}