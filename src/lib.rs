pub use yew_property_info_derive::PropertyInfo;

#[derive(Debug, PartialEq, Clone)]
pub struct PropertyRef {
    pub name: &'static str,
    pub ty: &'static str,
    pub required: bool,
    pub default: Option<&'static str>,
    pub description: Option<&'static str>
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertiesInfo {
    pub ty: &'static str,
    pub module: &'static str,
    pub fields: &'static [PropertyRef]
}

pub trait HasPropertyInfo {
    fn property_info() -> PropertiesInfo;
}
