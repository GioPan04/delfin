use serde::Serializer;

pub fn round_one_place<S: Serializer>(val: &f64, s: S) -> Result<S::Ok, S::Error> {
    let rounded = format!("{val:.1}").parse().unwrap();
    s.serialize_f64(rounded)
}
