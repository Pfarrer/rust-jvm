use model::class::{JvmClass, TypeSignature};

mod test_utils;

#[test]
fn validate_against_javap() {
    let class = test_utils::parse_class_in_testdata("ClassOnlyWithFields.class");

    // # javap -v ClassOnlyWithFields
    // public class ClassOnlyWithFields
    //   minor version: 0
    //   major version: 55
    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);

    //   flags: (0x0021) ACC_PUBLIC, ACC_SUPER
    assert_eq!(
        JvmClass::ACC_PUBLIC | JvmClass::ACC_SUPER,
        class.class_info.access_flags
    );

    //   this_class: #10                         // ClassOnlyWithFields
    assert_eq!("ClassOnlyWithFields", class.class_info.this_class);

    //   super_class: #11                        // java/lang/Object
    assert_eq!(Some("java/lang/Object".to_string()), class.class_info.super_class);

    //   interfaces: 0, fields: 5, methods: 2, attributes: 1
    assert_eq!(0, class.class_info.interfaces.len());
    assert_eq!(5, class.fields.len());
    assert_eq!(2, class.methods.len());
    assert_eq!(1, class.attributes.len());

    // Constant pool:
    //    #1 = Methodref          #11.#28        // java/lang/Object."<init>":()V
    //    #2 = Fieldref           #10.#29        // ClassOnlyWithFields.publicInt:I
    //    #3 = String             #30            // value
    //    #4 = Fieldref           #10.#31        // ClassOnlyWithFields.packageString:Ljava/lang/String;
    //    #5 = Double             3.14d
    //    #7 = Fieldref           #10.#32        // ClassOnlyWithFields.protectedDouble:D
    //    #8 = Fieldref           #10.#33        // ClassOnlyWithFields.privateBoolean:Z
    //    #9 = Fieldref           #10.#34        // ClassOnlyWithFields.staticBoolean:Z
    //   #10 = Class              #35            // ClassOnlyWithFields
    //   #11 = Class              #36            // java/lang/Object
    //   #12 = Utf8               staticBoolean
    //   #13 = Utf8               Z
    //   #14 = Utf8               publicInt
    //   #15 = Utf8               I
    //   #16 = Utf8               packageString
    //   #17 = Utf8               Ljava/lang/String;
    //   #18 = Utf8               protectedDouble
    //   #19 = Utf8               D
    //   #20 = Utf8               privateBoolean
    //   #21 = Utf8               <init>
    //   #22 = Utf8               ()V
    //   #23 = Utf8               Code
    //   #24 = Utf8               LineNumberTable
    //   #25 = Utf8               <clinit>
    //   #26 = Utf8               SourceFile
    //   #27 = Utf8               ClassOnlyWithFields.java
    //   #28 = NameAndType        #21:#22        // "<init>":()V
    //   #29 = NameAndType        #14:#15        // publicInt:I
    //   #30 = Utf8               value
    //   #31 = NameAndType        #16:#17        // packageString:Ljava/lang/String;
    //   #32 = NameAndType        #18:#19        // protectedDouble:D
    //   #33 = NameAndType        #20:#13        // privateBoolean:Z
    //   #34 = NameAndType        #12:#13        // staticBoolean:Z
    //   #35 = Utf8               ClassOnlyWithFields
    //   #36 = Utf8               java/lang/Object
    // {
    //   static boolean staticBoolean;
    //     descriptor: Z
    //     flags: (0x0008) ACC_STATIC
    assert_eq!(JvmClass::ACC_STATIC, class.fields[0].access_flags);
    assert_eq!(TypeSignature::Boolean, class.fields[0].descriptor);
    assert_eq!("staticBoolean", class.fields[0].name);
    assert_eq!(0, class.fields[0].attributes.len());

    //   public int publicInt;
    //     descriptor: I
    //     flags: (0x0001) ACC_PUBLIC
    assert_eq!(JvmClass::ACC_PUBLIC, class.fields[1].access_flags);
    assert_eq!(TypeSignature::Int, class.fields[1].descriptor);
    assert_eq!("publicInt", class.fields[1].name);
    assert_eq!(0, class.fields[1].attributes.len());

    //   java.lang.String packageString;
    //     descriptor: Ljava/lang/String;
    //     flags: (0x0000)
    assert_eq!(0x0, class.fields[2].access_flags);
    assert_eq!(
        TypeSignature::Class("java/lang/String".to_string()),
        class.fields[2].descriptor
    );
    assert_eq!("packageString", class.fields[2].name);
    assert_eq!(0, class.fields[2].attributes.len());

    //   protected double protectedDouble;
    //     descriptor: D
    //     flags: (0x0004) ACC_PROTECTED
    assert_eq!(JvmClass::ACC_PROTECTED, class.fields[3].access_flags);
    assert_eq!(TypeSignature::Double, class.fields[3].descriptor);
    assert_eq!("protectedDouble", class.fields[3].name);
    assert_eq!(0, class.fields[3].attributes.len());

    assert_eq!(JvmClass::ACC_PRIVATE, class.fields[4].access_flags);
    assert_eq!(TypeSignature::Boolean, class.fields[4].descriptor);
    assert_eq!("privateBoolean", class.fields[4].name);
    assert_eq!(0, class.fields[4].attributes.len());

    //   public ClassOnlyWithFields();
    //     descriptor: ()V
    //     flags: (0x0001) ACC_PUBLIC
    //     Code:
    //       stack=3, locals=1, args_size=1
    //          0: aload_0
    //          1: invokespecial #1                  // Method java/lang/Object."<init>":()V
    //          4: aload_0
    //          5: bipush        42
    //          7: putfield      #2                  // Field publicInt:I
    //         10: aload_0
    //         11: ldc           #3                  // String value
    //         13: putfield      #4                  // Field packageString:Ljava/lang/String;
    //         16: aload_0
    //         17: ldc2_w        #5                  // double 3.14d
    //         20: putfield      #7                  // Field protectedDouble:D
    //         23: aload_0
    //         24: iconst_0
    //         25: putfield      #8                  // Field privateBoolean:Z
    //         28: return
    //       LineNumberTable:
    //         line 1: 0
    //         line 4: 4
    //         line 5: 10
    //         line 6: 16
    //         line 7: 23

    //   static {};
    //     descriptor: ()V
    //     flags: (0x0008) ACC_STATIC
    //     Code:
    //       stack=1, locals=0, args_size=0
    //          0: iconst_1
    //          1: putstatic     #9                  // Field staticBoolean:Z
    //          4: return
    //       LineNumberTable:
    //         line 2: 0
    // }
    // SourceFile: "ClassOnlyWithFields.java"
}
