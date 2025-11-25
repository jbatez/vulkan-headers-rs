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
    Enums(Enums),
    Commands(Commands),
    Feature(Feature),
    Extensions(Extensions),
    Formats(Formats),
    SpirvExtensions(SpirvExtensions),
    SpirvCapabilities(SpirvCapabilities),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platforms {
    pub comment: Option<String>,
    pub contents: Vec<PlatformsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlatformsContent {
    Platform(Platform),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platform {
    pub comment: Option<String>,
    pub name: Option<String>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tags {
    pub comment: Option<String>,
    pub contents: Vec<TagsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagsContent {
    Tag(Tag),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag {
    pub author: Option<String>,
    pub contact: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Types {
    pub comment: Option<String>,
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
    Comment(String),
    Text(String),
    Type(String),
    Name(String),
    Member(Member),
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
    Comment(String),
    Text(String),
    Type(String),
    Name(String),
    Enum(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enums {
    pub bitwidth: Option<String>,
    pub comment: Option<String>,
    pub name: Option<String>,
    pub typ: Option<String>,
    pub contents: Vec<EnumsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumsContent {
    Comment(String),
    Enum(Enum),
    Unused(Unused),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    pub alias: Option<String>,
    pub api: Option<String>,
    pub bitpos: Option<String>,
    pub comment: Option<String>,
    pub deprecated: Option<String>,
    pub name: Option<String>,
    pub typ: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unused {
    pub comment: Option<String>,
    pub start: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Commands {
    pub comment: Option<String>,
    pub contents: Vec<CommandsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandsContent {
    Command(Command),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    pub alias: Option<String>,
    pub allownoqueues: Option<String>,
    pub api: Option<String>,
    pub cmdbufferlevel: Option<String>,
    pub comment: Option<String>,
    pub conditionalrendering: Option<String>,
    pub errorcodes: Option<String>,
    pub export: Option<String>,
    pub name: Option<String>,
    pub queues: Option<String>,
    pub renderpass: Option<String>,
    pub successcodes: Option<String>,
    pub tasks: Option<String>,
    pub videocoding: Option<String>,
    pub contents: Vec<CommandContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandContent {
    Proto(Proto),
    Param(Param),
    ImplicitExternSyncParams(ImplicitExternSyncParams),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proto {
    pub contents: Vec<ProtoContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProtoContent {
    Text(String),
    Type(String),
    Name(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Param {
    pub altlen: Option<String>,
    pub api: Option<String>,
    pub externsync: Option<String>,
    pub len: Option<String>,
    pub noautovalidity: Option<String>,
    pub objecttype: Option<String>,
    pub optional: Option<String>,
    pub stride: Option<String>,
    pub validstructs: Option<String>,
    pub contents: Vec<ParamContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParamContent {
    Text(String),
    Type(String),
    Name(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImplicitExternSyncParams {
    pub contents: Vec<ImplicitExternSyncParamsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImplicitExternSyncParamsContent {
    Param(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Feature {
    pub api: Option<String>,
    pub apitype: Option<String>,
    pub comment: Option<String>,
    pub depends: Option<String>,
    pub name: Option<String>,
    pub number: Option<String>,
    pub contents: Vec<FeatureContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureContent {
    Require(Require),
    Deprecate(Deprecate),
    Remove(Remove),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extensions {
    pub comment: Option<String>,
    pub contents: Vec<ExtensionsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionsContent {
    Extension(Extension),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    pub author: Option<String>,
    pub comment: Option<String>,
    pub contact: Option<String>,
    pub depends: Option<String>,
    pub deprecatedby: Option<String>,
    pub name: Option<String>,
    pub nofeatures: Option<String>,
    pub number: Option<String>,
    pub obsoletedby: Option<String>,
    pub platform: Option<String>,
    pub promotedto: Option<String>,
    pub provisional: Option<String>,
    pub ratified: Option<String>,
    pub sortorder: Option<String>,
    pub specialuse: Option<String>,
    pub supported: Option<String>,
    pub typ: Option<String>,
    pub contents: Vec<ExtensionContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionContent {
    Require(Require),
    Deprecate(Deprecate),
    Remove(Remove),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Require {
    pub api: Option<String>,
    pub comment: Option<String>,
    pub depends: Option<String>,
    pub contents: Vec<RequireContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequireContent {
    Comment(String),
    Type(GeneralRef),
    Enum(RequireEnum),
    Command(GeneralRef),
    Feature(FeatureRef),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequireEnum {
    pub alias: Option<String>,
    pub api: Option<String>,
    pub bitpos: Option<String>,
    pub comment: Option<String>,
    pub deprecated: Option<String>,
    pub dir: Option<String>,
    pub extends: Option<String>,
    pub extnumber: Option<String>,
    pub name: Option<String>,
    pub offset: Option<String>,
    pub protect: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deprecate {
    pub explanationlink: Option<String>,
    pub contents: Vec<DeprecateContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeprecateContent {
    Type(GeneralRef),
    Command(GeneralRef),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remove {
    pub comment: Option<String>,
    pub reasonlink: Option<String>,
    pub contents: Vec<RemoveContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemoveContent {
    Type(GeneralRef),
    Enum(GeneralRef),
    Command(GeneralRef),
    Feature(FeatureRef),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneralRef {
    pub comment: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeatureRef {
    pub name: Option<String>,
    pub struc: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Formats {
    pub contents: Vec<FormatsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatsContent {
    Format(Format),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Format {
    pub block_extent: Option<String>,
    pub block_size: Option<String>,
    pub chroma: Option<String>,
    pub class: Option<String>,
    pub compressed: Option<String>,
    pub name: Option<String>,
    pub packed: Option<String>,
    pub texels_per_block: Option<String>,
    pub contents: Vec<FormatContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatContent {
    Component(Component),
    Plane(Plane),
    SpirvImageFormat(SpirvImageFormat),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    pub bits: Option<String>,
    pub name: Option<String>,
    pub numeric_format: Option<String>,
    pub plane_index: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plane {
    pub compatible: Option<String>,
    pub height_divisor: Option<String>,
    pub index: Option<String>,
    pub width_divisor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvImageFormat {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvExtensions {
    pub comment: Option<String>,
    pub contents: Vec<SpirvExtensionsContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpirvExtensionsContent {
    SpirvExtension(SpirvExtension),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvExtension {
    pub name: Option<String>,
    pub contents: Vec<SpirvExtensionContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpirvExtensionContent {
    Enable(SpirvExtensionEnable),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvExtensionEnable {
    pub extension: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvCapabilities {
    pub comment: Option<String>,
    pub contents: Vec<SpirvCapabilitiesContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpirvCapabilitiesContent {
    SpirvCapability(SpirvCapability),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvCapability {
    pub name: Option<String>,
    pub contents: Vec<SpirvCapabilityContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpirvCapabilityContent {
    Enable(SpirvCapabilityEnable),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpirvCapabilityEnable {
    pub alias: Option<String>,
    pub extension: Option<String>,
    pub feature: Option<String>,
    pub member: Option<String>,
    pub property: Option<String>,
    pub requires: Option<String>,
    pub struc: Option<String>,
    pub value: Option<String>,
    pub version: Option<String>,
}
