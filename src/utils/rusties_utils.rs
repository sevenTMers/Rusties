
pub fn len_rusties(rustie: &str) -> usize {
    /*
     * Returns the number of symbols in a given rusties string.
     * paramater   rustie: a rustie string.
     * return: the symbol length of the rusties string.
    */ 

    return rustie.matches('[').count() + rustie.matches('.').count();
}

#[test]
fn test_len_rusties() {
    assert_eq!(len_rusties("[C][=C][F].[C]"), 5);
    }
