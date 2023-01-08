use chrono::NaiveDateTime;
use diesel::{prelude::*};
use rocket::serde::{Serialize};

#[derive(Queryable)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    #[serde(with = "naive_date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "naive_date_format")]
    pub updated_at: NaiveDateTime,
}

mod naive_date_format {
    use chrono::NaiveDateTime;
    use rocket::serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.%f+0000";
    /** NB: Can't use %+ format because NaiveDateTime can't include a timezone */
    // const FORMAT: &'static str = "%+";

    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // pub fn deserialize<'de, D>(
    //     deserializer: D,
    // ) -> Result<NaiveDateTime, D::Error>
    // where
    //     D: Deserializer<'de>,
    // {
    //     let s = String::deserialize(deserializer)?;
    //     NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    // }

}