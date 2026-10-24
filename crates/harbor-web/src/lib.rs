//! Historical repository-model replay; edits are hypothetical, never new findings.
use harbor_corpus::{CorpusEntry, Scale};
use harbor_gap::{find_gaps, null_result, GapSource};
use harbor_score::{Dimension, Rubric, Score};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

pub const MAX_INPUT_BYTES: usize = 65_536;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gateway {
    pub id: String,
    pub name: String,
    pub tier: String,
    pub score: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub scale: String,
    pub rubric: String,
    pub entries: Vec<Gateway>,
}

pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > MAX_INPUT_BYTES {
        return Err("Use at most 64 KiB of hypothetical gateway scores".into());
    }
    let input: Input = serde_json::from_str(input).map_err(|error| error.to_string())?;
    if input.scale != "national" || input.rubric != "v0" {
        return Err("Keep the declared national scope and native v0 rubric".into());
    }
    if input.entries.is_empty() || input.entries.len() > 16 {
        return Err("Use 1–16 aggregate gateway entries".into());
    }
    let mut seen = HashSet::new();
    let mut corpus = Vec::new();
    let mut missing = Vec::new();
    for entry in &input.entries {
        if entry.id.is_empty()
            || entry.id.len() > 64
            || !entry
                .id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-_:".contains(&c))
            || !seen.insert(&entry.id)
        {
            return Err("Use unique short aggregate gateway identifiers".into());
        }
        if entry.name.trim().is_empty()
            || entry.name.len() > 80
            || entry.name.chars().any(char::is_control)
        {
            return Err("Use a bounded gateway display name".into());
        }
        let mut scores = BTreeMap::new();
        if let Some(value) = entry.score {
            Score::new(value).map_err(|error| error.to_string())?;
            scores.insert(Dimension::Dim01.key().to_string(), value);
        } else {
            missing.push(entry.id.clone());
        }
        corpus.push(CorpusEntry {
            id: Some(entry.id.clone()),
            element_type: Some("port".into()),
            scale: Some(Scale::National),
            market: Some("repository-model-replay".into()),
            tier: Some(entry.tier.clone()),
            scores,
            ..CorpusEntry::default()
        });
    }
    let rubric = Rubric::default_v0();
    let gaps = find_gaps(&corpus, &rubric, Scale::National).map_err(|error| error.to_string())?;
    let overall_null = null_result("browser-replay", &corpus, &rubric, Scale::National)
        .map_err(|error| error.to_string())?;
    let channel: Vec<_> = gaps
        .iter()
        .filter(|gap| gap.dimension == Dimension::Dim01)
        .collect();
    let coverage: Vec<_> = gaps
        .iter()
        .filter(|gap| gap.source == GapSource::EmptyRegion)
        .map(|gap| gap.dimension.key())
        .collect();
    let classification = if channel
        .iter()
        .any(|gap| gap.source == GapSource::EmptyRegion)
    {
        "not-assessed"
    } else if channel
        .iter()
        .any(|gap| gap.source == GapSource::SystemicRegion)
    {
        "systemic"
    } else if channel
        .iter()
        .any(|gap| gap.source == GapSource::TailRegion)
    {
        "concentrated-tail"
    } else if channel
        .iter()
        .any(|gap| gap.source == GapSource::UnderServedRegion)
    {
        "minimum-alert-only"
    } else {
        "no-detected-DIM01-gap"
    };
    serde_json::to_string(&serde_json::json!({
        "schema":"harbor.hypothetical-gateway-distribution.v1",
        "evidence":"hypothetical replay of the repository June 2026 illustration; not current channel clearance",
        "scope":{"scale":"national","dimension":"DIM-01 scoped channel-adequacy illustration","rubric":"v0","source_context":"repository corpus with 2023 source context"},
        "threshold":gaps.first().map(|gap|gap.threshold.value()),
        "classification":classification,"scored_entries":corpus.len()-missing.len(),"missing_entry_ids":missing,
        "entries":input.entries,"channel_regions":channel,"all_gap_regions":gaps,"unassessed_dimensions":coverage,"overall_null":overall_null,
        "authority":{"engineering":false,"dredging":false,"navigation_clearance":false,"savings":false,"allocation":false,"rate_change":false}
    })).map_err(|error|error.to_string())
}

pub fn response(input: &str) -> String {
    match evaluate(input){Ok(canonical)=>serde_json::json!({"ok":true,"canonical":canonical,"result":serde_json::from_str::<serde_json::Value>(&canonical).expect("serialized result")}).to_string(),Err(error)=>serde_json::json!({"ok":false,"error":error}).to_string()}
}
#[cfg(target_arch = "wasm32")]
mod abi;

#[cfg(test)]
mod tests {
    use super::*;
    const RESET: &str = include_str!("../../../fixtures/browser-reset.json");
    #[test]
    fn replays_the_native_tail_and_preserves_coverage_gaps() {
        let value: serde_json::Value = serde_json::from_str(&evaluate(RESET).unwrap()).unwrap();
        assert_eq!(value["classification"], "concentrated-tail");
        assert_eq!(value["unassessed_dimensions"].as_array().unwrap().len(), 12);
        let tail = value["channel_regions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|gap| gap["source"] == "TailRegion")
            .unwrap();
        assert_eq!(tail["entry_ids"].as_array().unwrap().len(), 2);
        assert!(value["overall_null"].is_null());
    }
    #[test]
    fn systemic_and_missing_scores_remain_distinct() {
        let systemic = include_str!("../../../fixtures/browser-systemic.json");
        let missing = include_str!("../../../fixtures/browser-missing.json");
        let a: serde_json::Value = serde_json::from_str(&evaluate(systemic).unwrap()).unwrap();
        let b: serde_json::Value = serde_json::from_str(&evaluate(missing).unwrap()).unwrap();
        assert_eq!(a["classification"], "systemic");
        assert_eq!(b["scored_entries"], 7);
        assert_eq!(b["missing_entry_ids"].as_array().unwrap().len(), 1);
        assert!(b["entries"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["score"].is_null()));
    }
    #[test]
    fn rejects_scale_score_identity_and_size_drift() {
        assert!(evaluate(&RESET.replace("national", "local")).is_err());
        let mut input: serde_json::Value = serde_json::from_str(RESET).unwrap();
        input["entries"][0]["score"] = serde_json::json!(11);
        assert!(evaluate(&input.to_string()).is_err());
        assert!(evaluate(&"x".repeat(65537)).is_err());
        assert!(evaluate(&RESET.replacen('{', "{\"person_name\":\"example\",", 1)).is_err());
    }
}
