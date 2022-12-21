use php_codegen::attribute::AttributeGroup;
use php_codegen::class::Class;
use php_codegen::comment::Document;
use php_codegen::constant::ClassConstant;
use php_codegen::constant::Constant;
use php_codegen::data_type::DataType;
use php_codegen::file::File;
use php_codegen::function::Function;
use php_codegen::interface::Interface;
use php_codegen::literal::Value;
use php_codegen::method::Method;
use php_codegen::modifiers::Modifier;
use php_codegen::modifiers::VisibilityModifier;
use php_codegen::parameter::Parameter;
use php_codegen::property::Property;
use php_codegen::usage::Usage;
use php_codegen::Indentation;

fn main() {
    let file =
        File::new()
            .namespaced("Foo")
            .declare("strict_types", 1)
            .uses("Foo\\Bar\\Baz as Qux")
            .uses("Throwable")
            .uses("DateTime")
            .uses("DateTimeImmutable")
            .uses_function("strlen")
            .uses_function("array_map")
            .uses_function("array_filter")
            .uses_constant("Foo\\HELLO")
            .uses_constant("Foo\\HEY")
            .constant(Constant::new("A").valued("Hello World!"))
            .constant(Constant::new("B").valued(()))
            .constant(Constant::new("C").valued(1))
            .constant(Constant::new("D").valued(false))
            .constant(Constant::new("E"))
            .constant(("F", 213.412))
            .constant(("G", vec![1, 25]))
            .function(
                Function::new("hello")
                    .document(
                        Document::new()
                            .text("This is a simple hello function.")
                            .empty_line()
                            .tag("param", "non-empty-string $firstname")
                            .empty_line()
                            .tag("return", "string")
                            .empty_line()
                            .simple_tag("pure"),
                    )
                    .attributes(AttributeGroup::new().add("Qux", Some("foo: 1, bar: 2")))
                    .parameter(
                        Parameter::new("firstname")
                            .typed(DataType::String)
                            .attributes(
                                AttributeGroup::new()
                                    .add("Validation\\NotBlank", None)
                                    .add("Validation\\Length", Some("min: 2, max: 10")),
                            ),
                    )
                    .parameter(
                        Parameter::new("lastname")
                            .typed(DataType::String)
                            .default(Value::Literal("Qux::Foo".to_string())),
                    )
                    .returns(DataType::String)
                    .body("return 'Hello ' . $firstname . ' ' . $lastname . '!';"),
            )
            .function(Function::new("nothing").returns(DataType::Void))
            .function(
                Function::new("format")
                    .parameter(Parameter::new("template").typed(DataType::String))
                    .parameter(
                        Parameter::new("args")
                            .variadic()
                            .typed(DataType::Union(vec![
                                DataType::Integer,
                                DataType::Float,
                                DataType::String,
                                DataType::Null,
                            ])),
                    )
                    .returns(DataType::String)
                    .body(vec![
                        "return sprintf($template, ...array_map(",
                        "    static fn ($arg) => is_float($arg) ? number_format($arg, 2) : $arg,",
                        "    array_filter($args, static fn ($arg) => $arg !== null)",
                        "));",
                    ]),
            )
            .class(
                Class::new("Example")
                    .extends("Foo\\Bar\\Baz")
                    .implements("Foo\\Bar\\BazInterface")
                    .document(
                        Document::new()
                            .text("This is an example class.")
                            .empty_line()
                            .simple_tag("immutable"),
                    )
                    .modifier(Modifier::Abstract)
                    .using("A")
                    .using(vec!["B", "C"])
                    .using(
                        Usage::new(vec![
                            "D".to_string(),
                            "E".to_string(),
                            "F".to_string(),
                            "G".to_string(),
                        ])
                        .rename("E::bar", "baz")
                        .alias("D::foo", "bar", VisibilityModifier::Public)
                        .public("E::qux")
                        .protected("D::format")
                        .private("D::d")
                        .precede("D::drop", vec!["E"])
                        .precede("G::something", vec!["E", "F", "D"])
                        .visibility("E::e", VisibilityModifier::Protected),
                    )
                    .constant(
                        ClassConstant::new("A")
                            .valued("Hello World!")
                            .modifier(Modifier::Final),
                    )
                    .constant(ClassConstant::new("B").valued(()).protected())
                    .constant(ClassConstant::new("C").valued(1).private())
                    .constant(ClassConstant::new("D").valued(false).public())
                    .property(Property::new("foo").typed(DataType::String).private())
                    .property(Property::new("bar").typed(DataType::String).protected())
                    .property(
                        Property::new("baz")
                            .typed(DataType::Union(vec![DataType::String, DataType::Integer]))
                            .public()
                            .default("Hello World!"),
                    )
                    .method(
                        Method::new("hello")
                            .returns(DataType::String)
                            .parameter(Parameter::new("firstname").typed(DataType::String))
                            .parameter(
                                Parameter::new("lastname")
                                    .typed(DataType::String)
                                    .default(Value::Literal("Qux::Foo".to_string())),
                            )
                            .body("return 'Hello ' . $firstname . ' ' . $lastname . '!';")
                            .attributes(
                                AttributeGroup::new()
                                    .add("Qux", Some("foo: 1, bar: 2"))
                                    .add("Qux", Some("foo: 1, bar: 2")),
                            )
                            .document(
                                Document::new()
                                    .text("This is a simple hello function.")
                                    .empty_line()
                                    .tag("param", "non-empty-string $firstname")
                                    .empty_line()
                                    .tag("return", "string")
                                    .empty_line()
                                    .simple_tag("pure"),
                            ),
                    )
                    .method(
                        Method::new("x")
                            .public()
                            .returns(DataType::Mixed)
                            .body("return 'Hello!';")
                            .attributes(
                                AttributeGroup::new()
                                    .add("Foo", Some("foo: 1, bar: 2"))
                                    .add("Bar", Some("foo: 1, bar: 2")),
                            )
                            .attributes(AttributeGroup::new().add("Baz", None).add("Qux", None))
                            .document(
                                Document::new()
                                    .text("This is a simple x function.")
                                    .empty_line()
                                    .simple_tag("pure"),
                            ),
                    )
                    .method(
                        Method::new("poop")
                            .public()
                            .modifier(Modifier::Abstract)
                            .returns(DataType::Void)
                            .document(Document::new().text("This is a simple poop function.")),
                    )
                    .method(
                        Method::new("helloWorld")
                            .public()
                            .modifier(Modifier::Final)
                            .returns(DataType::Void)
                            .document(Document::new().text("This is a simple echo function."))
                            .body(|indentation: Indentation, level| {
                                indentation.indent("echo 'Hello World!';", level)
                            }),
                    ),
            )
            .interface(
                Interface::new("Formatter")
                    .document(
                        Document::new()
                            .text("This is a simple formatter interface.")
                            .empty_line()
                            .simple_tag("immutable"),
                    )
                    .attributes(
                        AttributeGroup::new()
                            .add("Foo", Some("foo: 1, bar: 2"))
                            .add("Bar", Some("foo: 1, bar: 2")),
                    )
                    .extends("Foo")
                    .extends("Bar")
                    .extends("Qux")
                    .method(
                        Method::new("format")
                            .parameter(Parameter::new("template").typed(DataType::String))
                            .parameter(Parameter::new("args").variadic().typed(DataType::Union(
                                vec![
                                    DataType::Integer,
                                    DataType::Float,
                                    DataType::String,
                                    DataType::Null,
                                ],
                            )))
                            .returns(DataType::String),
                    ),
            );

    print!("{file}");
}
