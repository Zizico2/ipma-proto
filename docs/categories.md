# Categorical values

The IPMA adapter translates documented source vocabularies into these shared
enums. The gateway forwards them; it does not infer or reinterpret categories.
Mappings require no additional upstream requests.

| Vocabulary | Data fields |
| --- | --- |
| `WarningLevel` | `Warning.warning_level` |
| `WarningType` | `Warning.warning_type` |
| `WeatherType` | `WeatherForecast.weather_type` |
| `WindSpeedClass` | `WeatherForecast.wind_speed_classification` |
| `PrecipitationIntensity` | `WeatherForecast.precipitation_intensity` |
| `FireRiskClass` | `FireRisk.classification` |
| `CompassDirection` | Forecast/observation `wind_direction`, sea `wave_direction` |

Use domain names on parent fields and the consistent name `value` inside
wrappers. For example:

```protobuf
message WarningLevelValue {
  WarningLevel value = 1;
  string source_value = 2;
}
```

Numeric wrappers retain `source_code`; warning wrappers retain `source_value`.
`DirectionValue` retains text or a numeric code through its `source` oneof.
Normalizing case/whitespace for matching never changes the retained source.
`CodeEntry` composes the original code and bilingual descriptions with a
`classification` oneof (`weather_type`, `wind_speed_class` or
`precipitation_intensity`). Descriptions stay in reference tables rather than
being duplicated on every forecast. All published weather codes remain distinct,
even when translated descriptions overlap.

Missing/null numeric categories and IPMA's -99 sentinel produce absent wrappers;
blank warning/direction strings also produce absent wrappers. A reference-table
row with code -99 is retained with an explicitly selected `UNSPECIFIED` enum.
Present unfamiliar source values map to `OTHER`, preserving the source for
inspection and future mappings. This differs from a future protobuf enum number:
clients and intermediaries must preserve unknown numeric enum values as received.
In Rust, inspect the raw `i32` field or use `TryFrom`; the generated enum accessor
returns its default for unknown enum numbers.

Enum numbers are API identifiers, not a general copy of IPMA codes. Zero always
means `UNSPECIFIED` in an enum, while source zero may be meaningful:

- Weather code 0 means `NO_INFORMATION`.
- Precipitation code 0 means `NO_PRECIPITATION`.
- Station direction code 0 means `NO_DIRECTION`.
- Station direction codes 1 and 9 both mean north, with their original code retained.

Existing fields, tags and RPC paths are unchanged. The typed fields are additive.
Older servers may omit them, so clients adopting them should retain a fallback to
existing raw fields until all servers have upgraded. Future changes must preserve
enum numbers and field tags; reserve removed names/numbers.

District, municipality/DICO, warning-area, station and location values remain
identifiers, not enums. DICO strings retain leading zeroes. UV risk is not inferred
from a numeric index. Geobuf preserves source properties without typed remapping.

The source vocabularies are documented by [IPMA](https://api.ipma.pt/), including
[weather types](https://api.ipma.pt/open-data/weather-type-classe.json),
[wind-speed classes](https://api.ipma.pt/open-data/wind-speed-daily-classe.json) and
[precipitation classes](https://api.ipma.pt/open-data/precipitation-classe.json).
