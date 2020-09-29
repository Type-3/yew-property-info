use yew::prelude::Properties;
use yew_property_info::{PropertyInfo};

#[derive(Debug, Clone, Properties, PropertyInfo)]
pub struct SimpleProps {
    pub required: bool,
    #[prop_or_default]
    pub field: String,
    #[prop_or(2)]
    #[prop_description("The Second Field")]
    pub field2: usize,
    #[prop_or("null")]
    pub reference: &'static str,
    #[prop_or("dynamic".to_string())]
    pub dynamic: String,

    #[prop_or_default]
    pub field3: Option<&'static str>
}

#[test]
fn run() {
    let props = SimpleProps::property_info();
    assert_eq!(props.ty, "SimpleProps");
    assert_eq!(props.module, "simple");
    let first = props.fields.first().unwrap();
    assert_eq!(first.required, true);
    assert_eq!(first.ty, "bool");
    assert_eq!(&first.name, &"required");

    let second = props.fields.get(1).unwrap();
    assert_eq!(second.required, false);
    assert_eq!(&second.name, &"field");
    assert_eq!(second.ty, "String");
    assert_eq!(second.default, Some("Default::default"));
    assert_eq!(second.description, None);

    let third = props.fields.get(2).unwrap();
    assert_eq!(third.required, false);
    assert_eq!(third.ty, "usize");
    assert_eq!(&third.name, &"field2");
    assert_eq!(third.default, Some("2"));
    assert_eq!(third.description, Some("The Second Field"));

    let forth = props.fields.get(3).unwrap();
    assert_eq!(forth.required, false);
    assert_eq!(&forth.name, &"reference");
    assert_eq!(forth.ty, "& 'static str");
    assert_eq!(forth.default, Some("\"null\""));

    let fifth = props.fields.get(4).unwrap();
    assert_eq!(fifth.required, false);
    assert_eq!(&fifth.name, &"dynamic");
    assert_eq!(fifth.ty, "String");
    assert_eq!(fifth.default, Some("\"dynamic\" . to_string()"));

    let sixth = props.fields.get(5).unwrap();
    assert_eq!(sixth.required, false);
    assert_eq!(&sixth.name, &"field3");
    assert_eq!(sixth.ty, "Option < & 'static str >");
    assert_eq!(sixth.default, Some("Default::default"));
}
