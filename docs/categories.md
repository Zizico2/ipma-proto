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

Reference identities also have typed wrappers: `GeographicRegion`, `District`,
`Municipality`, `WarningArea`, `ForecastLocation`, `CoastalLocation` and `Station`.
The large catalogues are explicitly partial snapshots, with `OTHER` for identities
outside the snapshot. Their enum numbers are independent of the source IDs; do not
cast an enum to construct a source ID. Existing request selectors still use the
original source IDs.

`MunicipalityValue.source_code` is the full DICO and retains leading zeroes.
Location records compose it from idDistrito and idConcelho only when both exist;
partial components remain available without inventing a global identity. District
context and the original local code accompany the value when present in that
response. Climate/fire responses retain their supplied DICO without an extra lookup.

`UvPeriod` recognises source codes 0 and 10 without assuming their internal meaning;
the original hour interval accompanies the code. `SeismicMagnitudeType`,
`SeismicDegree` and `SeismicSource` preserve exact published tokens. Free-text
observation regions and localities are not classified. UV risk is not inferred
from a numeric index.

Geobuf bytes and source properties remain unchanged. `feature_categories` supplies
optional typed companions joined by the zero-based feature index, never a feature
ID. Null properties omit an entry without shifting later indices. Stations carry
their typed ID and source label when present. Bivalves carry typed zone, zone type,
status, region, scientific taxon, common label, sanitary class and harvest-list
membership. Documentation/live aliases (`samp` versus `code`, `CLOSED` versus
`CLOSE`) retain their exact spelling; the two status tokens are distinct enum cases.
Undocumented qualifiers such as `B*`, `NA` and `NC` are recognised tokens without
an inferred safety interpretation. Future list names and taxa remain available
through `OTHER` and their original values. Missing scientific names can still carry
a common label with `UNSPECIFIED` taxon.

All mappings happen in the adapter using compile-time matches in
`ipma-service/src/reference.rs`; there are no additional reference-data fetches.
When expanding a catalogue, append stable enum cases without renumbering existing
ones and update the adapter matches in the same rollout. Unknown cases stay valid
data until then. The gateway preserves and caches these fields, including future
numeric protobuf enum values; it does not reinterpret them.

The source vocabularies are documented by [IPMA](https://api.ipma.pt/), including
[weather types](https://api.ipma.pt/open-data/weather-type-classe.json),
[wind-speed classes](https://api.ipma.pt/open-data/wind-speed-daily-classe.json) and
[precipitation classes](https://api.ipma.pt/open-data/precipitation-classe.json).
