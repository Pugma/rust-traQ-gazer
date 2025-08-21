#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct MeGetHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct StampsGetHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct StampsPostHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct StampsStampIdDeleteHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct StampsStampIdDeletePathParams {
            /// スタンプの uuid で指定する
                pub stamp_id: uuid::Uuid,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct StampsStampIdExclusionsPutHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct StampsStampIdExclusionsPutPathParams {
            /// スタンプの uuid で指定する
                pub stamp_id: uuid::Uuid,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsPostHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsWordIdDeleteHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsWordIdDeletePathParams {
            /// 単語の uuid で指定する
                pub word_id: uuid::Uuid,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsWordIdExclusionsPutHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsWordIdExclusionsPutPathParams {
            /// 単語の uuid で指定する
                pub word_id: uuid::Uuid,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct MeStampsGetHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct MeWordsGetHeaderParams {
        pub x_forwarded_user: Option<String>,
    }

            


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExcludedUser {
    #[serde(rename = "trapId")]
    pub trap_id: String,

}





impl ExcludedUser {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(trap_id: String, ) -> ExcludedUser {
        ExcludedUser {
            trap_id,
        }
    }
}

/// Converts the ExcludedUser value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExcludedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("trapId".to_string()),
            Some(self.trap_id.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExcludedUser value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExcludedUser {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trap_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ExcludedUser".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trapId" => intermediate_rep.trap_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ExcludedUser".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ExcludedUser {
            trap_id: intermediate_rep.trap_id.into_iter().next().ok_or_else(|| "trapId missing in ExcludedUser".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ExcludedUser> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExcludedUser>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExcludedUser>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExcludedUser - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ExcludedUser> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExcludedUser as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExcludedUser - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ExcludedUsers(Vec<ExcludedUser>);

impl validator::Validate for ExcludedUsers {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<ExcludedUser>> for ExcludedUsers {
    fn from(x: Vec<ExcludedUser>) -> Self {
        ExcludedUsers(x)
    }
}

impl std::convert::From<ExcludedUsers> for Vec<ExcludedUser> {
    fn from(x: ExcludedUsers) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<ExcludedUser> for ExcludedUsers {
    fn from_iter<U: IntoIterator<Item=ExcludedUser>>(u: U) -> Self {
        ExcludedUsers(Vec::<ExcludedUser>::from_iter(u))
    }
}

impl std::iter::IntoIterator for ExcludedUsers {
    type Item = ExcludedUser;
    type IntoIter = std::vec::IntoIter<ExcludedUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a ExcludedUsers {
    type Item = &'a ExcludedUser;
    type IntoIter = std::slice::Iter<'a, ExcludedUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut ExcludedUsers {
    type Item = &'a mut ExcludedUser;
    type IntoIter = std::slice::IterMut<'a, ExcludedUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for ExcludedUsers {
    type Target = Vec<ExcludedUser>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExcludedUsers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the ExcludedUsers value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ExcludedUsers {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ExcludedUsers value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ExcludedUsers {
    type Err = <ExcludedUser as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(ExcludedUsers(items))
    }
}


// Methods for converting between header::IntoHeaderValue<ExcludedUsers> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ExcludedUsers>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ExcludedUsers>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ExcludedUsers - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ExcludedUsers> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ExcludedUsers as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ExcludedUsers - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyInfo {
    #[serde(rename = "trapId")]
    pub trap_id: String,

}





impl MyInfo {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(trap_id: String, ) -> MyInfo {
        MyInfo {
            trap_id,
        }
    }
}

/// Converts the MyInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("trapId".to_string()),
            Some(self.trap_id.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trap_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MyInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trapId" => intermediate_rep.trap_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MyInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MyInfo {
            trap_id: intermediate_rep.trap_id.into_iter().next().ok_or_else(|| "trapId missing in MyInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MyInfo> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MyInfo>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MyInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MyInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MyInfo> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MyInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MyInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyStamp {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "stampName")]
    pub stamp_name: String,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}





impl MyStamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: uuid::Uuid, stamp_name: String, excluded_users: models::ExcludedUsers, ) -> MyStamp {
        MyStamp {
            id,
            stamp_name,
            excluded_users,
        }
    }
}

/// Converts the MyStamp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MyStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization


            Some("stampName".to_string()),
            Some(self.stamp_name.to_string()),

            // Skipping excludedUsers in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyStamp value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyStamp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub stamp_name: Vec<String>,
            pub excluded_users: Vec<models::ExcludedUsers>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MyStamp".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stampName" => intermediate_rep.stamp_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "excludedUsers" => intermediate_rep.excluded_users.push(<models::ExcludedUsers as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MyStamp".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MyStamp {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in MyStamp".to_string())?,
            stamp_name: intermediate_rep.stamp_name.into_iter().next().ok_or_else(|| "stampName missing in MyStamp".to_string())?,
            excluded_users: intermediate_rep.excluded_users.into_iter().next().ok_or_else(|| "excludedUsers missing in MyStamp".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MyStamp> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MyStamp>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MyStamp>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MyStamp - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MyStamp> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MyStamp as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MyStamp - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyStamps(Vec<MyStamp>);

impl validator::Validate for MyStamps {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<MyStamp>> for MyStamps {
    fn from(x: Vec<MyStamp>) -> Self {
        MyStamps(x)
    }
}

impl std::convert::From<MyStamps> for Vec<MyStamp> {
    fn from(x: MyStamps) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<MyStamp> for MyStamps {
    fn from_iter<U: IntoIterator<Item=MyStamp>>(u: U) -> Self {
        MyStamps(Vec::<MyStamp>::from_iter(u))
    }
}

impl std::iter::IntoIterator for MyStamps {
    type Item = MyStamp;
    type IntoIter = std::vec::IntoIter<MyStamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a MyStamps {
    type Item = &'a MyStamp;
    type IntoIter = std::slice::Iter<'a, MyStamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut MyStamps {
    type Item = &'a mut MyStamp;
    type IntoIter = std::slice::IterMut<'a, MyStamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for MyStamps {
    type Target = Vec<MyStamp>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MyStamps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the MyStamps value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MyStamps {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyStamps value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyStamps {
    type Err = <MyStamp as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(MyStamps(items))
    }
}


// Methods for converting between header::IntoHeaderValue<MyStamps> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MyStamps>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MyStamps>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MyStamps - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MyStamps> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MyStamps as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MyStamps - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyWord {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "wordValue")]
    pub word_value: String,

    #[serde(rename = "isRegex")]
    pub is_regex: bool,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}





impl MyWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: uuid::Uuid, word_value: String, is_regex: bool, excluded_users: models::ExcludedUsers, ) -> MyWord {
        MyWord {
            id,
            word_value,
            is_regex,
            excluded_users,
        }
    }
}

/// Converts the MyWord value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MyWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization


            Some("wordValue".to_string()),
            Some(self.word_value.to_string()),


            Some("isRegex".to_string()),
            Some(self.is_regex.to_string()),

            // Skipping excludedUsers in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyWord value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyWord {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<uuid::Uuid>,
            pub word_value: Vec<String>,
            pub is_regex: Vec<bool>,
            pub excluded_users: Vec<models::ExcludedUsers>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MyWord".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "wordValue" => intermediate_rep.word_value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "isRegex" => intermediate_rep.is_regex.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "excludedUsers" => intermediate_rep.excluded_users.push(<models::ExcludedUsers as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MyWord".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MyWord {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in MyWord".to_string())?,
            word_value: intermediate_rep.word_value.into_iter().next().ok_or_else(|| "wordValue missing in MyWord".to_string())?,
            is_regex: intermediate_rep.is_regex.into_iter().next().ok_or_else(|| "isRegex missing in MyWord".to_string())?,
            excluded_users: intermediate_rep.excluded_users.into_iter().next().ok_or_else(|| "excludedUsers missing in MyWord".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MyWord> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MyWord>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MyWord>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MyWord - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MyWord> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MyWord as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MyWord - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MyWords(Vec<MyWord>);

impl validator::Validate for MyWords {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<MyWord>> for MyWords {
    fn from(x: Vec<MyWord>) -> Self {
        MyWords(x)
    }
}

impl std::convert::From<MyWords> for Vec<MyWord> {
    fn from(x: MyWords) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<MyWord> for MyWords {
    fn from_iter<U: IntoIterator<Item=MyWord>>(u: U) -> Self {
        MyWords(Vec::<MyWord>::from_iter(u))
    }
}

impl std::iter::IntoIterator for MyWords {
    type Item = MyWord;
    type IntoIter = std::vec::IntoIter<MyWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a MyWords {
    type Item = &'a MyWord;
    type IntoIter = std::slice::Iter<'a, MyWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut MyWords {
    type Item = &'a mut MyWord;
    type IntoIter = std::slice::IterMut<'a, MyWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for MyWords {
    type Target = Vec<MyWord>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MyWords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the MyWords value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MyWords {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MyWords value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MyWords {
    type Err = <MyWord as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(MyWords(items))
    }
}


// Methods for converting between header::IntoHeaderValue<MyWords> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MyWords>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MyWords>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MyWords - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MyWords> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MyWords as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MyWords - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




/// sessionからuserIdを取る
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewStamp {
    #[serde(rename = "stampName")]
    pub stamp_name: String,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}





impl NewStamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(stamp_name: String, excluded_users: models::ExcludedUsers, ) -> NewStamp {
        NewStamp {
            stamp_name,
            excluded_users,
        }
    }
}

/// Converts the NewStamp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("stampName".to_string()),
            Some(self.stamp_name.to_string()),

            // Skipping excludedUsers in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewStamp value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewStamp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub stamp_name: Vec<String>,
            pub excluded_users: Vec<models::ExcludedUsers>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewStamp".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "stampName" => intermediate_rep.stamp_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "excludedUsers" => intermediate_rep.excluded_users.push(<models::ExcludedUsers as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewStamp".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewStamp {
            stamp_name: intermediate_rep.stamp_name.into_iter().next().ok_or_else(|| "stampName missing in NewStamp".to_string())?,
            excluded_users: intermediate_rep.excluded_users.into_iter().next().ok_or_else(|| "excludedUsers missing in NewStamp".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewStamp> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewStamp>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewStamp>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewStamp - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewStamp> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewStamp as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewStamp - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




/// sessionからuserIdを取る
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewWord {
    #[serde(rename = "wordValue")]
    pub word_value: String,

    #[serde(rename = "isRegex")]
    pub is_regex: bool,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}





impl NewWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(word_value: String, is_regex: bool, excluded_users: models::ExcludedUsers, ) -> NewWord {
        NewWord {
            word_value,
            is_regex,
            excluded_users,
        }
    }
}

/// Converts the NewWord value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("wordValue".to_string()),
            Some(self.word_value.to_string()),


            Some("isRegex".to_string()),
            Some(self.is_regex.to_string()),

            // Skipping excludedUsers in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewWord value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewWord {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub word_value: Vec<String>,
            pub is_regex: Vec<bool>,
            pub excluded_users: Vec<models::ExcludedUsers>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewWord".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "wordValue" => intermediate_rep.word_value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "isRegex" => intermediate_rep.is_regex.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "excludedUsers" => intermediate_rep.excluded_users.push(<models::ExcludedUsers as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewWord".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewWord {
            word_value: intermediate_rep.word_value.into_iter().next().ok_or_else(|| "wordValue missing in NewWord".to_string())?,
            is_regex: intermediate_rep.is_regex.into_iter().next().ok_or_else(|| "isRegex missing in NewWord".to_string())?,
            excluded_users: intermediate_rep.excluded_users.into_iter().next().ok_or_else(|| "excludedUsers missing in NewWord".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewWord> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewWord>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewWord>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewWord - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewWord> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewWord as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewWord - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Stamp {
    #[serde(rename = "stampName")]
    pub stamp_name: String,

    #[serde(rename = "stampId")]
    pub stamp_id: uuid::Uuid,

}





impl Stamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(stamp_name: String, stamp_id: uuid::Uuid, ) -> Stamp {
        Stamp {
            stamp_name,
            stamp_id,
        }
    }
}

/// Converts the Stamp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Stamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("stampName".to_string()),
            Some(self.stamp_name.to_string()),

            // Skipping stampId in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Stamp value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Stamp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub stamp_name: Vec<String>,
            pub stamp_id: Vec<uuid::Uuid>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Stamp".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "stampName" => intermediate_rep.stamp_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "stampId" => intermediate_rep.stamp_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Stamp".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Stamp {
            stamp_name: intermediate_rep.stamp_name.into_iter().next().ok_or_else(|| "stampName missing in Stamp".to_string())?,
            stamp_id: intermediate_rep.stamp_id.into_iter().next().ok_or_else(|| "stampId missing in Stamp".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Stamp> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Stamp>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Stamp>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Stamp - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Stamp> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Stamp as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Stamp - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Stamps(Vec<Stamp>);

impl validator::Validate for Stamps {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<Stamp>> for Stamps {
    fn from(x: Vec<Stamp>) -> Self {
        Stamps(x)
    }
}

impl std::convert::From<Stamps> for Vec<Stamp> {
    fn from(x: Stamps) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Stamp> for Stamps {
    fn from_iter<U: IntoIterator<Item=Stamp>>(u: U) -> Self {
        Stamps(Vec::<Stamp>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Stamps {
    type Item = Stamp;
    type IntoIter = std::vec::IntoIter<Stamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Stamps {
    type Item = &'a Stamp;
    type IntoIter = std::slice::Iter<'a, Stamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Stamps {
    type Item = &'a mut Stamp;
    type IntoIter = std::slice::IterMut<'a, Stamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for Stamps {
    type Target = Vec<Stamp>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Stamps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Stamps value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Stamps {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Stamps value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Stamps {
    type Err = <Stamp as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Stamps(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Stamps> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Stamps>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Stamps>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Stamps - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Stamps> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Stamps as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Stamps - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



