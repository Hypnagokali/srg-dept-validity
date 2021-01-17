use crate::init_validity::objects::{Dept, Prozedur, Fall};
use std::rc::Rc;
use chrono::{NaiveDateTime, NaiveDate, Duration};
use std::cell::RefCell;

mod init_validity;


fn create_sample_data_srg() -> Vec<Rc<Prozedur>> {
    vec![]
}

fn create_sample_data_fab(start_date: &NaiveDateTime, end_date: &NaiveDateTime) -> Vec<Rc<Dept>> {
    let fab1 = Dept {
        name: "2900".to_string(),
        aufnahme: Option::from(*start_date),
    };
    let fab2 = Dept {
        name: "0002".to_string(),
        aufnahme: Option::from(*start_date + Duration::days(4) + Duration::hours(14)),
    };

    let fab3 = Dept {
        name: "3000".to_string(),
        aufnahme: Option::from(*start_date + Duration::days(8) + Duration::hours(5)),
    };

    vec![Rc::new(fab1), Rc::new(fab2), Rc::new(fab3)]
}

fn create_fall(start_date: &NaiveDateTime, end_date: &NaiveDateTime) -> Fall {
    let dept = create_sample_data_fab(start_date, end_date);
    let srg = create_sample_data_srg();
    Fall {
        dept: RefCell::new(dept.clone()),
        srg: RefCell::new(srg),
        adt: Option::from(start_date.clone()),
        sdt: Option::from(end_date.clone()),
    }
}

fn main() {
    let start_date = NaiveDate::from_ymd(2020, 11,30).and_hms(0,0,0);
    let end_date = NaiveDate::from_ymd(2020, 12, 10).and_hms(23,59,0);
    let fall = create_fall(&start_date, &end_date);

    for dept in fall.dept.borrow().iter() {
        println!("{}", dept.name);
        println!("{}", dept.aufnahme.unwrap());
    }

    println!("App beendet!");
}
