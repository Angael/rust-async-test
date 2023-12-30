use std::rc::Rc;

pub fn run() {
    println!("[RcBox]");
    let rc_variable = Rc::new(5);

    println!("Count: {}", Rc::strong_count(&rc_variable));

    let _a = Rc::clone(&rc_variable);
    let _b = Rc::clone(&rc_variable);
    println!("Count: {}", Rc::strong_count(&rc_variable));
    {
        let _rc_variable2 = Rc::clone(&rc_variable);
        println!("Count: {}", Rc::strong_count(&rc_variable));
    }

    println!("Count: {}", Rc::strong_count(&rc_variable));
}
