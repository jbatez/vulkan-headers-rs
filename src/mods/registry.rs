#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registry {
    pub contents: Vec<RegistryContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryContent {
    Comment(String),
    Platforms(Platforms),
    Tags(Tags),
    Types(Types),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platforms {
    pub comment: String,
    pub contents: Vec<PlatformsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformsContent {
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
    pub contents: Vec<TagsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagsContent {
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
    pub contents: Vec<TypesContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypesContent {
    Comment(String),
    Type(Type),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type {
    pub alias: Option<String>,
    pub allowduplicate: Option<String>,
    pub api: Option<String>,
    pub bitvalues: Option<String>,
    pub category: Option<String>,
    pub comment: Option<String>,
    pub name: Option<String>,
    pub objtypeenum: Option<String>,
    pub parent: Option<String>,
    pub requiredlimittype: Option<String>,
    pub requires: Option<String>,
    pub returnedonly: Option<String>,
    pub structextends: Option<String>,
    pub contents: Vec<TypeContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeContent {
    Text(String),
    Type(String),
    Name(String),
    Member(Member),
    Comment(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub altlen: Option<String>,
    pub api: Option<String>,
    pub deprecated: Option<String>,
    pub externsync: Option<String>,
    pub featurelink: Option<String>,
    pub len: Option<String>,
    pub limittype: Option<String>,
    pub noautovalidity: Option<String>,
    pub objecttype: Option<String>,
    pub optional: Option<String>,
    pub selection: Option<String>,
    pub selector: Option<String>,
    pub values: Option<String>,
    pub contents: Vec<MemberContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemberContent {
    Text(String),
    Type(String),
    Name(String),
    Enum(String),
    Comment(String),
}
