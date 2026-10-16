// @generated
impl serde::Serialize for CreateResourceMappingGroupRequest {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.CreateResourceMappingGroupRequest", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceMappingGroupRequest {
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
            "name",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
            NamespaceFqn,
            Name,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = CreateResourceMappingGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.CreateResourceMappingGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceMappingGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut name__ = None;
                let mut metadata__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateResourceMappingGroupRequest {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.CreateResourceMappingGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.CreateResourceMappingGroupResponse", len)?;
        if let Some(v) = self.resource_mapping_group.as_ref() {
            struct_ser.serialize_field("resourceMappingGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping_group",
            "resourceMappingGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappingGroup,
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
                            "resourceMappingGroup" | "resource_mapping_group" => Ok(GeneratedField::ResourceMappingGroup),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceMappingGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.CreateResourceMappingGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceMappingGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappingGroup => {
                            if resource_mapping_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappingGroup"));
                            }
                            resource_mapping_group__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateResourceMappingGroupResponse {
                    resource_mapping_group: resource_mapping_group__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.CreateResourceMappingGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceMappingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_value_id.is_empty() {
            len += 1;
        }
        if !self.terms.is_empty() {
            len += 1;
        }
        if !self.group_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.CreateResourceMappingRequest", len)?;
        if !self.attribute_value_id.is_empty() {
            struct_ser.serialize_field("attributeValueId", &self.attribute_value_id)?;
        }
        if !self.terms.is_empty() {
            struct_ser.serialize_field("terms", &self.terms)?;
        }
        if !self.group_id.is_empty() {
            struct_ser.serialize_field("groupId", &self.group_id)?;
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
impl<'de> serde::Deserialize<'de> for CreateResourceMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_value_id",
            "attributeValueId",
            "terms",
            "group_id",
            "groupId",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeValueId,
            Terms,
            GroupId,
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
                            "attributeValueId" | "attribute_value_id" => Ok(GeneratedField::AttributeValueId),
                            "terms" => Ok(GeneratedField::Terms),
                            "groupId" | "group_id" => Ok(GeneratedField::GroupId),
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
            type Value = CreateResourceMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.CreateResourceMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_value_id__ = None;
                let mut terms__ = None;
                let mut group_id__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeValueId => {
                            if attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueId"));
                            }
                            attribute_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Terms => {
                            if terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            terms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GroupId => {
                            if group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupId"));
                            }
                            group_id__ = Some(map_.next_value()?);
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
                Ok(CreateResourceMappingRequest {
                    attribute_value_id: attribute_value_id__.unwrap_or_default(),
                    terms: terms__.unwrap_or_default(),
                    group_id: group_id__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.CreateResourceMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.CreateResourceMappingResponse", len)?;
        if let Some(v) = self.resource_mapping.as_ref() {
            struct_ser.serialize_field("resourceMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping",
            "resourceMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMapping,
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
                            "resourceMapping" | "resource_mapping" => Ok(GeneratedField::ResourceMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.CreateResourceMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMapping => {
                            if resource_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMapping"));
                            }
                            resource_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateResourceMappingResponse {
                    resource_mapping: resource_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.CreateResourceMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteResourceMappingGroupRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.DeleteResourceMappingGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteResourceMappingGroupRequest {
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
            type Value = DeleteResourceMappingGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.DeleteResourceMappingGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteResourceMappingGroupRequest, V::Error>
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
                Ok(DeleteResourceMappingGroupRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.DeleteResourceMappingGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.DeleteResourceMappingGroupResponse", len)?;
        if let Some(v) = self.resource_mapping_group.as_ref() {
            struct_ser.serialize_field("resourceMappingGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping_group",
            "resourceMappingGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappingGroup,
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
                            "resourceMappingGroup" | "resource_mapping_group" => Ok(GeneratedField::ResourceMappingGroup),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteResourceMappingGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.DeleteResourceMappingGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteResourceMappingGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappingGroup => {
                            if resource_mapping_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappingGroup"));
                            }
                            resource_mapping_group__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteResourceMappingGroupResponse {
                    resource_mapping_group: resource_mapping_group__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.DeleteResourceMappingGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteResourceMappingRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.DeleteResourceMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteResourceMappingRequest {
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
            type Value = DeleteResourceMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.DeleteResourceMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteResourceMappingRequest, V::Error>
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
                Ok(DeleteResourceMappingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.DeleteResourceMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteResourceMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.DeleteResourceMappingResponse", len)?;
        if let Some(v) = self.resource_mapping.as_ref() {
            struct_ser.serialize_field("resourceMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteResourceMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping",
            "resourceMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMapping,
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
                            "resourceMapping" | "resource_mapping" => Ok(GeneratedField::ResourceMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteResourceMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.DeleteResourceMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteResourceMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMapping => {
                            if resource_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMapping"));
                            }
                            resource_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteResourceMappingResponse {
                    resource_mapping: resource_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.DeleteResourceMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceMappingGroupRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.GetResourceMappingGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceMappingGroupRequest {
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
            type Value = GetResourceMappingGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.GetResourceMappingGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceMappingGroupRequest, V::Error>
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
                Ok(GetResourceMappingGroupRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.GetResourceMappingGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.GetResourceMappingGroupResponse", len)?;
        if let Some(v) = self.resource_mapping_group.as_ref() {
            struct_ser.serialize_field("resourceMappingGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping_group",
            "resourceMappingGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappingGroup,
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
                            "resourceMappingGroup" | "resource_mapping_group" => Ok(GeneratedField::ResourceMappingGroup),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceMappingGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.GetResourceMappingGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceMappingGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappingGroup => {
                            if resource_mapping_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappingGroup"));
                            }
                            resource_mapping_group__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetResourceMappingGroupResponse {
                    resource_mapping_group: resource_mapping_group__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.GetResourceMappingGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceMappingRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.GetResourceMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceMappingRequest {
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
            type Value = GetResourceMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.GetResourceMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceMappingRequest, V::Error>
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
                Ok(GetResourceMappingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.GetResourceMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.GetResourceMappingResponse", len)?;
        if let Some(v) = self.resource_mapping.as_ref() {
            struct_ser.serialize_field("resourceMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping",
            "resourceMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMapping,
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
                            "resourceMapping" | "resource_mapping" => Ok(GeneratedField::ResourceMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.GetResourceMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMapping => {
                            if resource_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMapping"));
                            }
                            resource_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetResourceMappingResponse {
                    resource_mapping: resource_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.GetResourceMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingGroupsRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingGroupsRequest", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingGroupsRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
            NamespaceFqn,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
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
            type Value = ListResourceMappingGroupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingGroupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingGroupsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut pagination__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListResourceMappingGroupsRequest {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingGroupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingGroupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_mapping_groups.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingGroupsResponse", len)?;
        if !self.resource_mapping_groups.is_empty() {
            struct_ser.serialize_field("resourceMappingGroups", &self.resource_mapping_groups)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingGroupsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping_groups",
            "resourceMappingGroups",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappingGroups,
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
                            "resourceMappingGroups" | "resource_mapping_groups" => Ok(GeneratedField::ResourceMappingGroups),
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
            type Value = ListResourceMappingGroupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingGroupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingGroupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping_groups__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappingGroups => {
                            if resource_mapping_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappingGroups"));
                            }
                            resource_mapping_groups__ = Some(map_.next_value()?);
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
                Ok(ListResourceMappingGroupsResponse {
                    resource_mapping_groups: resource_mapping_groups__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingGroupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingsByGroupFqnsRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingsByGroupFqnsRequest", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingsByGroupFqnsRequest {
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
            type Value = ListResourceMappingsByGroupFqnsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingsByGroupFqnsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingsByGroupFqnsRequest, V::Error>
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
                Ok(ListResourceMappingsByGroupFqnsRequest {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingsByGroupFqnsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingsByGroupFqnsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqn_resource_mapping_groups.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingsByGroupFqnsResponse", len)?;
        if !self.fqn_resource_mapping_groups.is_empty() {
            struct_ser.serialize_field("fqnResourceMappingGroups", &self.fqn_resource_mapping_groups)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingsByGroupFqnsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqn_resource_mapping_groups",
            "fqnResourceMappingGroups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FqnResourceMappingGroups,
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
                            "fqnResourceMappingGroups" | "fqn_resource_mapping_groups" => Ok(GeneratedField::FqnResourceMappingGroups),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListResourceMappingsByGroupFqnsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingsByGroupFqnsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingsByGroupFqnsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqn_resource_mapping_groups__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FqnResourceMappingGroups => {
                            if fqn_resource_mapping_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqnResourceMappingGroups"));
                            }
                            fqn_resource_mapping_groups__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListResourceMappingsByGroupFqnsResponse {
                    fqn_resource_mapping_groups: fqn_resource_mapping_groups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingsByGroupFqnsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.group_id.is_empty() {
            len += 1;
        }
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingsRequest", len)?;
        if !self.group_id.is_empty() {
            struct_ser.serialize_field("groupId", &self.group_id)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group_id",
            "groupId",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GroupId,
            NamespaceId,
            NamespaceFqn,
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
                            "groupId" | "group_id" => Ok(GeneratedField::GroupId),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
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
            type Value = ListResourceMappingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group_id__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GroupId => {
                            if group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupId"));
                            }
                            group_id__ = Some(map_.next_value()?);
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
                Ok(ListResourceMappingsRequest {
                    group_id: group_id__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceMappingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_mappings.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ListResourceMappingsResponse", len)?;
        if !self.resource_mappings.is_empty() {
            struct_ser.serialize_field("resourceMappings", &self.resource_mappings)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceMappingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mappings",
            "resourceMappings",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappings,
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
                            "resourceMappings" | "resource_mappings" => Ok(GeneratedField::ResourceMappings),
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
            type Value = ListResourceMappingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ListResourceMappingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceMappingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mappings__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappings => {
                            if resource_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappings"));
                            }
                            resource_mappings__ = Some(map_.next_value()?);
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
                Ok(ListResourceMappingsResponse {
                    resource_mappings: resource_mappings__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ListResourceMappingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceMappingsByGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.group.is_some() {
            len += 1;
        }
        if !self.mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.ResourceMappingsByGroup", len)?;
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if !self.mappings.is_empty() {
            struct_ser.serialize_field("mappings", &self.mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceMappingsByGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "mappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Mappings,
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
                            "group" => Ok(GeneratedField::Group),
                            "mappings" => Ok(GeneratedField::Mappings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceMappingsByGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.ResourceMappingsByGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceMappingsByGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut group__ = None;
                let mut mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                        GeneratedField::Mappings => {
                            if mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mappings"));
                            }
                            mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResourceMappingsByGroup {
                    group: group__,
                    mappings: mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.ResourceMappingsByGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceMappingGroupRequest {
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
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.namespace_fqn.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.UpdateResourceMappingGroupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
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
impl<'de> serde::Deserialize<'de> for UpdateResourceMappingGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "name",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            NamespaceId,
            NamespaceFqn,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
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
            type Value = UpdateResourceMappingGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.UpdateResourceMappingGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceMappingGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
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
                Ok(UpdateResourceMappingGroupRequest {
                    id: id__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.UpdateResourceMappingGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.UpdateResourceMappingGroupResponse", len)?;
        if let Some(v) = self.resource_mapping_group.as_ref() {
            struct_ser.serialize_field("resourceMappingGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResourceMappingGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping_group",
            "resourceMappingGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMappingGroup,
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
                            "resourceMappingGroup" | "resource_mapping_group" => Ok(GeneratedField::ResourceMappingGroup),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateResourceMappingGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.UpdateResourceMappingGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceMappingGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMappingGroup => {
                            if resource_mapping_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappingGroup"));
                            }
                            resource_mapping_group__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateResourceMappingGroupResponse {
                    resource_mapping_group: resource_mapping_group__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.UpdateResourceMappingGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceMappingRequest {
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
        if !self.attribute_value_id.is_empty() {
            len += 1;
        }
        if !self.terms.is_empty() {
            len += 1;
        }
        if !self.group_id.is_empty() {
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
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.UpdateResourceMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.attribute_value_id.is_empty() {
            struct_ser.serialize_field("attributeValueId", &self.attribute_value_id)?;
        }
        if !self.terms.is_empty() {
            struct_ser.serialize_field("terms", &self.terms)?;
        }
        if !self.group_id.is_empty() {
            struct_ser.serialize_field("groupId", &self.group_id)?;
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
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResourceMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "attribute_value_id",
            "attributeValueId",
            "terms",
            "group_id",
            "groupId",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AttributeValueId,
            Terms,
            GroupId,
            NamespaceId,
            NamespaceFqn,
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
                            "attributeValueId" | "attribute_value_id" => Ok(GeneratedField::AttributeValueId),
                            "terms" => Ok(GeneratedField::Terms),
                            "groupId" | "group_id" => Ok(GeneratedField::GroupId),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
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
            type Value = UpdateResourceMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.UpdateResourceMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut attribute_value_id__ = None;
                let mut terms__ = None;
                let mut group_id__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
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
                        GeneratedField::AttributeValueId => {
                            if attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueId"));
                            }
                            attribute_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Terms => {
                            if terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            terms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GroupId => {
                            if group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupId"));
                            }
                            group_id__ = Some(map_.next_value()?);
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
                Ok(UpdateResourceMappingRequest {
                    id: id__.unwrap_or_default(),
                    attribute_value_id: attribute_value_id__.unwrap_or_default(),
                    terms: terms__.unwrap_or_default(),
                    group_id: group_id__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.UpdateResourceMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.resourcemapping.UpdateResourceMappingResponse", len)?;
        if let Some(v) = self.resource_mapping.as_ref() {
            struct_ser.serialize_field("resourceMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResourceMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_mapping",
            "resourceMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceMapping,
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
                            "resourceMapping" | "resource_mapping" => Ok(GeneratedField::ResourceMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateResourceMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.resourcemapping.UpdateResourceMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceMapping => {
                            if resource_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMapping"));
                            }
                            resource_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateResourceMappingResponse {
                    resource_mapping: resource_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.resourcemapping.UpdateResourceMappingResponse", FIELDS, GeneratedVisitor)
    }
}
