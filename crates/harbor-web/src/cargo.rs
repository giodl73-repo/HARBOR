//! Frozen publisher-source reconciliation, separate from the hypothetical gap model.
use serde::Deserialize;
use serde_json::{json, Value};

const SOURCE: &str = include_str!("../../../data/port-la-2023/source.json");
const COLUMNS: [&str; 7] = [
    "loaded_imports",
    "empty_imports",
    "total_imports",
    "loaded_exports",
    "empty_exports",
    "total_exports",
    "total_teu",
];

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Basis {
    Total,
    Loaded,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    month: Option<u8>,
    basis: Basis,
}

#[derive(Debug, Deserialize)]
struct Month {
    month: u8,
    name: String,
    centi_teu: [i64; 7],
    reported_prior_year_change: String,
}

#[derive(Debug, Deserialize)]
struct News {
    annual_centi_teu: i64,
    december_centi_teu: i64,
    publication_date: String,
    precision: String,
}

#[derive(Debug, Deserialize)]
struct Source {
    months: Vec<Month>,
    reported_calendar_year_centi_teu: [i64; 7],
    news: News,
    partial_fiscal_year: Value,
    custody: Value,
}

fn basis_value(values: &[i64; 7], basis: Basis) -> i64 {
    match basis {
        Basis::Total => values[6],
        Basis::Loaded => values[0] + values[3],
    }
}

fn component_residuals(values: &[i64; 7]) -> [i64; 3] {
    [
        values[0] + values[1] - values[2],
        values[3] + values[4] - values[5],
        values[2] + values[5] - values[6],
    ]
}

fn rounded_million(values: i64) -> String {
    let tenths = (values + 5_000_000) / 10_000_000;
    format!("{}.{:01}", tenths / 10, tenths % 10)
}

/// Query committed source data with exact integer arithmetic, in centi-TEU.
pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > super::MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB for a frozen-source query".into());
    }
    let query: Query = serde_json::from_str(input).map_err(|error| error.to_string())?;
    if query.month.is_some_and(|month| month > 12) {
        return Err("Choose all months (0), a month 1–12, or no month (null)".into());
    }
    let source: Source = serde_json::from_str(SOURCE).map_err(|error| error.to_string())?;
    if source.months.len() != 12
        || source
            .months
            .iter()
            .enumerate()
            .any(|(index, row)| usize::from(row.month) != index + 1)
    {
        return Err("Frozen source month coverage is invalid".into());
    }
    let mut monthly_sum = [0_i64; 7];
    for month in &source.months {
        for (sum, value) in monthly_sum.iter_mut().zip(month.centi_teu) {
            *sum += value;
        }
    }
    let sum_residuals: Vec<_> = monthly_sum
        .iter()
        .zip(source.reported_calendar_year_centi_teu)
        .map(|(sum, reported)| sum - reported)
        .collect();
    let rows: Vec<_> = source
        .months
        .iter()
        .filter(|row| {
            query
                .month
                .is_some_and(|month| month == 0 || month == row.month)
        })
        .map(|row| {
            json!({
                "month":row.month,"name":row.name,"centi_teu":row.centi_teu,
                "selected_basis_centi_teu":basis_value(&row.centi_teu,query.basis),
                "component_residuals_centi_teu":component_residuals(&row.centi_teu),
                "reported_prior_year_change":row.reported_prior_year_change
            })
        })
        .collect();
    let selected_sum: Option<i64> = query.month.map(|month| {
        source
            .months
            .iter()
            .filter(|row| month == 0 || month == row.month)
            .map(|row| basis_value(&row.centi_teu, query.basis))
            .sum()
    });
    let annual = source.reported_calendar_year_centi_teu;
    let table_rounded = rounded_million(annual[6]);
    let news_rounded = rounded_million(source.news.annual_centi_teu);
    serde_json::to_string(&json!({
        "schema":"harbor.frozen-port-la-container-reconciliation.v1",
        "evidence":"frozen 2026-10-26 publisher captures of 2023 local gateway throughput; not current operations",
        "scope":{"gateway":"Port of Los Angeles","scale":"local gateway source context","calendar_year":2023,"months":12},
        "unit":"centi-TEU (100 units = 1 TEU)","columns":COLUMNS,
        "selection":{"month":query.month,"basis":match query.basis{Basis::Total=>"total",Basis::Loaded=>"loaded"},"empty":rows.is_empty(),"sum_centi_teu":selected_sum},
        "rows":rows,
        "annual":{
            "reported_centi_teu":annual,"sum_of_months_centi_teu":monthly_sum,
            "sum_minus_reported_centi_teu":sum_residuals,
            "component_residuals_centi_teu":component_residuals(&annual),
            "loaded_centi_teu":basis_value(&annual,Basis::Loaded),
            "empty_centi_teu":annual[1]+annual[4],"total_centi_teu":annual[6]
        },
        "publisher_comparison":{
            "table_annual_centi_teu":annual[6],"news_annual_centi_teu":source.news.annual_centi_teu,
            "table_minus_news_annual_centi_teu":annual[6]-source.news.annual_centi_teu,
            "table_december_centi_teu":source.months[11].centi_teu[6],"news_december_centi_teu":source.news.december_centi_teu,
            "table_minus_news_december_centi_teu":source.months[11].centi_teu[6]-source.news.december_centi_teu,
            "news_publication_date":source.news.publication_date,"news_precision":source.news.precision,
            "table_total_one_decimal_million":table_rounded,"news_total_one_decimal_million":news_rounded,
            "retained_corpus_context_million":"8.6","both_round_to_corpus_context":table_rounded=="8.6"&&news_rounded=="8.6",
            "discrepancy_cause":null,"revision_chronology":null,"historical_corpus_original_source_recovered":false
        },
        "partial_fiscal_year":source.partial_fiscal_year,"custody":source.custody,
        "observations":{"capacity":null,"dwell":null,"navigation_clearance":null,"hinterland_access":null,"reliability":null,"equity":null,"costs":null,"savings":null},
        "authority":{"engineering":false,"dredging":false,"operating":false,"allocation":false,"fiscal_admission":false,"rate_change":false,"savings":false}
    }))
    .map_err(|error| error.to_string())
}

