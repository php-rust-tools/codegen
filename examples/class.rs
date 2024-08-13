use php_codegen::class::Class;
use php_codegen::data_type::DataType;
use php_codegen::file::File;
use php_codegen::literal::Value;
use php_codegen::modifiers::VisibilityModifier;
use php_codegen::property::Property;
use php_codegen::property::PropertyHook;
use php_codegen::property::PropertySetHookParameter;

fn main() {
    let file = File::new()
        .namespaced("App")
        .declare("strict_types", 1)
        // A class that uses property hooks
        .class(
            Class::new("SimpleUser")
                .property(
                    Property::new("firstName")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Private)
                        .default(Value::String("Jane".to_string())),
                )
                .property(
                    Property::new("lastName")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Private)
                        .default(Value::String("Doe".to_string())),
                )
                .property(
                    Property::new("fullname")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Public)
                        .hook(PropertyHook::Get(
                            false,
                            vec!["return $this->firstName . ' ' . $this->lastName;"].into(),
                        ))
                        .hook(PropertyHook::Set(
                            Some(
                                PropertySetHookParameter::new("$fullname").typed(DataType::String),
                            ),
                            vec![
                                "[$first, $last] = explode(' ', $fullname);",
                                "$this->firstName = $first;",
                                "$this->lastName = $last;",
                            ]
                            .into(),
                        )),
                ),
        )
        .class(
            Class::new("SimpleUser2")
                .property(
                    Property::new("firstName")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Private)
                        .default(Value::String("Jane".to_string())),
                )
                .property(
                    Property::new("lastName")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Private)
                        .default(Value::String("Doe".to_string())),
                )
                .property(
                    Property::new("fullname")
                        .typed(DataType::String)
                        .visibility(VisibilityModifier::Public)
                        .hook(PropertyHook::Get(
                            false,
                            vec!["return $this->firstName . ' ' . $this->lastName;"].into(),
                        )),
                ),
        );

    print!("{file}");
}
