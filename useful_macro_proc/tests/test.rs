use useful_macro_proc::anon_enum;

type Test = anon_enum!{
  Blank,
  Name(String),
  Number(i32)
}

#[test]
fn anon_emum_test() {
  println!("{}", Test::Blank);
}
