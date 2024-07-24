use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;

fn main() {
    struct GenSerde;

    impl CustomizeCallback for GenSerde {
        fn message(&self, _message: &MessageDescriptor) -> Customize {
            Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
        }

        fn field(&self, field: &FieldDescriptor) -> Customize {
            // if field.proto().type_() == Type::TYPE_ENUM {
            //     // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
            //     Customize::default().before(
            //         "#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")]")
            // } else {
            Customize::default()
            // }
        }

        fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
            Customize::default().before("#[serde(skip)]")
        }
    }

    Codegen::new()
        .protoc()
        .includes(&["tokenfactory"])
        // Inputs must reside in some of include paths.
        .input("tokenfactory/v1beta1/test.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();
}
