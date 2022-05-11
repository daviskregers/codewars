fn reverse_words(str: &str) -> String {
    str
        .split(' ')
        .map(|x| x.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn sample_test() {
  assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
  assert_eq!(reverse_words("apple"), "elppa");
  assert_eq!(reverse_words("a b c d"),"a b c d");
  assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}
