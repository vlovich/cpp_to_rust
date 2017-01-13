// ------------------------------
// from cpp_data

/// One item of a C++ enum declaration
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CppEnumValue {
  /// Identifier
  pub name: String,
  /// Corresponding value
  pub value: i64,
  /// C++ documentation for this item in HTML
  pub doc: Option<String>,
}

/// Member field of a C++ class declaration
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppClassField {
  /// Identifier
  pub name: String,
  /// Field type
  pub field_type: CppType,
  /// Visibility
  pub visibility: CppVisibility,
  /// Size of type in bytes
  pub size: Option<i32>,
}

/// A "using" directive inside a class definition,
/// indicating that the class should inherite a
/// certain method of a base class.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppClassUsingDirective {
  /// Name of the base class
  pub class_name: String,
  /// Name of the method
  pub method_name: String,
}

/// Item of base class list in a class declaration
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppBaseSpecifier {
  /// Base class type (can include template arguments)
  pub base_type: CppType,
  /// True if this base is virtual
  pub is_virtual: bool,
  /// Base visibility (public, protected or private)
  pub visibility: CppVisibility,
}


/// Information about a C++ type declaration
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum CppTypeKind {
  /// Enum declaration
  Enum {
    /// List of items
    values: Vec<CppEnumValue>,
  },
  /// Class declaration
  Class {
    /// List of class types this class is derived from
    bases: Vec<CppBaseSpecifier>,
    /// List of class fields
    fields: Vec<CppClassField>,
    /// Information about template arguments of this type.
    template_arguments: Option<TemplateArgumentsDeclaration>,
    /// List of using directives, like "using BaseClass::method1;"
    using_directives: Vec<CppClassUsingDirective>,
  },
}

/// Location of a C++ type's definition in header files.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppOriginLocation {
  // Full path to the include file
  pub include_file_path: String,
  /// Line of the file
  pub line: u32,
  /// Column of the file
  pub column: u32,
}

/// Visibility of a C++ entity. Defaults to `Public`
/// for entities that can't have visibility (like free functions)
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppVisibility {
  Public,
  Protected,
  Private,
}

/// C++ documentation for a type
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CppTypeDoc {
  /// HTML content
  pub html: String,
  /// Absolute URL to online documentation page for this type
  pub url: String,
  /// Absolute documentation URLs encountered in the content
  pub cross_references: Vec<String>,
}

/// Information about a C++ type declaration
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CppTypeData {
  /// Identifier, including namespaces and nested classes
  /// (separated with "::", like in C++)
  pub name: String,
  /// File name of the include file (without full path)
  pub include_file: String,
  /// Exact location of the declaration
  pub origin_location: CppOriginLocation,
  /// Type information
  pub kind: CppTypeKind,
  /// C++ documentation data for this type
  pub doc: Option<CppTypeDoc>,
}

/// Information about template arguments of a C++ class type
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct TemplateArgumentsDeclaration {
  /// Indicates how many template types this type is nested into.
  ///
  /// In the following example class `A`
  /// has level 0, and class `B` has level 1.
  ///
  /// ```C++
  /// template<class T>
  /// class A {
  ///   template<class T2>
  ///   class B {};
  /// };
  /// ```
  pub nested_level: i32,
  /// Names of template arguments. Names themselves are
  /// not particularly important, but their count is.
  pub names: Vec<String>,
}

/// Information about a C++ template class
/// instantiation.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct CppTemplateInstantiation {
  /// List of template arguments used in this instantiation
  pub template_arguments: Vec<CppType>,
}

/// List of template instantiations of
/// a template class.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct CppTemplateInstantiations {
  /// Template class name
  pub class_name: String,
  /// List of encountered instantiations
  pub instantiations: Vec<CppTemplateInstantiation>,
}

