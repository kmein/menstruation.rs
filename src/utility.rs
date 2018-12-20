pub fn partition<A, P, I>(predicate: P, xs: I) -> (Vec<A>, Vec<A>)
where
    P: Fn(&A) -> bool,
    I: Iterator<Item = A>,
{
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
