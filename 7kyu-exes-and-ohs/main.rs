fn xo(string: &'static str) -> bool {
    let mut x = 0;
    let mut o = 0;

    for c in string.to_lowercase().chars() {
        if c == 'x' {
            x += 1;
        }
        if c == 'o' {
            o += 1;
        }
    }

    x == o
}

fn xo2(string: &'static str) -> bool {
    string.chars().fold(0, |a,c| {
        match c {
            'x' | 'X' => a + 1,
            'o' | 'O' => a - 1,
            _ => a
        }
    }) == 0
}

fn xo3(string: &'static str) -> bool {
    let lower = string.to_lowercase();
    lower.matches("x").count() == lower.matches("o").count()
}

#[test]
fn returns_expected() {
  assert_eq!(xo("xo"), true);
  assert_eq!(xo("Xo"), true);
  assert_eq!(xo("xxOo"), true);
  assert_eq!(xo("xxxm"), false);
  assert_eq!(xo("Oo"), false);
  assert_eq!(xo("ooom"), false);
}

#[test]
fn returns_expected_2() {
  assert_eq!(xo2("xo"), true);
  assert_eq!(xo2("Xo"), true);
  assert_eq!(xo2("xxOo"), true);
  assert_eq!(xo2("xxxm"), false);
  assert_eq!(xo2("Oo"), false);
  assert_eq!(xo2("ooom"), false);
}

#[test]
fn returns_expected_3() {
  assert_eq!(xo3("xo"), true);
  assert_eq!(xo3("Xo"), true);
  assert_eq!(xo3("xxOo"), true);
  assert_eq!(xo3("xxxm"), false);
  assert_eq!(xo3("Oo"), false);
  assert_eq!(xo3("ooom"), false);
}
