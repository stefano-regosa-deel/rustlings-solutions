// structs1.rs
// Address all the TODOs to make the tests pass!


struct ColorClassicStruct {
  name: String,
  hex: String,
}

struct ColorTupleStruct<A, B>(A, B);

#[derive(Debug)]
struct UnitStruct<A>(A);

#[cfg(test)]

mod tests {
  use super::ColorClassicStruct;
  use super::ColorTupleStruct;
  use super::UnitStruct;

  #[test]
  fn classic_c_structs() {
    let green = ColorClassicStruct {
      name: String::from("green"),
      hex: String::from("#00FF00"),
    };
    assert_eq!(green.name, "green");
    assert_eq!(green.hex, "#00FF00");
  }

  #[test]
  fn tuple_structs() {
    // TODO: Instantiate a tuple struct!
    let green = ColorTupleStruct("green", "#00FF00");
    assert_eq!(green.0, "green");
    assert_eq!(green.1, "#00FF00");
  }

  #[test]
  fn unit_structs() {
    let unit_struct  = UnitStruct("UnitStruct").0;
    let message = format!("{}s are fun!", unit_struct);

    assert_eq!(message, "UnitStructs are fun!");
  }
}