/// C++ parser output
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct CppData {
  /// List of found type declarations
  pub types: Vec<CppTypeData>,
  /// List of found methods
  pub methods: Vec<CppMethod>,
  /// List of found template instantiations. Key is name of
  /// the template class, value is list of instantiations.
  pub template_instantiations: Vec<CppTemplateInstantiations>,
  /// List of all argument types used by signals,
  /// including variations with omitted arguments,
  /// but excluding argument types from dependencies.
  pub signal_argument_types: Vec<Vec<CppType>>,
  /// Data of dependencies
  pub dependencies: Vec<CppData>,
}

// -----------------------------------
// from cpp_method

/// Information about an argument of a C++ method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppFunctionArgument {
  /// Identifier. If the argument doesn't have a name
  /// (which is allowed in C++), this field contains
  /// generated name "argX" (X is position of the argument).
  pub name: String,
  /// Argument type
  pub argument_type: CppType,
  /// Flag indicating that the argument has default value and
  /// therefore can be omitted when calling the method
  pub has_default_value: bool,
}

/// Enumerator indicating special cases of C++ methods.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppMethodKind {
  /// Just a class method
  Regular,
  /// Constructor
  Constructor,
  /// Destructor
  Destructor,
}

/// Variation of a field accessor method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppFieldAccessorType {
  /// Returns copy of the field
  CopyGetter,
  /// Returns const reference to the field
  ConstRefGetter,
  /// Returns mutable reference to the field
  MutRefGetter,
  /// Copies value from its argument to the field
  Setter,
}

/// Information about automatically generated method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum FakeCppMethod {
  /// Method for accessing a public field of a class
  FieldAccessor {
    accessor_type: CppFieldAccessorType,
    field_name: String,
  },
}


/// for accessing a public field of a class
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppFieldAccessor {
  /// Type of the accessor
  pub accessor_type: CppFieldAccessorType,
  /// Name of the C++ field
  pub field_name: String,
}

/// Information about a C++ class member method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppMethodClassMembership {
  /// Type of the class where this method belong. This is used to construct
  /// type of "this" pointer and return type of constructors.
  pub class_type: CppTypeClassBase,
  /// Whether this method is a constructor, a destructor or an operator
  pub kind: CppMethodKind,
  /// True if this is a virtual method
  pub is_virtual: bool,
  /// True if this is a pure virtual method (requires is_virtual = true)
  pub is_pure_virtual: bool,
  /// True if this is a const method, i.e. "this" pointer receives by
  /// this method has const type
  pub is_const: bool,
  /// True if this is a static method, i.e. it doesn't receive "this" pointer at all.
  pub is_static: bool,
  /// Method visibility
  pub visibility: CppVisibility,
  /// True if the method is a Qt signal
  pub is_signal: bool,
  /// True if the method is a Qt slot
  pub is_slot: bool,
  /// If this method is a generated field accessor, this field contains
  /// information about it. Field accessors do not have real C++ methods corresponding to them.
  pub fake: Option<FakeCppMethod>,
}

/// C++ documentation for a method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppMethodDoc {
  /// HTML anchor of this documentation entry
  /// (used to detect duplicates)
  pub anchor: String,
  /// HTML content
  pub html: String,
  /// If the documentation parser couldn't find documentation for the exact same
  /// method, it can still provide documentation entry for the closest match.
  /// In this case, this field should contain C++ declaration of the found method.
  pub mismatched_declaration: Option<String>,
  /// Absolute URL to online documentation page for this method
  pub url: String,
  /// Absolute documentation URLs encountered in the content
  pub cross_references: Vec<String>,
}

