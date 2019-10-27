use cursive::{
    event::{Event, EventResult, EventTrigger, Key},
    view::{View, ViewWrapper},
    views::{OnEventView, ViewRef},
    Cursive,
};
use ring_queue::Ring;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// pub DEBUG_VIEW: String = String::from("debug-view");
// pub TEXT_VIEW: String = String::from("text-view");
// pub REPO_FIELD: String = String::from("repo-field");
// pub TAG_DISPLAY: String = String::from("tag-display");
// pub TAG_POOL: String = String::from("tag-pool");
// pub NEW_TAG: String = String::from("new-tag");
// pub const DEBUG_VIEW: &str = "debug-view";
// pub const TEXT_VIEW: &str = "text-view";
// pub const REPO_FIELD: &str = "repo-field";
// pub const TAG_DISPLAY: &str = "tag-display";
// pub const TAG_POOL: &str = "tag-pool";
// pub const NEW_TAG: &str = "new-tag";

pub struct Foci {
    ring: Ring<String>,
    // ring: Ring<&'a str>,
}

impl Foci {
    // impl<'a> Foci<'a> {
    // pub const DEBUG_VIEW: String = String::from("debug-view");
    // pub const TEXT_VIEW: String = String::from("text-view");
    // pub const REPO_FIELD: String = String::from("repo-field");
    // pub const TAG_DISPLAY: String = String::from("tag-display");
    // pub const TAG_POOL: String = String::from("tag-pool");
    // pub const NEW_TAG: String = String::from("new-tag");

    pub fn new(ring: Ring<String>) -> Foci {
        Foci { ring }
    }
    // pub fn new(ring: Ring<&str>) -> Foci {
    //     Foci { ring }
    // }

    pub fn make_event_layer<S, T, E, M>(
        self,
        // &mut self,
        s: &mut Cursive,
        e: Event,
        // from: impl View,
        from: T,
        cb: M, // from: Box<S>,
               // ) -> ()
    ) -> OnEventView<T>
    where
        S: View,
        // S: Box<dyn View>,
        T: ViewWrapper,
        E: Into<EventTrigger>,
        M: 'static + Fn(&mut Cursive),
    {
        //     // let f: dyn (Fn(OnEventView<_>) -> OnEventView<_>) =
        //     // trait FF = Fn(&'static mut Cursive);
        //     // type FFF = Box<(dyn FF) + 'static>;
        //     // trait F = 'static + Fn(&mut Cursive);
        //     // type H = dyn |s| {self.focus_change(s, e)};
        //     trait G: Fn(&mut Cursive) {}
        //     // impl G for
        //     type F = dyn 'static + Fn(&mut Cursive);
        //     // type F = dyn 'static + G;
        //     // type F = dyn 'static + G;
        //     // let f: Box<G> = Box::new(|s| self.focus_change(s, e));
        //     // let f: FF = Box::new(|s| self.focus_change(s, e));
        //     static ff: Box<Fn(&mut Cursive)> =
        //         Box::new(|s| self.focus_change(s, e));
        //     // let f: Box<F> = Box::new(|s| self.focus_change(s, e));
        OnEventView::new(from)
            // .on_event(e, f)
            // .on_event(e, move |s| self.focus_change(s, e))
            .on_event(e.clone(), self.focus_change(s, e))
        // .on_event(e, cb)
    }

    pub fn focus_change(
        // pub fn focus_change<'b: 'a, T>(
        self,
        // &mut self,
        s: &mut Cursive,
        e: Event,
    ) -> Box<dyn Fn(&mut Cursive) + 'static>
// ) -> Box<dyn Fn(&'b mut Cursive) + 'static>
// ) -> Box<(dyn for<'r> Fn(&'r mut cursive::Cursive) + 'static)>
// ) -> Box<T>
    // where
        // T: dyn<Fn(&mut Cursive) + 'static>,
    {
        // let s2 = Rc::new(RefCell::new(self.ring));
        // let s2: Rc<RefCell<Ring<&str>>> = Rc::new(RefCell::new(self.ring));
        let s2: Rc<RefCell<Ring<String>>> = Rc::new(RefCell::new(self.ring));
        let e2: Rc<Event> = Rc::new(e);
        // let e2: Rc<RefCell<Event>> = Rc::new(RefCell::new(e));
        Box::new(move |s: &mut Cursive| {
            // s;
            match *e2 {
                Event::Key(Key::Right) => {
                    s.focus_id((s2.borrow_mut()).rotate(1)[0].as_str())
                }
                Event::Key(Key::Left) => {
                    s.focus_id((s2.borrow_mut()).rotate(-1)[0].as_str())
                }
                _ => Ok(()),
            }
            .unwrap();
        })
    }

    // pub fn focus_change<'b>(&mut self, s: &mut Cursive, e: Event) -> () {
    //     match e {
    //         Event::Key(Key::Right) => s.focus_id((self.ring).rotate(1)[0]),
    //         Event::Key(Key::Left) => s.focus_id((self.ring).rotate(-1)[0]),
    //         _ => Ok(()),
    //     }
    //     .unwrap()
    // }

    pub fn get_view<T>(&self, s: &mut Cursive, id: &str) -> ViewRef<T>
    where
        T: View + Any,
    {
        s.find_id(id).unwrap()
    }
}
