trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println! ("这是你们的机长在讲话。");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println! ("上升！");
    }
}

impl Human {
    fn fly(&self) {
        println! ("*疯狂地挥舞双臂*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
