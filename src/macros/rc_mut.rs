// use std::collections::HashMap;
// use std::prelude::v1
// use std::hash::Hash;
// use std::iter::FromIterator;

#[macro_export]
macro_rules! rc_borr {
    ($e: ident) => {
        // use std::cell::{RefCell, RefMut};
        // use std::rc::Rc;
        // use std::ops::{Deref, DerefMut};

        $e
            .deref()
            .borrow_mut()
    };
}

// macro_rules! mkHash {
//     ( $e:ident, $( $s:expr ),* ) => (

//         use std::collections::HashMap;
//         use std::hash::{BuildHasher, Hash};
//         use std::cmp::Eq;

//         let mut $e = HashMap::new();
//         let mut i: i32 = 0;
//         $($e.insert($s, i);
//         i = i + 1;)*

//         /// A trait that lets you invert a HashMap so Keys become Values & vice versa
//         trait Invertible<K, V, S> where
//             V: Hash + Eq,
//             K: Eq + Hash,
//             S: BuildHasher + Default,
//         {
//             type H;
//             fn invert(self) -> Self::H;
//         }

//         impl<K, V, S> Invertible<K, V, S> for HashMap<K, V, S> where
//             V: Hash + Eq,
//             K: Eq + Hash,
//             S: BuildHasher + Default,
//         {
//             type H = HashMap<V, K, S>;

//             // fn invert<K, V, S>(hmap: HashMap<K, V, S>) -> HashMap<V, K, S> where
//             fn invert(self) -> HashMap<V, K, S> where
//             V: Hash + Eq,
//             K: Eq + Hash,
//             S: BuildHasher + Default,
//             {
//                 self.into_iter()
//                     .map(|(k,v)| (v,k) )
//                     // .collect::<HashMap<V, K, S>>()
//                     .collect()
//             }
//         }
//     )
// }
