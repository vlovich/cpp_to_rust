extern crate qt_core;

use qt_core::vector::*;
use qt_core::cpp_utils::*;

#[test]
fn vector1() {
  let mut vec = VectorCInt::new(AsStruct);
  vec.append(&1);
  vec.append(&2);
  vec.append(&4);
  assert_eq!(vec.count(()), 3);
  assert_eq!(vec.at(0), &1);
  assert_eq!(vec.at(1), &2);
  assert_eq!(vec.at(2), &4);
}
