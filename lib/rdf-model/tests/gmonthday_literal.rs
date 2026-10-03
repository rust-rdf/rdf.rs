#![cfg(feature = "alloc")]

use rdf_model::HeapTerm;

#[test]
fn equal_month_day_values_retain_distinct_rdf_lexical_identity() {
    let spellings = ["--02-29Z", "--02-29+00:00", "--02-29-00:00"];
    let terms: Vec<_> = spellings
        .iter()
        .map(|text| HeapTerm::typed_literal(*text, xsd::G_MONTH_DAY))
        .collect();
    assert_eq!(
        std::collections::HashSet::<_>::from_iter(terms.clone()).len(),
        3
    );
    assert_eq!(
        std::collections::BTreeSet::<_>::from_iter(terms.clone()).len(),
        3
    );
    for (lexical, term) in spellings.into_iter().zip(terms) {
        assert_eq!(
            xsd::parse_g_month_day(lexical).unwrap(),
            xsd::parse_g_month_day("--02-29Z").unwrap()
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
                    oxrdf::NamedNode::new("http://www.w3.org/2001/XMLSchema#gMonthDay").unwrap()
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
        xsd::parse_g_month_day("--02-29").unwrap(),
        xsd::parse_g_month_day("--02-29Z").unwrap()
    );
}
