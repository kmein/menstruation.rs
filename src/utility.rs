pub fn partition<A, P>(predicate: P, xs: &Vec<A>) -> (Vec<&A>, Vec<&A>)
where
    P: Fn(&A) -> bool,
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
