use ipma_proto::v1::*;
use prost::Message;

#[test]
fn future_enum_numbers_and_source_values_round_trip() {
    let warning = Warning {
        level: "future colour".into(),
        warning_level: Some(WarningLevelValue {
            value: 777,
            source_value: "future colour".into(),
        }),
        ..Default::default()
    };
    let decoded = Warning::decode(warning.encode_to_vec().as_slice()).unwrap();
    assert_eq!(decoded, warning);
    assert_eq!(decoded.warning_level.unwrap().value, 777);
    let entry = CodeEntry {
        code: 123,
        description_portuguese: "Nova classe".into(),
        description_english: "New class".into(),
        classification: Some(code_entry::Classification::WeatherType(888)),
    };
    assert_eq!(
        CodeEntry::decode(entry.encode_to_vec().as_slice()).unwrap(),
        entry
    );
}

#[test]
fn explicit_zero_source_codes_and_direction_aliases_round_trip() {
    let value = PrecipitationIntensityValue {
        value: PrecipitationIntensity::NoPrecipitation as i32,
        source_code: 0,
    };
    assert_eq!(
        PrecipitationIntensityValue::decode(value.encode_to_vec().as_slice()).unwrap(),
        value
    );
    for code in [1, 9] {
        let value = DirectionValue {
            value: CompassDirection::North as i32,
            source: Some(direction_value::Source::Code(code)),
        };
        assert_eq!(
            DirectionValue::decode(value.encode_to_vec().as_slice()).unwrap(),
            value
        );
    }
}
