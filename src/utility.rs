pub fn partition<A>(predicate: fn(&A) -> bool, xs: &[A]) -> (Vec<&A>, Vec<&A>) {
    let mut toepfchen = Vec::new();
    let mut kroepfchen = Vec::new();
    for x in xs {
        if predicate(&x) {
            toepfchen.push(x);
        } else {
            kroepfchen.push(x);
        }
    }
    (toepfchen, kroepfchen)
}
