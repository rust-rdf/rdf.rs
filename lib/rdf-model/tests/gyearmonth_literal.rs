#![cfg(feature = "alloc")]

use rdf_model::HeapTerm;

#[test]
fn equal_year_month_values_retain_distinct_rdf_lexical_identity() {
    let spellings = ["0000-01Z", "0000-01+00:00", "0000-01-00:00", "-0000-01Z"];
    let terms: Vec<_> = spellings
        .iter()
        .map(|text| HeapTerm::typed_literal(*text, xsd::G_YEAR_MONTH))
        .collect();
    assert_eq!(
        std::collections::HashSet::<_>::from_iter(terms.clone()).len(),
        4
    );
    assert_eq!(
        std::collections::BTreeSet::<_>::from_iter(terms.clone()).len(),
        4
    );
    for (lexical, term) in spellings.into_iter().zip(terms) {
        assert_eq!(
            xsd::parse_g_year_month(lexical).unwrap(),
            xsd::parse_g_year_month("0000-01Z").unwrap()
        );
        assert_eq!(term.value_str(), lexical);
        assert_ne!(term, HeapTerm::typed_literal(lexical, xsd::STRING));
        #[cfg(feature = "oxrdf")]
        {
            let external = oxrdf::Term::from(term.clone());
            assert_eq!(
                external,
                oxrdf::Term::Literal(oxrdf::Literal::new_typed_literal(
                    lexical,
                    oxrdf::NamedNode::new("http://www.w3.org/2001/XMLSchema#gYearMonth").unwrap()
                ))
            );
            assert_eq!(HeapTerm::from(external), term);
        }
        #[cfg(feature = "serde")]
        assert_eq!(
            serde_json::from_value::<HeapTerm>(serde_json::to_value(&term).unwrap()).unwrap(),
            term
        );
    }
    assert_ne!(
        xsd::parse_g_year_month("0000-01").unwrap(),
        xsd::parse_g_year_month("0000-01Z").unwrap()
    );
}
