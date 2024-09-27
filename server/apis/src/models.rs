#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetRecommendedWordsForUserPathParams {
            /// ID of the user
                pub user_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetUsersWithSimilarWordsPathParams {
            /// ID of the user
                pub user_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct StampsGetQueryParams {
                #[serde(rename = "word")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub word: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct StampsPostHeaderParams {
        pub x_forwarded_user: String,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTodayTrendingWordsQueryParams {
            /// 返すwordの数
                #[serde(rename = "limit")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub limit: Option<i32>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForDayPathParams {
            /// Specific day in the format \"YYYY-MM-DD\"
                pub day: String,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForDayQueryParams {
            /// 返すwordの数
                #[serde(rename = "limit")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub limit: Option<i32>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForMonthPathParams {
            /// Specific month in the format \"YYYY-MM\"
                pub month: String,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForMonthQueryParams {
            /// 返すwordの数
                #[serde(rename = "limit")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub limit: Option<i32>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForYearPathParams {
            /// Specific year in the format \"YYYY\"
                pub year: String,
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct GetTrendingWordsForYearQueryParams {
            /// 返すwordの数
                #[serde(rename = "limit")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub limit: Option<i32>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsGetQueryParams {
                #[serde(rename = "word")]
                #[serde(skip_serializing_if="Option::is_none")]
                pub word: Option<String>,
    }

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsMeGetHeaderParams {
        pub x_forwarded_user: String,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsPostHeaderParams {
        pub x_forwarded_user: String,
    }

            
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsUserIdGetPathParams {
            /// traP ID で指定する
                pub user_id: String,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsWordIdDeleteHeaderParams {
        pub x_forwarded_user: String,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsWordIdDeletePathParams {
            /// 単語の uuid で指定する
                pub word_id: uuid::Uuid,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct WordsWordIdPutHeaderParams {
        pub x_forwarded_user: String,
    }

            
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct WordsWordIdPutPathParams {
            /// 単語の uuid で指定する
                pub word_id: uuid::Uuid,
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
pub struct MyStamp {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "time")]
    pub time: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}


impl MyStamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, id: uuid::Uuid, time: chrono::DateTime::<chrono::Utc>, excluded_users: models::ExcludedUsers, ) -> MyStamp {
        MyStamp {
            name,
            id,
            time,
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

            Some("name".to_string()),
            Some(self.name.to_string()),

            // Skipping id in query parameter serialization

            // Skipping time in query parameter serialization

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
            pub name: Vec<String>,
            pub id: Vec<uuid::Uuid>,
            pub time: Vec<chrono::DateTime::<chrono::Utc>>,
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
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time" => intermediate_rep.time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in MyStamp".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in MyStamp".to_string())?,
            time: intermediate_rep.time.into_iter().next().ok_or_else(|| "time missing in MyStamp".to_string())?,
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
    #[serde(rename = "word")]
    pub word: String,

    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "time")]
    pub time: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}


impl MyWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(word: String, id: uuid::Uuid, time: chrono::DateTime::<chrono::Utc>, excluded_users: models::ExcludedUsers, ) -> MyWord {
        MyWord {
            word,
            id,
            time,
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

            Some("word".to_string()),
            Some(self.word.to_string()),

            // Skipping id in query parameter serialization

            // Skipping time in query parameter serialization

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
            pub word: Vec<String>,
            pub id: Vec<uuid::Uuid>,
            pub time: Vec<chrono::DateTime::<chrono::Utc>>,
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
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "time" => intermediate_rep.time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in MyWord".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in MyWord".to_string())?,
            time: intermediate_rep.time.into_iter().next().ok_or_else(|| "time missing in MyWord".to_string())?,
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
    #[serde(rename = "stamp")]
    pub stamp: String,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}


impl NewStamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(stamp: String, excluded_users: models::ExcludedUsers, ) -> NewStamp {
        NewStamp {
            stamp,
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

            Some("stamp".to_string()),
            Some(self.stamp.to_string()),

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
            pub stamp: Vec<String>,
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
                    "stamp" => intermediate_rep.stamp.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
            stamp: intermediate_rep.stamp.into_iter().next().ok_or_else(|| "stamp missing in NewStamp".to_string())?,
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
    #[serde(rename = "word")]
    pub word: String,

    #[serde(rename = "excludedUsers")]
    pub excluded_users: models::ExcludedUsers,

}


impl NewWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(word: String, excluded_users: models::ExcludedUsers, ) -> NewWord {
        NewWord {
            word,
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

            Some("word".to_string()),
            Some(self.word.to_string()),

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
            pub word: Vec<String>,
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
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in NewWord".to_string())?,
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
pub struct RecommendedWord {
    #[serde(rename = "number")]
    pub number: i32,

    #[serde(rename = "word")]
    pub word: String,

}


impl RecommendedWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(number: i32, word: String, ) -> RecommendedWord {
        RecommendedWord {
            number,
            word,
        }
    }
}

/// Converts the RecommendedWord value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RecommendedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("number".to_string()),
            Some(self.number.to_string()),


            Some("word".to_string()),
            Some(self.word.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecommendedWord value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecommendedWord {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub number: Vec<i32>,
            pub word: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RecommendedWord".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "number" => intermediate_rep.number.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RecommendedWord".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecommendedWord {
            number: intermediate_rep.number.into_iter().next().ok_or_else(|| "number missing in RecommendedWord".to_string())?,
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in RecommendedWord".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecommendedWord> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RecommendedWord>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RecommendedWord>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RecommendedWord - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RecommendedWord> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RecommendedWord as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RecommendedWord - {}",
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
pub struct RecommendedWords(Vec<RecommendedWord>);

impl validator::Validate for RecommendedWords {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<RecommendedWord>> for RecommendedWords {
    fn from(x: Vec<RecommendedWord>) -> Self {
        RecommendedWords(x)
    }
}

impl std::convert::From<RecommendedWords> for Vec<RecommendedWord> {
    fn from(x: RecommendedWords) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<RecommendedWord> for RecommendedWords {
    fn from_iter<U: IntoIterator<Item=RecommendedWord>>(u: U) -> Self {
        RecommendedWords(Vec::<RecommendedWord>::from_iter(u))
    }
}

impl std::iter::IntoIterator for RecommendedWords {
    type Item = RecommendedWord;
    type IntoIter = std::vec::IntoIter<RecommendedWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a RecommendedWords {
    type Item = &'a RecommendedWord;
    type IntoIter = std::slice::Iter<'a, RecommendedWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut RecommendedWords {
    type Item = &'a mut RecommendedWord;
    type IntoIter = std::slice::IterMut<'a, RecommendedWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for RecommendedWords {
    type Target = Vec<RecommendedWord>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RecommendedWords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the RecommendedWords value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RecommendedWords {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecommendedWords value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecommendedWords {
    type Err = <RecommendedWord as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(RecommendedWords(items))
    }
}


// Methods for converting between header::IntoHeaderValue<RecommendedWords> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RecommendedWords>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RecommendedWords>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RecommendedWords - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RecommendedWords> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RecommendedWords as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RecommendedWords - {}",
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
pub struct SimilarUser {
    #[serde(rename = "trapId")]
    pub trap_id: String,

}


impl SimilarUser {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(trap_id: String, ) -> SimilarUser {
        SimilarUser {
            trap_id,
        }
    }
}

/// Converts the SimilarUser value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SimilarUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("trapId".to_string()),
            Some(self.trap_id.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SimilarUser value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SimilarUser {
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
                None => return std::result::Result::Err("Missing value while parsing SimilarUser".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trapId" => intermediate_rep.trap_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SimilarUser".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SimilarUser {
            trap_id: intermediate_rep.trap_id.into_iter().next().ok_or_else(|| "trapId missing in SimilarUser".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SimilarUser> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SimilarUser>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SimilarUser>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SimilarUser - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SimilarUser> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SimilarUser as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SimilarUser - {}",
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
pub struct SimilarUsers(Vec<SimilarUser>);

impl validator::Validate for SimilarUsers {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<SimilarUser>> for SimilarUsers {
    fn from(x: Vec<SimilarUser>) -> Self {
        SimilarUsers(x)
    }
}

impl std::convert::From<SimilarUsers> for Vec<SimilarUser> {
    fn from(x: SimilarUsers) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<SimilarUser> for SimilarUsers {
    fn from_iter<U: IntoIterator<Item=SimilarUser>>(u: U) -> Self {
        SimilarUsers(Vec::<SimilarUser>::from_iter(u))
    }
}

impl std::iter::IntoIterator for SimilarUsers {
    type Item = SimilarUser;
    type IntoIter = std::vec::IntoIter<SimilarUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a SimilarUsers {
    type Item = &'a SimilarUser;
    type IntoIter = std::slice::Iter<'a, SimilarUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut SimilarUsers {
    type Item = &'a mut SimilarUser;
    type IntoIter = std::slice::IterMut<'a, SimilarUser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for SimilarUsers {
    type Target = Vec<SimilarUser>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SimilarUsers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the SimilarUsers value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for SimilarUsers {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SimilarUsers value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SimilarUsers {
    type Err = <SimilarUser as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(SimilarUsers(items))
    }
}


// Methods for converting between header::IntoHeaderValue<SimilarUsers> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<SimilarUsers>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SimilarUsers>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SimilarUsers - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<SimilarUsers> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SimilarUsers as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SimilarUsers - {}",
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
    #[serde(rename = "trapId")]
    pub trap_id: String,

    #[serde(rename = "name")]
    pub name: String,

}


impl Stamp {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(trap_id: String, name: String, ) -> Stamp {
        Stamp {
            trap_id,
            name,
        }
    }
}

/// Converts the Stamp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Stamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("trapId".to_string()),
            Some(self.trap_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),

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
            pub trap_id: Vec<String>,
            pub name: Vec<String>,
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
                    "trapId" => intermediate_rep.trap_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Stamp".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Stamp {
            trap_id: intermediate_rep.trap_id.into_iter().next().ok_or_else(|| "trapId missing in Stamp".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Stamp".to_string())?,
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
pub struct Stamps(Vec<Word>);

impl validator::Validate for Stamps {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<Word>> for Stamps {
    fn from(x: Vec<Word>) -> Self {
        Stamps(x)
    }
}

impl std::convert::From<Stamps> for Vec<Word> {
    fn from(x: Stamps) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Word> for Stamps {
    fn from_iter<U: IntoIterator<Item=Word>>(u: U) -> Self {
        Stamps(Vec::<Word>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Stamps {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Stamps {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Stamps {
    type Item = &'a mut Word;
    type IntoIter = std::slice::IterMut<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for Stamps {
    type Target = Vec<Word>;
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
    type Err = <Word as std::str::FromStr>::Err;

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







#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TrendingWord {
    #[serde(rename = "number")]
    pub number: i32,

    #[serde(rename = "word")]
    pub word: String,

}


impl TrendingWord {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(number: i32, word: String, ) -> TrendingWord {
        TrendingWord {
            number,
            word,
        }
    }
}

/// Converts the TrendingWord value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TrendingWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("number".to_string()),
            Some(self.number.to_string()),


            Some("word".to_string()),
            Some(self.word.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TrendingWord value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TrendingWord {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub number: Vec<i32>,
            pub word: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TrendingWord".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "number" => intermediate_rep.number.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TrendingWord".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TrendingWord {
            number: intermediate_rep.number.into_iter().next().ok_or_else(|| "number missing in TrendingWord".to_string())?,
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in TrendingWord".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TrendingWord> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TrendingWord>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TrendingWord>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TrendingWord - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TrendingWord> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TrendingWord as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TrendingWord - {}",
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
pub struct TrendingWords(Vec<TrendingWord>);

impl validator::Validate for TrendingWords {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<TrendingWord>> for TrendingWords {
    fn from(x: Vec<TrendingWord>) -> Self {
        TrendingWords(x)
    }
}

impl std::convert::From<TrendingWords> for Vec<TrendingWord> {
    fn from(x: TrendingWords) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<TrendingWord> for TrendingWords {
    fn from_iter<U: IntoIterator<Item=TrendingWord>>(u: U) -> Self {
        TrendingWords(Vec::<TrendingWord>::from_iter(u))
    }
}

impl std::iter::IntoIterator for TrendingWords {
    type Item = TrendingWord;
    type IntoIter = std::vec::IntoIter<TrendingWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a TrendingWords {
    type Item = &'a TrendingWord;
    type IntoIter = std::slice::Iter<'a, TrendingWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut TrendingWords {
    type Item = &'a mut TrendingWord;
    type IntoIter = std::slice::IterMut<'a, TrendingWord>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for TrendingWords {
    type Target = Vec<TrendingWord>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TrendingWords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the TrendingWords value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TrendingWords {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TrendingWords value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TrendingWords {
    type Err = <TrendingWord as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(TrendingWords(items))
    }
}


// Methods for converting between header::IntoHeaderValue<TrendingWords> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TrendingWords>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TrendingWords>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TrendingWords - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TrendingWords> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TrendingWords as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TrendingWords - {}",
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
pub struct Word {
    #[serde(rename = "trapId")]
    pub trap_id: String,

    #[serde(rename = "word")]
    pub word: String,

}


impl Word {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(trap_id: String, word: String, ) -> Word {
        Word {
            trap_id,
            word,
        }
    }
}

/// Converts the Word value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("trapId".to_string()),
            Some(self.trap_id.to_string()),


            Some("word".to_string()),
            Some(self.word.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Word value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Word {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trap_id: Vec<String>,
            pub word: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Word".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trapId" => intermediate_rep.trap_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Word".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Word {
            trap_id: intermediate_rep.trap_id.into_iter().next().ok_or_else(|| "trapId missing in Word".to_string())?,
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in Word".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Word> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Word>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Word>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Word - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Word> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Word as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Word - {}",
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
pub struct Words(Vec<Word>);

impl validator::Validate for Words {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::convert::From<Vec<Word>> for Words {
    fn from(x: Vec<Word>) -> Self {
        Words(x)
    }
}

impl std::convert::From<Words> for Vec<Word> {
    fn from(x: Words) -> Self {
        x.0
    }
}

impl std::iter::FromIterator<Word> for Words {
    fn from_iter<U: IntoIterator<Item=Word>>(u: U) -> Self {
        Words(Vec::<Word>::from_iter(u))
    }
}

impl std::iter::IntoIterator for Words {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a Words {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::IntoIterator for &'a mut Words {
    type Item = &'a mut Word;
    type IntoIter = std::slice::IterMut<'a, Word>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::ops::Deref for Words {
    type Target = Vec<Word>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Words {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Converts the Words value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Words {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Words value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Words {
    type Err = <Word as std::str::FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut items = vec![];
        for item in s.split(',')
        {
            items.push(item.parse()?);
        }
        std::result::Result::Ok(Words(items))
    }
}


// Methods for converting between header::IntoHeaderValue<Words> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Words>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Words>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Words - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Words> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Words as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Words - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



