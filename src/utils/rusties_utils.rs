use regex::Regex;


pub fn len_rusties(rustie: &str) -> usize {
    /*
     * Returns the number of symbols in a given rusties string.
     * paramater   rustie: a rustie string.
     * return: the symbol length of the rusties string.
    */ 

    return rustie.matches('[').count() + rustie.matches('.').count();
}


pub fn split_rusties(rustie: &str) -> Vec<&str> {
    // :param rustie: a rusties string.
    // :return: the symbols of the rusties string one-by-one with order preserved
    // needs lots of work
    
    let re = Regex::new(r"\[[\w]\]|\[.\w\]|\.").unwrap();
    //let references: Vec<&str> = strings.iter().collect();
    let mut references: Vec<&str> = Vec::new();
    for cap in re.captures_iter(rustie) {
        references.push(cap); 
}
    return references; 
}



#[test]
fn test_len_rusties() {
    assert_eq!(len_rusties("[C][=C][F].[C]"), 5);
    }
