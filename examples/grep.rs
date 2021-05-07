use sliceslice::x86::DynamicAvx2Searcher;

#[inline(never)]
pub fn search_in_slice(searcher: &DynamicAvx2Searcher<&[u8]>, haystack: &[u8]) -> bool {
    unsafe { searcher.inlined_search_in(haystack) }
}

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let needle = args.next().unwrap();
    let filename = args.next().unwrap();
    let contents = std::fs::read_to_string(&filename).expect("Could not open file");
    let searcher = unsafe { DynamicAvx2Searcher::new(needle.as_bytes()) };
    println!(
        "Searching for {} in {:?}: {}",
        needle,
        filename,
        search_in_slice(&searcher, contents.as_bytes())
    );
}
