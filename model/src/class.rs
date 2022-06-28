#[derive(Default, Clone, Debug)]
pub struct JvmClass {
    pub version: ClassVersion,
    pub constants: Vec<ClassConstant>,
    pub class_info: ClassInfo,
    pub fields: Vec<ClassField>,
    pub methods: Vec<ClassMethod>,
    pub attributes: Vec<ClassAttribute>,
}

impl JvmClass {
    pub const ACC_PUBLIC: u16 = 0x0001; // Declared public; may be accessed from outside its package.
//pub const ACC_PRIVATE: u16 = 0x0002; // Declared private; usable only within the defining class.
//pub const ACC_PROTECTED: u16 = 0x0004; // Declared protected; may be accessed within subclasses.
    pub const ACC_STATIC: u16 = 0x0008; // Declared static.
//    final = 0x0010, // Declared final; no subclasses allowed.
//    super = 0x0020, // Treat superclass methods specially when invoked by the invokespecial instruction.
    pub const ACC_INTERFACE: u16 = 0x0200; // Is an interface, not a class.
    pub const ACC_NATIVE: u16 = 0x0100; // Declared native; implemented in a language other than Java.
    pub const ACC_ABSTRACT: u16 = 0x0400; // Declared abstract; must not be instantiated.
//    ACC_SYNTHETIC = 0x1000, // Declared synthetic; not present in the source code.
//    ACC_ANNOTATION = 0x2000, //	Declared as an annotation type.
//    ACC_ENUM = 0x4000, // Declared as an enum type.
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
    Fieldref(String, String, TypeSignature),

    // class_name, method_name, method_signature
    Methodref(String, String, MethodSignature),

    // class_name, method_name, method_signature
    InterfaceMethodref(String, String, MethodSignature),

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
    NameAndType(String, TypeSignature),

    // Value
    Utf8(String),

    // reference_kind, reference_index
    // See https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-5.html#jvms-5.4.3.5
    MethodHandle(u8, u16),

    // descriptor_index
    MethodType(String), // TODO is this TypeSignature or MethodSignature?

    // bootstrap_method_attr_index, name_and_type_index
    InvokeDynamic(u16, u16),
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
    pub name: String,
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
    BootstrapMethods(Vec<BootstrapMethod>),
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

#[derive(Default, Clone, Debug)]
pub struct BootstrapMethod {
    pub method_ref: u16,
    pub arguments: Vec<u16>,
}

#[derive(Clone, Debug)]
pub enum TypeSignature {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Class(String),
    Array(Box<TypeSignature>),
}

#[derive(Clone, Debug)]
pub struct MethodSignature {
    pub parameters: Vec<TypeSignature>,
    pub return_type: TypeSignature,
}

impl std::fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TypeSignature::Void => "V".to_string(),
            TypeSignature::Boolean => "Z".to_string(),
            TypeSignature::Byte => "B".to_string(),
            TypeSignature::Char => "C".to_string(),
            TypeSignature::Short => "S".to_string(),
            TypeSignature::Int => "I".to_string(),
            TypeSignature::Long => "J".to_string(),
            TypeSignature::Float => "F".to_string(),
            TypeSignature::Double => "D".to_string(),
            TypeSignature::Class(class_path) => format!("{}{}", "L", class_path),
            TypeSignature::Array(inner_type) => format!("{}{}", "[", inner_type),
        };
        write!(f, "{}", text)
    }
}
