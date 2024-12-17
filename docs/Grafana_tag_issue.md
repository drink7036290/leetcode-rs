# InfluxDB 3.0 SQL in Grafana: "impl" Tag Issue

## Summary

When querying InfluxDB 3.0 with SQL in Grafana, a field named `impl` with data type `Dictionary(Int32, Utf8)`(which is a columnar optimization used in InfluxDB 3.0 for efficient storage) is treated as a string field instead of a tag. This causes data from differnet tags to be mixed together instead of different data series.
This differs from the behavior when using Flux, where "impl" is correctly recognized as a tag.

## Details

* **InfluxDB Version:** 3.0 (Cloud Serverless)
* **Grafana Version:** Grafana Cloud (steady)
* **Data Source:** InfluxDB
* **Query Language:** SQL

**InfluxDB Data:**

The `impl` field in InfluxDB has the data type `Dictionary(Int32, Utf8)`.

**Grafana Query Example ($Measurement is a Grafana variable):**

```sql
SELECT time, impl, "Runtime_estimate" FROM "$Measurement"
```

**DataFrame JSON Example (From Query inspector):**

Here, `impl` is presented as a string field rather than a tag-like dimension.

```json
[
  {
    "schema": {
      // ... schema details
      "fields": [
        {
          "name": "Runtime_estimate",
          "type": "number",
          "typeInfo": {"frame": "float64", "nullable": true},
          "config": {}
        },
        {
          "name": "time",
          "type": "time",
          "typeInfo": {"frame": "time.Time"},
          "config": {}
        },
        {
          "name": "impl",
          "type": "string",
          "typeInfo": {"frame": "string", "nullable": true},
          "config": {}
        }
      ]
    },
    "data": {
      "values": [
        [440013.6142377223, 441454.6712280183, ...],
        [1734155609961, 1734164102116, ...],
        ["intrusive_two_hashmaps", "priority_queue", ...]
      ]
    }
  }
]
```

**Expected Behavior:**

The "impl" field should be recognized as a tag in Grafana, similar to how it's handled when using Flux.

**Actual Behavior:**

Grafana treats "impl" as a regular string field.

## Analysis

It’s not necessarily a bug, but rather a difference in how Grafana and InfluxDB 3.0 interact compared to InfluxDB 1.x/2.x with Flux.

**Key Points:**

* **InfluxDB 3.0 and Flight SQL vs. Flux:**

  In InfluxDB 3.0 (Cloud Serverless), queries are executed via Flight SQL. The underlying storage engine and query interface differ significantly from InfluxDB 1.x or 2.x. As a result, how data types and metadata are exposed also differs.

* **No Strict “Tags” vs “Fields” in SQL Metadata:**

  InfluxDB’s SQL interface on v3 presents data as columns in a table-like structure, without explicitly classifying them as “tags” or “fields” in the same way Flux or InfluxQL does. When using Flux queries, you see _field and_measurement columns, along with tag columns, which allow Grafana to infer what is a tag, field, or time column. With the SQL interface, such semantics may not be provided, so Grafana just sees a string column.

* **Dictionary Column Type in InfluxDB 3.0:**

  The Dictionary(Int32, Utf8) type is a columnar optimization in Arrow format, commonly used by InfluxDB 3.0 for efficient storage. Grafana’s DataFrame model, when translating Arrow/Flight SQL data, turns this into a standard string column. Since no additional metadata indicates it should be considered a tag, Grafana treats it as just another field.

* **Mismatch in Expectations from Previous InfluxDB Versions:**

  With InfluxDB 2.x and Flux queries, Grafana had a well-defined schema and metadata to understand tags and fields. In InfluxDB 3.0 SQL queries, that context is missing. As a result, what was once a “tag” in Flux queries may just appear as a string column in SQL queries, losing that semantic meaning.

* **Current State of InfluxDB 3.0 Integrations:**

  InfluxDB 3.0 and its Flight SQL integration are relatively new, and Grafana’s support for it is still evolving. Grafana’s current handling of columns returned by the SQL endpoint might not fully reflect the tag/field schema you’re used to from Flux queries.

This may change over time as the InfluxDB and Grafana ecosystems mature. Grafana plugins or the core platform might add better heuristics or metadata reading capabilities to differentiate “tag-like” columns from fields when using Flight SQL.

## Conclusion

It’s likely not a strict “bug” in Grafana, but rather a limitation or missing feature in the current integration with InfluxDB 3.0’s SQL interface.

Grafana simply sees a string column and does not have the same metadata as Flux to treat it as a tag.

The best next step might be to open a feature request or discussion on Grafana’s GitHub or community forums, explaining the difference between Flux and SQL query results in InfluxDB 3.0 and how you’d like tags to be identified. This could prompt improvements or clarifications in future releases.

## Considering InfluxQL vs. Flux

  InfluxQL was typically used for InfluxDB 1.x, where data was stored in traditional databases.

  In InfluxDB 2.x, you must map buckets to databases (DBRP mapping) to use InfluxQL.

  Flux works across InfluxDB 1.x, 2.x, and 3.x, providing rich metadata that helps tools like Grafana distinguish between tags and fields.

  SQL in InfluxDB 3.0 (Flight SQL) does not yet provide the same metadata as Flux does. This means fields that were once tags in Flux queries may only appear as generic string columns in SQL queries.
