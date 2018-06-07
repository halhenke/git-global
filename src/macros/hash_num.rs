use std::collections::HashMap;
// use std::prelude::v1
// use std::hash::Hash;
use std::hash::{BuildHasher, Hash};
use std::cmp::Eq;
// use std::iter::FromIterator;

#[macro_export]
macro_rules! mkHash {
    ( $e:item, $( $s:item ),* ) => (
        let mut $e = HashMap::new()
        let mut i: i32 = 0;
        $($e.insert($s, i);
        i = i + 1;)*
        )
}

pub fn invert_map<K, V, S>(hmap: HashMap<K, V, S>) -> HashMap<V, K, S> where
    V: Hash + Eq,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    hmap.into_iter()
        .map(|(k,v)| (v,k) )
        // .collect::<HashMap<V, K, S>>()
        .collect()
}


// trait Invertible<K, V>
    // where V: Hash + Eq
//     // where V: std::cmp::Hash,
// {
//     type H;
//     fn invert(& self) -> Self::H;
//     // fn invert(& self) -> Self;
// }



// // impl<K, V> Invertible<K, V> for HashMap<K, V> {
// //     fn invert(&self) -> HashMap<V, K> {
// //     // fn invert(&self) -> HashMap<K, V> {
// //         unimplemented!();
// //     }
// // }

// impl<K, V> Invertible<K, V> for HashMap<K, V>
//     where   V: Hash + Eq,
//             K: Hash + Eq
// {
//     type H = HashMap<V, K>;

//     // fn invert(&self) -> HashMap<K, V> {
//     fn invert(&self) -> Self::H {
//         let mut h_new: Self::H = HashMap::new();
//         // let kvs = self.iter()
//         //     .collect::<Vec<(&K, &V)>>();

//         self.into_iter()
//             .map(||)

//         // let tlist: Vec<(&V, &K)> = self.into_iter()
//         //     .map(|(k, v)| (v, k))
//         //     .collect();
//         // let tlist = self.into_iter()
//         //     // .cloned()
//         //     // .map(|(k, v)| (v, k))
//         //     .collect();
//             // .iter();

//             // .for_each(move |(k, v)| println!("Hey"))
//             // .map(move |(k, v)| (k, v))
//             // .cloned()
//             // .for_each(
//             //     |(&k, &v)| {
//             //         h_new.insert(v, k).unwrap();
//             //         return ()
//             //         }
//             // );
//         // for (k, v) in self.iter() {
//         //     h_new.insert(v, k).unwrap();
//         // }
//         // HashMap::from_iter(tlist)
//         tlist
//     }
// }
