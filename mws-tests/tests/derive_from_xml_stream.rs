#[macro_use]
extern crate mws_derive;
#[macro_use]
extern crate mws;
extern crate chrono;

use chrono::{DateTime, Utc};
pub use mws::{result, xmlhelper};

#[test]
fn derive_struct() {
  #[derive(Debug, PartialEq, Default, FromXmlStream)]
  struct S {
    a: String,
    b: i32,
    date: Option<DateTime<Utc>>,
  }

  test_decode!(
    S,
    r#"
      <a>AAA</a>
      <b>777</b>
      <date>2016-11-03T00:09:40Z</date>
    "#,
    S {
      a: "AAA".to_owned(),
      b: 777,
      date: Some("2016-11-03T00:09:40Z".parse().unwrap())
    }
  );
}

#[test]
fn derive_vec() {
  #[derive(Debug, PartialEq, Default, FromXmlStream)]
  struct S {
    items: Vec<i32>,
  }

  test_decode!(
    S,
    r#"
      <items>
        <value>1</value>
        <value>3</value>
        <value>5</value>
        <value>7</value>
      </items>
    "#,
    S {
      items: vec![1, 3, 5, 7],
    }
  );
}

#[test]
fn derive_struct_vec() {
  #[derive(Debug, PartialEq, Default, FromXmlStream)]
  struct Item {
    a: String,
    b: i32,
  }

  #[derive(Debug, PartialEq, Default, FromXmlStream)]
  struct S {
    items: Vec<Item>,
  }

  test_decode!(
    S,
    r#"
      <items>
        <item>
          <a>AAA</a>
          <b>777</b>
        </item>
        <item>
          <a>BBB</a>
          <b>888</b>
        </item>
      </items>
    "#,
    S {
      items: vec![
        Item {
          a: "AAA".to_string(),
          b: 777,
        },
        Item {
          a: "BBB".to_string(),
          b: 888,
        },
      ],
    }
  );
}
