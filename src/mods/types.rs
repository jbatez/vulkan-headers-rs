pub struct Registry {
    pub elems: Vec<RegistryElem>,
}

pub enum RegistryElem {
    Comment(String),
    Platforms(Platforms),
    Tags(Tags),
}

pub struct Platforms {
    pub comment: String,
    pub elems: Vec<PlatformsElem>,
}

pub enum PlatformsElem {
    Platform(Platform),
}

pub struct Platform {
    pub comment: String,
    pub name: String,
    pub protect: String,
}

pub struct Tags {
    pub comment: String,
    pub elems: Vec<TagsElem>,
}

pub enum TagsElem {
    Tag(Tag),
}

pub struct Tag {
    pub author: String,
    pub contact: String,
    pub name: String,
}