pub fn response(input: &str) -> String {
    match evaluate(input) {
        Ok(canonical) => json!({"ok":true,"result":serde_json::from_str::<Value>(&canonical).expect("serialized result"),"canonical":canonical}).to_string(),
        Err(error) => json!({"ok":false,"error":error}).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> Value {
        serde_json::from_str(&evaluate(input).unwrap()).unwrap()
    }

    #[test]
    fn annual_sums_and_component_identities_reconcile_all_seven_columns() {
        let result = run(r#"{"month":0,"basis":"total"}"#);
        assert_eq!(result["rows"].as_array().unwrap().len(), 12);
        assert_eq!(result["annual"]["total_centi_teu"], 862_968_110_i64);
        assert_eq!(
            result["annual"]["sum_minus_reported_centi_teu"],
            json!(vec![0; 7])
        );
        assert_eq!(
            result["annual"]["component_residuals_centi_teu"],
            json!(vec![0; 3])
        );
        for row in result["rows"].as_array().unwrap() {
            assert_eq!(row["component_residuals_centi_teu"], json!(vec![0; 3]));
        }
        assert_eq!(
            result["partial_fiscal_year"]["included_in_calendar_sums"],
            false
        );
        assert_eq!(
            result["partial_fiscal_year"]["centi_teu"][6],
            449_230_185_i64
        );
    }

    #[test]
    fn source_discrepancy_survives_equal_rounded_context() {
        let result = run(r#"{"month":12,"basis":"total"}"#);
        let comparison = &result["publisher_comparison"];
        assert_eq!(
            comparison["table_minus_news_annual_centi_teu"],
            -481_590_i64
        );
        assert_eq!(
            comparison["table_minus_news_december_centi_teu"],
            -481_600_i64
        );
        assert_eq!(comparison["both_round_to_corpus_context"], true);
        assert_eq!(comparison["table_total_one_decimal_million"], "8.6");
        assert_eq!(comparison["news_total_one_decimal_million"], "8.6");
        assert!(comparison["discrepancy_cause"].is_null());
        assert!(comparison["revision_chronology"].is_null());
        assert_eq!(
            comparison["historical_corpus_original_source_recovered"],
            false
        );
    }

    #[test]
    fn loaded_basis_preserves_full_totals_and_month_filter() {
        let result = run(r#"{"month":0,"basis":"loaded"}"#);
        assert_eq!(result["selection"]["sum_centi_teu"], 573_332_700_i64);
        assert_eq!(result["annual"]["empty_centi_teu"], 289_635_410_i64);
        assert_eq!(result["annual"]["total_centi_teu"], 862_968_110_i64);
        let december = run(r#"{"month":12,"basis":"loaded"}"#);
        assert_eq!(december["rows"].as_array().unwrap().len(), 1);
        assert_eq!(december["selection"]["sum_centi_teu"], 48_623_575_i64);
        assert_eq!(december["rows"][0]["centi_teu"][6], 74_251_900_i64);
    }

    #[test]
    fn no_selection_is_absent_not_zero_and_keeps_annual_context() {
        let result = run(r#"{"month":null,"basis":"total"}"#);
        assert_eq!(result["selection"]["empty"], true);
        assert!(result["selection"]["sum_centi_teu"].is_null());
        assert!(result["rows"].as_array().unwrap().is_empty());
        assert_eq!(result["annual"]["total_centi_teu"], 862_968_110_i64);
        assert!(result["observations"]
            .as_object()
            .unwrap()
            .values()
            .all(Value::is_null));
        assert!(result["authority"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v == false));
    }

    #[test]
    fn rejects_out_of_scope_queries_and_oversized_input() {
        for input in [
            r#"{"month":13,"basis":"total"}"#,
            r#"{"month":-1,"basis":"total"}"#,
            r#"{"month":1.5,"basis":"total"}"#,
            r#"{"month":0,"basis":"capacity"}"#,
            r#"{"month":0,"basis":"total","channel_depth":53}"#,
            "not JSON",
        ] {
            assert!(evaluate(input).is_err(), "{input}");
            assert_eq!(
                serde_json::from_str::<Value>(&response(input)).unwrap()["ok"],
                false
            );
        }
        assert!(evaluate(&"x".repeat(super::super::MAX_INPUT_BYTES + 1)).is_err());
    }
}
