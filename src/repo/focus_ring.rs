use cursive::{
    event::{Event, EventResult, EventTrigger, Key},
    view::{View, ViewWrapper},
    views::{OnEventView, ViewRef},
    Cursive,
};
use ring_queue::Ring;
use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
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

// #[derive(PartialEq, Eq, Clone)]
// pub struct Foci<'a> {
//     ring: Ring<String>,
//     pt: PhantomData<&'a i32>, // ring: Ring<&'a str>,
// }

// trait Focal: Clone {}
// impl<'a> Focal for Foci<'a> {}

// impl<'a> From<Focal> for Foci<'a> {
//     fn from(f: Focal) -> Self {
//         Foci {
//             ring: f.ring,
//             pt: f.pt,
//         }
//     }
// }

// impl From

#[derive(PartialEq, Eq, Clone)]
pub struct Foci {
    ring: Rc<RefCell<Ring<String>>>,
}

// impl<'a> Foci<'a> {
impl<'a> Foci {
    // impl<'a> Foci<'a> {
    // pub const DEBUG_VIEW: String = String::from("debug-view");
    // pub const TEXT_VIEW: String = String::from("text-view");
    // pub const REPO_FIELD: String = String::from("repo-field");
    // pub const TAG_DISPLAY: String = String::from("tag-display");
    // pub const TAG_POOL: String = String::from("tag-pool");
    // pub const NEW_TAG: String = String::from("new-tag");

    // pub fn new<'b>(ring: Ring<String>) -> impl Focal + 'static {
    //     Foci {
    //         ring,
    //         pt: PhantomData,
    //     }
    // }
    // pub fn new(ring: Ring<&str>) -> Foci {
    //     Foci { ring }
    // }
    pub fn new(ring: Ring<String>) -> Foci {
        Foci {
            ring: Rc::new(RefCell::new(ring)),
        }
        // Foci { ring }
    }

    pub fn rcrf_ring<'b>(&self) -> Rc<RefCell<Ring<String>>> {
        // Rc::new(RefCell::new(self.ring))
        Rc::clone(&self.ring)
    }

    pub fn make_event_layer<T>(
        // &mut self,
        &self,
        // &self,
        // &mut self,
        s: &mut Cursive,
        // e: Event,
        mut e: Vec<Event>,
        // e: Key,
        // e: EventTrigger,
        // e: E,
        // from: impl View,
        from: T,
        // cb: M, // from: Box<S>,
        // ) -> ()
    ) -> OnEventView<T>
    where
        // S: View,
        // S: Box<dyn View>,
        T: ViewWrapper,
        // E: Into<EventTrigger>,
        // E: Key,
        // M: 'static + Fn(&mut Cursive),
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
        // let e2: Event = Event::from(e);
        // let e2: EventTrigger = EventTrigger::from_fn(|e| )
        // let e2 = EventTrigger::arrows();
        // let e3: Key = EventTrigger::inßto(e);
        let e1 = e.pop().unwrap();
        assert_eq!(e1, Event::Key(Key::Right));
        let e2 = e.pop().unwrap();
        assert_eq!(e2, Event::Key(Key::Left));
        // let sel1 = self.clone();
        // let sel2 = self.clone();
        OnEventView::new(from)
            // .on_event(e, f)
            // .on_event(e, move |s| self.focus_change(s, e))
            // .on_event(e, self.focus_change(s, Event::Key(e3)))
            // .on_event(e, self.focus_change(s, EventTrigger::Into(Key)(e)))
            // .on_event(e1.clone(), self.focus_change(s, e1))
            // .on_event(e2.clone(), self.focus_change(s, e2))
            .on_event(e1.clone(), self.focus_change(s, e1))
            .on_event(e2.clone(), self.focus_change(s, e2))
        // .on_event(e, cb)
    }

    pub fn focus_change(
        // pub fn focus_change<'b: 'a, T>(
        // self,
        &self,
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
        let s2: Rc<RefCell<Ring<String>>> = self.rcrf_ring();
        // let s2: Rc<RefCell<Ring<String>>> = Rc::new(RefCell::new(self.ring));
        let e2: Rc<Event> = Rc::new(e);
        // let e2: Rc<RefCell<Event>> = Rc::new(RefCell::new(e));
        Box::new(move |s: &mut Cursive| {
            // s;
            match *e2 {
                Event::Key(Key::Right) => {
                    s.focus_id((s2.borrow_mut()).rotate(1)[0].as_str())
                    // s.focus_id((*self).ring.rotate(1)[0].as_str())
                }
                Event::Key(Key::Left) => {
                    // s.focus_id((*self).ring.rotate(-1)[0].as_str())
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