/// Information about a C++ method
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppMethod {
  /// Identifier. For class methods, this field includes
  /// only the method's own name. For free functions,
  /// this field also includes namespaces (if any).
  pub name: String,
  /// Additional information about a class member function
  /// or None for free functions
  pub class_membership: Option<CppMethodClassMembership>,
  /// If the method is a C++ operator, indicates its kind
  pub operator: Option<CppOperator>,
  /// Return type of the method.
  /// Return type is reported as void for constructors and destructors.
  pub return_type: CppType,
  /// List of the method's arguments
  pub arguments: Vec<CppFunctionArgument>,
  /// If Some, the method is derived from another method by omitting arguments,
  /// and this field contains all arguments of the original method.
  pub arguments_before_omitting: Option<Vec<CppFunctionArgument>>,
  /// Whether the argument list is terminated with "..."
  pub allows_variadic_arguments: bool,
  /// File name of the include file where the method is defined
  /// (without full path)
  pub include_file: String,
  /// Exact location of declaration of the method.
  /// Can be None if the method is generated automatically
  /// and doesn't have corresponding C++ declaration.
  pub origin_location: Option<CppOriginLocation>,
  /// Names of the method's template arguments.
  /// None if this is not a template method.
  /// If the method belongs to a template class,
  /// the class's template arguments are not included here.
  pub template_arguments: Option<TemplateArgumentsDeclaration>,
  /// For an instantiated template method, this field contains the types
  /// used for instantiation. For example, `T QObject::findChild<T>()` would have
  /// no `template_arguments_values` because it's not instantiated, and
  /// `QWidget* QObject::findChild<QWidget*>()` would have `QWidget*` type in
  /// `template_arguments_values`.
  pub template_arguments_values: Option<Vec<CppType>>,
  /// C++ code of the method's declaration.
  /// None if the method was not explicitly declared.
  pub declaration_code: Option<String>,
  /// List of base classes this method was inferited from.
  /// The first item is the most base class.
  pub inheritance_chain: Vec<CppBaseSpecifier>, /* TODO: fill inheritance_chain for explicitly redeclared methods (#23) */
  /// C++ documentation data for this method
  pub doc: Option<CppMethodDoc>,
  /// If true, FFI generator skips some checks
  pub is_ffi_whitelisted: bool,
  // If true, this is an unsafe (from base to derived) static_cast wrapper.
  pub is_unsafe_static_cast: bool,
}

// ------------------------------
// from cpp_operators

/// Available types of C++ operators
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppOperator {
  /// (type) a
  Conversion(CppType),
  /// a = b
  Assignment,
  /// a + b
  Addition,
  /// a - b
  Subtraction,
  /// +a
  UnaryPlus,
  /// -a
  UnaryMinus,
  /// a * b
  Multiplication,
  /// a / b
  Division,
  /// a % b
  Modulo,
  /// ++a
  PrefixIncrement,
  /// a++
  PostfixIncrement,
  /// --a
  PrefixDecrement,
  /// a--
  PostfixDecrement,
  /// a == b
  EqualTo,
  /// a != b
  NotEqualTo,
  /// a > b
  GreaterThan,
  /// a < b
  LessThan,
  /// a >= b
  GreaterThanOrEqualTo,
  /// a <= b
  LessThanOrEqualTo,
  /// !a
  LogicalNot,
  /// a && b
  LogicalAnd,
  /// a || b
  LogicalOr,
  /// ~a
  BitwiseNot,
  /// a & b
  BitwiseAnd,
  /// a | b
  BitwiseOr,
  /// a ^ b
  BitwiseXor,
  /// a << b
  BitwiseLeftShift,
  /// a >> b
  BitwiseRightShift,

  /// a += b
  AdditionAssignment,
  /// a -= b
  SubtractionAssignment,
  /// a *= b
  MultiplicationAssignment,
  /// a /= b
  DivisionAssignment,
  /// a %= b
  ModuloAssignment,
  /// a &= b
  BitwiseAndAssignment,
  /// a |= b
  BitwiseOrAssignment,
  /// a ^= b
  BitwiseXorAssignment,
  /// a <<= b
  BitwiseLeftShiftAssignment,
  /// a >>= b
  BitwiseRightShiftAssignment,
  /// a[b]
  Subscript,
  /// *a
  Indirection,
  /// &a
  AddressOf,
  /// a->b
  StructureDereference,
  /// a->*b
  PointerToMember,
  /// a(a1, a2)
  FunctionCall,
  /// a, b
  Comma,
  /// new type
  New,
  /// new type[n]
  NewArray,
  /// delete a
  Delete,
  /// delete[] a
  DeleteArray,
}

// -------------------------------
// from cpp_type

