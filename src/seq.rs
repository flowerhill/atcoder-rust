use permutohedron::LexicalPermutation;

pub fn is_subsequence_of<T: PartialEq>(subseq: &[T], seq: &[T]) -> bool {
    let mut subseq_iter = subseq.iter();
    let mut current_subseq_item = subseq_iter.next();

    for seq_item in seq {
        if let Some(subseq_item) = current_subseq_item {
            if seq_item == subseq_item {
                current_subseq_item = subseq_iter.next();
            }
        } else {
            break;
        }
    }
    current_subseq_item.is_none()
}

pub fn distinct_permutation(mut cs: Vec<char>) -> Vec<String> {
    cs.sort();
    let mut v = vec![];
    loop {
        v.push(cs.clone().into_iter().collect());
        if !cs.next_permutation() {
            break;
        }
    }
    v
}
