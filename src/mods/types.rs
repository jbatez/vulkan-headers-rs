#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry {
    pub elems: Vec<RegistryElem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryElem {
    Comment(String),
    Platforms(Platforms),
    Tags(Tags),
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
