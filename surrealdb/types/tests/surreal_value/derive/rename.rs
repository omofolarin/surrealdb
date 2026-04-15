use surrealdb_types::{SurrealValue, Value, object};

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged)]
enum DisputeStatus {
	#[surreal(rename = "customer_won")]
	CustomerWon,
	#[surreal(rename = "brand_won")]
	BrandWon,
}

#[test]
fn test_enum_variant_rename_unit_variant() {
	assert_eq!(DisputeStatus::CustomerWon.into_value(), Value::String("customer_won".into()));
	assert_eq!(DisputeStatus::BrandWon.into_value(), Value::String("brand_won".into()));
	assert_eq!(
		DisputeStatus::from_value(Value::String("customer_won".into())).unwrap(),
		DisputeStatus::CustomerWon
	);
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
enum RenamedNamedVariant {
	#[surreal(rename = "customer_won")]
	CustomerWon {
		reason: String,
	},
}

#[test]
fn test_enum_variant_rename_named_variant() {
	let value = RenamedNamedVariant::CustomerWon {
		reason: "chargeback".into(),
	}
	.into_value();

	assert_eq!(
		value,
		Value::Object(object! {
			"customer_won".to_string() => Value::Object(object! {
				"reason".to_string() => Value::String("chargeback".into())
			})
		})
	);
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
enum RenamedUnnamedVariant {
	#[surreal(rename = "customer_won")]
	CustomerWon(String),
}

#[test]
fn test_enum_variant_rename_unnamed_variant() {
	let value = RenamedUnnamedVariant::CustomerWon("chargeback".into()).into_value();

	assert_eq!(
		value,
		Value::Object(object! {
			"customer_won".to_string() => Value::String("chargeback".into())
		})
	);
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(rename_all = "camelCase")]
struct Order {
	customer_id: String,
	total_amount: f64,
}

#[test]
fn test_struct_rename_all() {
	let value = Order {
		customer_id: "c_1".into(),
		total_amount: 7.5,
	}
	.into_value();

	if let Value::Object(obj) = &value {
		assert_eq!(obj.get("customerId"), Some(&Value::String("c_1".into())));
		assert_eq!(obj.get("totalAmount"), Some(&Value::Number(7.5.into())));
		assert!(obj.get("customer_id").is_none());
		assert!(obj.get("total_amount").is_none());
	} else {
		panic!("Expected object value");
	}
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(rename_all = "camelCase")]
struct ExplicitRenameWins {
	#[surreal(rename = "EXPLICIT")]
	foo_bar: String,
}

#[test]
fn test_struct_rename_precedence_over_rename_all() {
	let value = ExplicitRenameWins {
		foo_bar: "value".into(),
	}
	.into_value();

	if let Value::Object(obj) = &value {
		assert_eq!(obj.get("EXPLICIT"), Some(&Value::String("value".into())));
		assert!(obj.get("fooBar").is_none());
	} else {
		panic!("Expected object value");
	}
}

macro_rules! assert_enum_rename_all_case {
	($enum_ty:ty, $variant:path, $expected:expr) => {
		assert_eq!($variant.into_value(), Value::String($expected.into()));
		assert_eq!(
			<$enum_ty as SurrealValue>::from_value(Value::String($expected.into())).unwrap(),
			$variant
		);
	};
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "lowercase")]
enum EnumLowercase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "UPPERCASE")]
enum EnumUppercase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "PascalCase")]
enum EnumPascalCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "camelCase")]
enum EnumCamelCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "snake_case")]
enum EnumSnakeCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "SCREAMING_SNAKE_CASE")]
enum EnumScreamingSnakeCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "kebab-case")]
enum EnumKebabCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[derive(SurrealValue, Debug, PartialEq)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, rename_all = "SCREAMING-KEBAB-CASE")]
enum EnumScreamingKebabCase {
	InProgress,
	ShippedOut,
	OnHold,
}

#[test]
fn test_enum_rename_all_supported_values() {
	assert_enum_rename_all_case!(EnumLowercase, EnumLowercase::InProgress, "inprogress");
	assert_enum_rename_all_case!(EnumUppercase, EnumUppercase::InProgress, "INPROGRESS");
	assert_enum_rename_all_case!(EnumPascalCase, EnumPascalCase::InProgress, "InProgress");
	assert_enum_rename_all_case!(EnumCamelCase, EnumCamelCase::InProgress, "inProgress");
	assert_enum_rename_all_case!(EnumSnakeCase, EnumSnakeCase::InProgress, "in_progress");
	assert_enum_rename_all_case!(
		EnumScreamingSnakeCase,
		EnumScreamingSnakeCase::InProgress,
		"IN_PROGRESS"
	);
	assert_enum_rename_all_case!(EnumKebabCase, EnumKebabCase::InProgress, "in-progress");
	assert_enum_rename_all_case!(
		EnumScreamingKebabCase,
		EnumScreamingKebabCase::InProgress,
		"IN-PROGRESS"
	);
}
