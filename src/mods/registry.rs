#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry {
    pub elems: Vec<RegistryElem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryElem {
    Comment(String),
    Platforms(Platforms),
    Tags(Tags),
    Types(Types),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platforms {
    pub comment: String,
    pub elems: Vec<PlatformsElem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformsElem {
    Platform(Platform),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platform {
    pub comment: String,
    pub name: String,
    pub protect: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tags {
    pub comment: String,
    pub elems: Vec<TagsElem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagsElem {
    Tag(Tag),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag {
    pub author: String,
    pub contact: String,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Types {
    pub comment: String,
    pub elems: Vec<TypesElem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypesElem {
    Comment(String),
    Type(Type),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type {
    pub alias: Option<String>,
    pub api: Option<String>,
    pub bitvalues: Option<String>,
    pub category: Option<String>,
    pub comment: Option<String>,
    pub name: Option<String>,
    pub objtypeenum: Option<String>,
    pub parent: Option<String>,
    pub requires: Option<String>,
    pub contents: Vec<TypeContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeContent {
    Text(String),
    Name(String),
    Type(String),
}
