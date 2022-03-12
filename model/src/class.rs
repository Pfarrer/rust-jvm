#[derive(Default, Clone, Debug)]
pub struct JvmClass {
    pub version: ClassVersion,
    pub constants: Vec<ClassConstant>,
    pub class_info: ClassInfo,
    pub fields: Vec<ClassField>,
    pub methods: Vec<ClassMethod>,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Default, Clone, Debug)]
pub struct ClassVersion {
    pub major: u16,
    pub minor: u16,
}

#[derive(Clone, Debug)]
pub enum ClassConstant {
    // This will be the first element of the constants pool for each class reader. This enables
    // easier handling of index parameters since Java class indexes are not 0 based.
    None(),

    // name_index
    Class(String),

    // class_name, field_name, type_descriptor
    Fieldref(String, String, String),

    // class_name, method_name, method_signature
    Methodref(String, String, String),

    // class_name, method_name, method_signature
    InterfaceMethodref(String, String, String),

    // class_index, name_and_type_index
    //    InterfaceMethodref(u16, u16),

    // string_index
    String(String),

    // Value
    Integer(i32),

    // Value
    Float(f32),

    // Value
    Long(i64),

    // Value
    Double(f64),

    // name, descriptor
    NameAndType(String, String),

    // Value
    Utf8(String),
    // reference_kind, reference_index
    //    MethodHandle(u8, u16),

    // descriptor_index
    //    MethodType(u16),

    // bootstrap_method_attr_index, name_and_type_index
    //    InvokeDynamic(u16, u16),
}

#[derive(Default, Clone, Debug)]
pub struct ClassInfo {
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
}

#[derive(Default, Clone, Debug)]
pub struct ClassField {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Default, Clone, Debug)]
pub struct ClassMethod {
    pub access_flags: u16,
    pub name_index: usize,
    pub descriptor_index: usize,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Clone, Debug)]
pub enum ClassAttribute {
    Code(CodeAttribute),
    LineNumberTable(Vec<SourceLineNumber>),
    SourceFile(u16),
    Exceptions(Vec<u16>),
    Signature(u16),
    ConstantValue(u16),
    Deprecated,
    NotImplemented,
}

#[derive(Default, Clone, Debug)]
pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTable>,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Default, Clone, Debug)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Default, Clone, Debug)]
pub struct SourceLineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}