/// C++ type variants based on indirection
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub enum CppTypeIndirection {
  /// No indirection
  None,
  /// Pointer, like int*
  Ptr,
  /// Reference, like int&
  Ref,
  /// Reference to pointer, like int*&
  PtrRef,
  /// Pointer to pointer, like int**
  PtrPtr,
  /// R-value reference, like Class&&
  RValueRef,
}

/// Available built-in C++ numeric types.
/// All these types have corresponding
/// `clang::TypeKind` values (except for `CharS` and `CharU`
/// which map to `CppBuiltInNumericType::Char`)
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppBuiltInNumericType {
  Bool,
  Char,
  SChar,
  UChar,
  WChar,
  Char16,
  Char32,
  Short,
  UShort,
  Int,
  UInt,
  Long,
  ULong,
  LongLong,
  ULongLong,
  Int128,
  UInt128,
  Float,
  Double,
  LongDouble,
}

/// Information about a fixed-size primitive type
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub enum CppSpecificNumericTypeKind {
  Integer { is_signed: bool },
  FloatingPoint,
}

/// Information about base C++ class type
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppTypeClassBase {
  /// Name, including namespaces and nested classes
  pub name: String,
  /// For template classes, C++ types used as template
  /// arguments in this type,
  /// like [QString, int] in QHash<QString, int>
  pub template_arguments: Option<Vec<CppType>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppFunctionPointerType {
  /// Return type of the function
  pub return_type: Box<CppType>,
  /// Arguments of the function
  pub arguments: Vec<CppType>,
  /// Whether arguments are terminated with "..."
  pub allows_variadic_arguments: bool,
}

/// Base C++ type. `CppType` can add indirection
/// and constness to `CppTypeBase`, but otherwise
/// this enum lists all supported types.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CppTypeBase {
  /// Void
  Void,
  /// Built-in C++ primitive type, like int
  BuiltInNumeric(CppBuiltInNumericType),
  /// Fixed-size primitive type, like qint64 or int64_t
  /// (may be translated to Rust's i64)
  SpecificNumeric {
    /// Type identifier (most likely a typedef name)
    name: String,
    /// Size of type in bits
    bits: i32,
    /// Information about the type (float or integer,
    /// signed or unsigned)
    kind: CppSpecificNumericTypeKind,
  },
  /// Pointer sized integer, like qintptr
  /// (may be translated to Rust's isize)
  PointerSizedInteger { name: String, is_signed: bool },
  /// Enum type
  Enum {
    /// Name, including namespaces and nested classes
    name: String,
  },
  /// Class type
  Class(CppTypeClassBase),
  /// Template parameter, like "T" anywhere inside
  /// QVector<T> declaration
  TemplateParameter {
    /// Template instantiation level. For example,
    /// if there is a template class and a template method in it,
    /// the class's template parameters will have level = 0 and
    /// the method's template parameters will have level = 1.
    /// If only the class or only the method is a template,
    /// the level will be 0.
    nested_level: i32,
    /// Index of the parameter. In QHash<K, V> "K" has index = 0
    /// and "V" has index = 1.
    index: i32,
  },
  /// Function pointer type
  FunctionPointer(CppFunctionPointerType),
}

/// Information about a C++ type
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[derive(Serialize, Deserialize)]
pub struct CppType {
  /// Information about base type
  pub base: CppTypeBase,
  /// Indirection applied to base type
  pub indirection: CppTypeIndirection,
  /// If the type has const qualifier. Defaults to false
  /// when not applicable.
  pub is_const: bool,
  /// If 2nd indirection part of the type is const, e.g.
  /// true for "int* const*".
  pub is_const2: bool,
}

// -------------------------
// from rust_type

/// Rust identifier. Represented by
/// a vector of name parts. For a regular name,
/// first part is name of the crate,
/// last part is own name of the entity,
/// and intermediate names are module names.
/// Built-in types are represented
/// by a single vector item, like `vec!["i32"]`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct RustName {
  /// Parts of the name
  pub parts: Vec<String>,
}


// -------------------------
// from rust_info

