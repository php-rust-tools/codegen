use php_codegen::comment::Document;
use php_codegen::data_type::DataType;
use php_codegen::file::File;
use php_codegen::function::Function;
use php_codegen::parameter::Parameter;

fn main() {
    let file = File::new()
        .namespaced("App")
        .declare("strict_types", 1)
        .function(
            Function::new("format")
                .document(
                    Document::new()
                        .text("Format a string with the given arguments using sprintf.")
                        .empty_line()
                        .tag("param", "non-empty-string $template")
                        .empty_line()
                        .simple_tag("pure"),
                )
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
        );

    print!("{file}");
}
