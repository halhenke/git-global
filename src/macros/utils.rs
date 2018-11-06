// use std::collections::HashMap;
// use std::prelude::v1
// use std::hash::Hash;
// use std::iter::FromIterator;

#[macro_export]
macro_rules! debug_file {
    ($name: expr, $( $messages: expr ),+) => {
    // ($( $messages: expr ),+; $name: ident) => {
        let strs_join: String = vec!(
            $( $messages, )+
        ).as_slice().join("\n");
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open($name)
            .unwrap()
            .write_all(strs_join.as_ref());
    };
}