/// C++ documentation data for a enum variant
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct CppEnumValueDocItem {
  /// C++ name of the variant
  pub variant_name: String,
  /// HTML content
  pub doc: Option<String>,
}

/// One variant of a Rust enum
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct RustEnumValue {
  /// Identifier
  pub name: String,
  /// Corresponding value
  pub value: i64,
  /// Documentation of corresponding C++ variants
  pub cpp_docs: Vec<CppEnumValueDocItem>,
  /// True if this variant was added because enums with
  /// one variant are not supported
  pub is_dummy: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub enum RustTypeIndirection {
  None,
  Ptr,
  Ref { lifetime: Option<String> },
  PtrPtr,
  PtrRef { lifetime: Option<String> },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub enum RustType {
  Void,
  Common {
    base: RustName,
    generic_arguments: Option<Vec<RustType>>,
    is_const: bool,
    is_const2: bool,
    indirection: RustTypeIndirection,
  },
  FunctionPointer {
    return_type: Box<RustType>,
    arguments: Vec<RustType>,
  },
}

/// Relation between original C++ method's argument value
/// and corresponding FFI function's argument value
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum IndirectionChange {
  /// Argument types are identical
  NoChange,
  /// C++ argument is a class value (like QPoint)
  /// and FFI argument is a pointer (like QPoint*)
  ValueToPointer,
  /// C++ argument is a reference (like QPoint&)
  /// and FFI argument is a pointer (like QPoint*)
  ReferenceToPointer,
  /// C++ argument is QFlags<T>
  /// and FFI argument is uint
  QFlagsToUInt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub enum RustToCTypeConversion {
  None,
  RefToPtr,
  OptionRefToPtr,
  ValueToPtr,
  CppBoxToPtr,
  QFlagsToUInt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct CompleteType {
  pub cpp_type: CppType,
  pub cpp_ffi_type: CppType,
  pub cpp_to_ffi_conversion: IndirectionChange,
  pub rust_ffi_type: RustType,
  pub rust_api_type: RustType,
  pub rust_api_to_c_conversion: RustToCTypeConversion,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub struct RustQtSlotWrapper {
  pub arguments: Vec<CompleteType>,
  pub receiver_id: String,
  pub public_type_name: String,
  pub callback_name: String,
}

/// Information about a Rust type wrapper
#[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Serialize, Deserialize)]
pub enum RustTypeWrapperKind {
  /// Enum wrapper
  Enum {
    /// List of enum values
    values: Vec<RustEnumValue>,
    /// True if `FlaggableEnum` trait is implemented
    /// for this type, i.e. if `QFlags<T>` with this C++ type
    /// is used in API.
    is_flaggable: bool,
  },
  /// Struct wrapper
  Struct {
    /// Name of the constant containing size of the corresponding
    /// C++ type in bytes. Value of the constant is determined at
    /// crate compile time.
    size_const_name: String,
    /// True if `CppDeletable` trait is implemented
    /// for this type, i.e. if this C++ type has public destructor.
    is_deletable: bool,
  },
  /// Pointer-only type wrapper
  EmptyEnum {
    is_deletable: bool,
    slot_wrapper: Option<RustQtSlotWrapper>,
  },
}

// -------------------------
// from rust_generator

/// Exported information about a Rust wrapper type
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct RustProcessedTypeInfo {
  /// Full name of corresponding C++ type (class or enum).
  pub cpp_name: String,
  /// C++ documentation for this type
  pub cpp_doc: Option<CppTypeDoc>,
  /// Template arguments. None if C++ type is not a template class.
  pub cpp_template_arguments: Option<Vec<CppType>>,
  /// Kind of the type and additional information.
  pub kind: RustTypeWrapperKind,
  /// Identifier of Rust type
  pub rust_name: RustName,
  /// Indicates whether this type is public
  pub is_public: bool,
}



/// Exported information about generated crate
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct RustExportInfo {
  /// Name of the crate
  pub crate_name: String,
  /// Version of the crate
  pub crate_version: String,
  /// List of generated types
  pub rust_types: Vec<RustProcessedTypeInfo>,
}
