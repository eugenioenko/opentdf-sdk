// @generated
impl serde::Serialize for ActionAttributeValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_identifier.is_some() {
            len += 1;
        }
        if self.attribute_value_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.ActionAttributeValue", len)?;
        if let Some(v) = self.action_identifier.as_ref() {
            match v {
                action_attribute_value::ActionIdentifier::ActionId(v) => {
                    struct_ser.serialize_field("actionId", v)?;
                }
                action_attribute_value::ActionIdentifier::ActionName(v) => {
                    struct_ser.serialize_field("actionName", v)?;
                }
            }
        }
        if let Some(v) = self.attribute_value_identifier.as_ref() {
            match v {
                action_attribute_value::AttributeValueIdentifier::AttributeValueId(v) => {
                    struct_ser.serialize_field("attributeValueId", v)?;
                }
                action_attribute_value::AttributeValueIdentifier::AttributeValueFqn(v) => {
                    struct_ser.serialize_field("attributeValueFqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActionAttributeValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_id",
            "actionId",
            "action_name",
            "actionName",
            "attribute_value_id",
            "attributeValueId",
            "attribute_value_fqn",
            "attributeValueFqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionId,
            ActionName,
            AttributeValueId,
            AttributeValueFqn,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "actionId" | "action_id" => Ok(GeneratedField::ActionId),
                            "actionName" | "action_name" => Ok(GeneratedField::ActionName),
                            "attributeValueId" | "attribute_value_id" => Ok(GeneratedField::AttributeValueId),
                            "attributeValueFqn" | "attribute_value_fqn" => Ok(GeneratedField::AttributeValueFqn),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionAttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.ActionAttributeValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActionAttributeValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_identifier__ = None;
                let mut attribute_value_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionId => {
                            if action_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionId"));
                            }
                            action_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(action_attribute_value::ActionIdentifier::ActionId);
                        }
                        GeneratedField::ActionName => {
                            if action_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionName"));
                            }
                            action_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(action_attribute_value::ActionIdentifier::ActionName);
                        }
                        GeneratedField::AttributeValueId => {
                            if attribute_value_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueId"));
                            }
                            attribute_value_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(action_attribute_value::AttributeValueIdentifier::AttributeValueId);
                        }
                        GeneratedField::AttributeValueFqn => {
                            if attribute_value_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueFqn"));
                            }
                            attribute_value_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(action_attribute_value::AttributeValueIdentifier::AttributeValueFqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ActionAttributeValue {
                    action_identifier: action_identifier__,
                    attribute_value_identifier: attribute_value_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.ActionAttributeValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRegisteredResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.CreateRegisteredResourceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRegisteredResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "values",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Values,
            NamespaceId,
            NamespaceFqn,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "values" => Ok(GeneratedField::Values),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRegisteredResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.CreateRegisteredResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRegisteredResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut values__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceFqn => {
                            if namespace_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceFqn"));
                            }
                            namespace_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateRegisteredResourceRequest {
                    name: name__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.CreateRegisteredResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRegisteredResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.CreateRegisteredResourceResponse", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRegisteredResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRegisteredResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.CreateRegisteredResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRegisteredResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateRegisteredResourceResponse {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.CreateRegisteredResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.action_attribute_values.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.CreateRegisteredResourceValueRequest", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.action_attribute_values.is_empty() {
            struct_ser.serialize_field("actionAttributeValues", &self.action_attribute_values)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
            "value",
            "action_attribute_values",
            "actionAttributeValues",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
            Value,
            ActionAttributeValues,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            "value" => Ok(GeneratedField::Value),
                            "actionAttributeValues" | "action_attribute_values" => Ok(GeneratedField::ActionAttributeValues),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRegisteredResourceValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.CreateRegisteredResourceValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRegisteredResourceValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                let mut value__ = None;
                let mut action_attribute_values__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionAttributeValues => {
                            if action_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionAttributeValues"));
                            }
                            action_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateRegisteredResourceValueRequest {
                    resource_id: resource_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    action_attribute_values: action_attribute_values__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.CreateRegisteredResourceValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.CreateRegisteredResourceValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRegisteredResourceValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.CreateRegisteredResourceValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRegisteredResourceValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateRegisteredResourceValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.CreateRegisteredResourceValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.DeleteRegisteredResourceRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.DeleteRegisteredResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRegisteredResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteRegisteredResourceRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.DeleteRegisteredResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.DeleteRegisteredResourceResponse", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.DeleteRegisteredResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRegisteredResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteRegisteredResourceResponse {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.DeleteRegisteredResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.DeleteRegisteredResourceValueRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredResourceValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.DeleteRegisteredResourceValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRegisteredResourceValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteRegisteredResourceValueRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.DeleteRegisteredResourceValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.DeleteRegisteredResourceValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredResourceValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.DeleteRegisteredResourceValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRegisteredResourceValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteRegisteredResourceValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.DeleteRegisteredResourceValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceRequest", len)?;
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_registered_resource_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                get_registered_resource_request::Identifier::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_fqn",
            "namespaceFqn",
            "namespace_id",
            "namespaceId",
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceFqn,
            NamespaceId,
            Id,
            Name,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_fqn__ = None;
                let mut namespace_id__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceFqn => {
                            if namespace_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceFqn"));
                            }
                            namespace_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_registered_resource_request::Identifier::Id);
                        }
                        GeneratedField::Name => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_registered_resource_request::Identifier::Name);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceRequest {
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceResponse", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceResponse {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceValueRequest", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_registered_resource_value_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                get_registered_resource_value_request::Identifier::Fqn(v) => {
                    struct_ser.serialize_field("fqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Fqn,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "fqn" => Ok(GeneratedField::Fqn),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_registered_resource_value_request::Identifier::Id);
                        }
                        GeneratedField::Fqn => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_registered_resource_value_request::Identifier::Fqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceValueRequest {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceValuesByFqNsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceValuesByFQNsRequest", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceValuesByFqNsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fqns,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqns" => Ok(GeneratedField::Fqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceValuesByFqNsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceValuesByFQNsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceValuesByFqNsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fqns => {
                            if fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqns"));
                            }
                            fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceValuesByFqNsRequest {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceValuesByFQNsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredResourceValuesByFqNsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqn_value_map.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.GetRegisteredResourceValuesByFQNsResponse", len)?;
        if !self.fqn_value_map.is_empty() {
            struct_ser.serialize_field("fqnValueMap", &self.fqn_value_map)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredResourceValuesByFqNsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqn_value_map",
            "fqnValueMap",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FqnValueMap,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqnValueMap" | "fqn_value_map" => Ok(GeneratedField::FqnValueMap),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredResourceValuesByFqNsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.GetRegisteredResourceValuesByFQNsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRegisteredResourceValuesByFqNsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqn_value_map__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FqnValueMap => {
                            if fqn_value_map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqnValueMap"));
                            }
                            fqn_value_map__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetRegisteredResourceValuesByFqNsResponse {
                    fqn_value_map: fqn_value_map__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.GetRegisteredResourceValuesByFQNsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRegisteredResourceValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.ListRegisteredResourceValuesRequest", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRegisteredResourceValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRegisteredResourceValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.ListRegisteredResourceValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRegisteredResourceValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListRegisteredResourceValuesRequest {
                    resource_id: resource_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.ListRegisteredResourceValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRegisteredResourceValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.ListRegisteredResourceValuesResponse", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRegisteredResourceValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRegisteredResourceValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.ListRegisteredResourceValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRegisteredResourceValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListRegisteredResourceValuesResponse {
                    values: values__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.ListRegisteredResourceValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRegisteredResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.sort.is_empty() {
            len += 1;
        }
        if self.search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.ListRegisteredResourcesRequest", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.sort.is_empty() {
            struct_ser.serialize_field("sort", &self.sort)?;
        }
        if let Some(v) = self.search.as_ref() {
            struct_ser.serialize_field("search", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRegisteredResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "pagination",
            "sort",
            "search",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
            NamespaceFqn,
            Pagination,
            Sort,
            Search,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "sort" => Ok(GeneratedField::Sort),
                            "search" => Ok(GeneratedField::Search),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRegisteredResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.ListRegisteredResourcesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRegisteredResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut pagination__ = None;
                let mut sort__ = None;
                let mut search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceFqn => {
                            if namespace_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceFqn"));
                            }
                            namespace_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Sort => {
                            if sort__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            sort__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Search => {
                            if search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            search__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListRegisteredResourcesRequest {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                    search: search__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.ListRegisteredResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRegisteredResourcesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resources.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.ListRegisteredResourcesResponse", len)?;
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRegisteredResourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resources",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resources,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resources" => Ok(GeneratedField::Resources),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRegisteredResourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.ListRegisteredResourcesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRegisteredResourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resources__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListRegisteredResourcesResponse {
                    resources: resources__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.ListRegisteredResourcesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredResourcesSort {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.RegisteredResourcesSort", len)?;
        if self.field != 0 {
            let v = SortRegisteredResourcesType::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::SortDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredResourcesSort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredResourcesSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.RegisteredResourcesSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredResourcesSort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<SortRegisteredResourcesType>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::SortDirection>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RegisteredResourcesSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.RegisteredResourcesSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortRegisteredResourcesType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_REGISTERED_RESOURCES_TYPE_UNSPECIFIED",
            Self::Name => "SORT_REGISTERED_RESOURCES_TYPE_NAME",
            Self::CreatedAt => "SORT_REGISTERED_RESOURCES_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_REGISTERED_RESOURCES_TYPE_UPDATED_AT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortRegisteredResourcesType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_REGISTERED_RESOURCES_TYPE_UNSPECIFIED",
            "SORT_REGISTERED_RESOURCES_TYPE_NAME",
            "SORT_REGISTERED_RESOURCES_TYPE_CREATED_AT",
            "SORT_REGISTERED_RESOURCES_TYPE_UPDATED_AT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortRegisteredResourcesType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SORT_REGISTERED_RESOURCES_TYPE_UNSPECIFIED" => Ok(SortRegisteredResourcesType::Unspecified),
                    "SORT_REGISTERED_RESOURCES_TYPE_NAME" => Ok(SortRegisteredResourcesType::Name),
                    "SORT_REGISTERED_RESOURCES_TYPE_CREATED_AT" => Ok(SortRegisteredResourcesType::CreatedAt),
                    "SORT_REGISTERED_RESOURCES_TYPE_UPDATED_AT" => Ok(SortRegisteredResourcesType::UpdatedAt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegisteredResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.UpdateRegisteredResourceRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegisteredResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Metadata,
            MetadataUpdateBehavior,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "metadataUpdateBehavior" | "metadata_update_behavior" => Ok(GeneratedField::MetadataUpdateBehavior),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegisteredResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.UpdateRegisteredResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegisteredResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut metadata__ = None;
                let mut metadata_update_behavior__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataUpdateBehavior => {
                            if metadata_update_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUpdateBehavior"));
                            }
                            metadata_update_behavior__ = Some(map_.next_value::<super::super::common::MetadataUpdateEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateRegisteredResourceRequest {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.UpdateRegisteredResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegisteredResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.UpdateRegisteredResourceResponse", len)?;
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegisteredResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resource,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegisteredResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.UpdateRegisteredResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegisteredResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateRegisteredResourceResponse {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.UpdateRegisteredResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.action_attribute_values.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.UpdateRegisteredResourceValueRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.action_attribute_values.is_empty() {
            struct_ser.serialize_field("actionAttributeValues", &self.action_attribute_values)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegisteredResourceValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value",
            "action_attribute_values",
            "actionAttributeValues",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Value,
            ActionAttributeValues,
            Metadata,
            MetadataUpdateBehavior,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "value" => Ok(GeneratedField::Value),
                            "actionAttributeValues" | "action_attribute_values" => Ok(GeneratedField::ActionAttributeValues),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "metadataUpdateBehavior" | "metadata_update_behavior" => Ok(GeneratedField::MetadataUpdateBehavior),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegisteredResourceValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.UpdateRegisteredResourceValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegisteredResourceValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value__ = None;
                let mut action_attribute_values__ = None;
                let mut metadata__ = None;
                let mut metadata_update_behavior__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionAttributeValues => {
                            if action_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionAttributeValues"));
                            }
                            action_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataUpdateBehavior => {
                            if metadata_update_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUpdateBehavior"));
                            }
                            metadata_update_behavior__ = Some(map_.next_value::<super::super::common::MetadataUpdateEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateRegisteredResourceValueRequest {
                    id: id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    action_attribute_values: action_attribute_values__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.UpdateRegisteredResourceValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.registeredresources.UpdateRegisteredResourceValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegisteredResourceValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegisteredResourceValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.registeredresources.UpdateRegisteredResourceValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRegisteredResourceValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateRegisteredResourceValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.registeredresources.UpdateRegisteredResourceValueResponse", FIELDS, GeneratedVisitor)
    }
}
