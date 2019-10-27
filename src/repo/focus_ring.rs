use cursive::{
    event::{Event, EventResult, EventTrigger, Key},
    view::{View, ViewWrapper},
    views::{OnEventView, ViewRef},
    Cursive,
};
use ring_queue::Ring;
use std::any::Any;

pub const DEBUG_VIEW: &str = "debug-view";
pub const TEXT_VIEW: &str = "text-view";
pub const REPO_FIELD: &str = "repo-field";
pub const TAG_DISPLAY: &str = "tag-display";
pub const TAG_POOL: &str = "tag-pool";
pub const NEW_TAG: &str = "new-tag";

pub struct Foci<'a> {
    ring: Ring<&'a str>,
}

impl<'a> Foci<'a> {
    pub fn new(ring: Ring<&str>) -> Foci {
        Foci { ring }
    }

    pub fn make_event_layer<S, T, E, M>(
        &mut self,
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
            .on_event(e, cb)
    }

    pub fn focus_change<'b>(&mut self, s: &mut Cursive, e: Event) -> () {
        match e {
            Event::Key(Key::Right) => s.focus_id((self.ring).rotate(1)[0]),
            Event::Key(Key::Left) => s.focus_id((self.ring).rotate(-1)[0]),
            _ => Ok(()),
        }
        .unwrap()
    }

    pub fn get_view<T>(&self, s: &mut Cursive, id: &str) -> ViewRef<T>
    where
        T: View + Any,
    {
        s.find_id(id).unwrap()
    }
}
