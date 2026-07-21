use std::collections::BTreeMap;
use std::str::FromStr;

pub use harbor_network::DemandBasis;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    pub const ALL: [Self; 4] = [
        Self::International,
        Self::National,
        Self::Regional,
        Self::Local,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Self::International => "international",
            Self::National => "national",
            Self::Regional => "regional",
            Self::Local => "local",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl FromStr for Scale {
    type Err = CorpusError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "international" => Ok(Self::International),
            "national" => Ok(Self::National),
            "regional" => Ok(Self::Regional),
            "local" => Ok(Self::Local),
            other => Err(CorpusError::UnknownScale(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceLabel {
    Implemented,
    Heuristic,
    Simulated,
    Proxy,
    Planned,
    Held,
    SourceNeeded,
    ConfidenceLimited,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: Option<EvidenceLabel>,
    pub source_id: Option<String>,
    pub demand_basis: Option<DemandBasis>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    pub scale: Option<Scale>,
    pub market: Option<String>,
    pub termini: Vec<String>,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
    pub cross_scale: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationSeverity {
    Held,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn rejected(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Rejected)
    }

    pub fn held(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Held)
    }

    pub fn is_promotable(&self) -> bool {
        self.issues.is_empty()
    }
}

impl CorpusEntry {
    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::default();
        if self.id.as_deref().map(str::is_empty).unwrap_or(true) {
            report.issues.push(ValidationIssue {
                severity: ValidationSeverity::Rejected,
                reason: "missing stable element id".to_string(),
            });
        }
        if self.scale.is_none() {
            report.issues.push(ValidationIssue {
                severity: ValidationSeverity::Held,
                reason: "missing scale tag".to_string(),
            });
        }
        for quantity in &self.quantities {
            if quantity
                .source_id
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true)
                && quantity.label.is_none()
            {
                report.issues.push(ValidationIssue {
                    severity: ValidationSeverity::Held,
                    reason: format!(
                        "quantity {} {} lacks source id or evidence label",
                        quantity.value, quantity.unit
                    ),
                });
            }
        }
        report
    }

    pub fn from_markdown(markdown: &str) -> Result<Self, CorpusError> {
        let (frontmatter, body) = split_frontmatter(markdown)?;
        let mut entry = parse_frontmatter(frontmatter)?;
        entry.quantities = parse_quantities(body)?;
        entry.scores = parse_scores(body)?;
        Ok(entry)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CorpusError {
    #[error("corpus entry is missing frontmatter")]
    MissingFrontmatter,
    #[error("frontmatter line is malformed: {0}")]
    MalformedFrontmatter(String),
    #[error("quantity row is malformed: {0}")]
    MalformedQuantity(String),
    #[error("score row is malformed: {0}")]
    MalformedScore(String),
    #[error("unknown evidence label: {0}")]
    UnknownEvidenceLabel(String),
    #[error("unknown scale: {0}")]
    UnknownScale(String),
    #[error("unknown demand basis: {0}")]
    UnknownDemandBasis(String),
}

fn split_frontmatter(markdown: &str) -> Result<(&str, &str), CorpusError> {
    let Some(rest) = markdown.strip_prefix("---\n") else {
        return Err(CorpusError::MissingFrontmatter);
    };
    let Some((frontmatter, body)) = rest.split_once("\n---\n") else {
        return Err(CorpusError::MissingFrontmatter);
    };
    Ok((frontmatter, body))
}

fn parse_frontmatter(frontmatter: &str) -> Result<CorpusEntry, CorpusError> {
    let mut entry = CorpusEntry::default();
    for raw_line in frontmatter.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            return Err(CorpusError::MalformedFrontmatter(line.to_string()));
        };
        let value = value.trim();
        match key.trim() {
            "id" => entry.id = optional_string(value),
            "type" => entry.element_type = optional_string(value),
            "scale" => entry.scale = parse_optional_scale(value)?,
            "market" => entry.market = optional_string(value),
            "termini" => entry.termini = parse_string_list(value),
            "tier" => entry.tier = optional_string(value),
            "sla" => entry.sla = optional_string(value),
            "cross_scale" => entry.cross_scale = parse_bool(value),
            _ => {}
        }
    }
    Ok(entry)
}

fn parse_quantities(body: &str) -> Result<Vec<Quantity>, CorpusError> {
    let mut quantities = Vec::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        let Some(row) = line.strip_prefix("quantity:") else {
            continue;
        };
        let parts = row.split('|').map(str::trim).collect::<Vec<_>>();
        if !(4..=5).contains(&parts.len()) {
            return Err(CorpusError::MalformedQuantity(line.to_string()));
        }
        let value = parts[0]
            .parse::<f64>()
            .map_err(|_| CorpusError::MalformedQuantity(line.to_string()))?;
        quantities.push(Quantity {
            value,
            unit: parts[1].to_string(),
            label: parse_optional_label(parts[2])?,
            source_id: optional_string(parts[3]),
            demand_basis: parse_optional_demand_basis(parts.get(4).copied().unwrap_or("-"))?,
        });
    }
    Ok(quantities)
}

fn parse_scores(body: &str) -> Result<BTreeMap<String, f64>, CorpusError> {
    let mut scores = BTreeMap::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        let Some(row) = line.strip_prefix("score:") else {
            continue;
        };
        let parts = row.split('|').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 2 || parts[0].is_empty() {
            return Err(CorpusError::MalformedScore(line.to_string()));
        }
        let value = parts[1]
            .parse::<f64>()
            .map_err(|_| CorpusError::MalformedScore(line.to_string()))?;
        scores.insert(parts[0].to_string(), value);
    }
    Ok(scores)
}

fn parse_string_list(value: &str) -> Vec<String> {
    value
        .trim_matches(['[', ']'])
        .split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_matches('"');
    if trimmed.is_empty() || trimmed == "-" {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "yes" | "1"
    )
}

fn parse_optional_scale(value: &str) -> Result<Option<Scale>, CorpusError> {
    let Some(value) = optional_string(value) else {
        return Ok(None);
    };
    Scale::from_str(&value).map(Some)
}

fn parse_optional_label(value: &str) -> Result<Option<EvidenceLabel>, CorpusError> {
    let Some(value) = optional_string(value) else {
        return Ok(None);
    };
    let label = match value.as_str() {
        "implemented" => EvidenceLabel::Implemented,
        "heuristic" => EvidenceLabel::Heuristic,
        "simulated" => EvidenceLabel::Simulated,
        "proxy" => EvidenceLabel::Proxy,
        "planned" => EvidenceLabel::Planned,
        "held" => EvidenceLabel::Held,
        "source-needed" => EvidenceLabel::SourceNeeded,
        "confidence-limited" => EvidenceLabel::ConfidenceLimited,
        _ => return Err(CorpusError::UnknownEvidenceLabel(value)),
    };
    Ok(Some(label))
}

fn parse_optional_demand_basis(value: &str) -> Result<Option<DemandBasis>, CorpusError> {
    let Some(value) = optional_string(value) else {
        return Ok(None);
    };
    let basis = match value.as_str() {
        "peak-season" | "peak" | "surge" => DemandBasis::PeakSeason,
        "average" => DemandBasis::Average,
        _ => return Err(CorpusError::UnknownDemandBasis(value)),
    };
    Ok(Some(basis))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_id_is_rejected() {
        let entry = CorpusEntry {
            id: None,
            scale: Some(Scale::National),
            element_type: Some("port".to_string()),
            ..CorpusEntry::default()
        };
        let report = entry.validate();
        assert_eq!(report.rejected().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason == "missing stable element id"));
    }

    #[test]
    fn uncited_quantity_without_label_is_held() {
        let entry = CorpusEntry {
            id: Some("port:long-beach".to_string()),
            scale: Some(Scale::National),
            quantities: vec![Quantity {
                value: 9_000_000.0,
                unit: "TEU".to_string(),
                label: None,
                source_id: None,
                demand_basis: Some(DemandBasis::PeakSeason),
            }],
            ..CorpusEntry::default()
        };
        let report = entry.validate();
        assert_eq!(report.held().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason.contains("lacks source id or evidence label")));
    }

    #[test]
    fn missing_scale_is_held() {
        let entry = CorpusEntry {
            id: Some("port:long-beach".to_string()),
            scale: None,
            ..CorpusEntry::default()
        };
        let report = entry.validate();
        assert_eq!(report.held().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason == "missing scale tag"));
    }

    #[test]
    fn label_and_scale_are_preserved_from_markdown_frontmatter_entry() {
        let entry = CorpusEntry::from_markdown(
            "---\nid: port:long-beach\ntype: port\nscale: national\nmarket: us-pacific\ntermini: [long-beach]\ntier: T1\nsla: gateway-critical\n---\n\nquantity: 9000000 | TEU | source-needed | - | peak-season\n",
        )
        .expect("fixture should parse");
        assert_eq!(entry.id.as_deref(), Some("port:long-beach"));
        assert_eq!(entry.scale, Some(Scale::National));
        assert_eq!(entry.market.as_deref(), Some("us-pacific"));
        assert_eq!(entry.quantities[0].label, Some(EvidenceLabel::SourceNeeded));
        assert_eq!(
            entry.quantities[0].demand_basis,
            Some(DemandBasis::PeakSeason)
        );
        assert_eq!(entry.validate().held().count(), 0);
    }

    #[test]
    fn score_rows_are_parsed_from_body() {
        let entry = CorpusEntry::from_markdown(
            "---\nid: port:houston\ntype: port\nscale: national\n---\n\nscore: DIM-01 | 2.0\nquantity: 45 | feet-channel-depth | implemented | usace | average\n",
        )
        .expect("fixture should parse");
        assert_eq!(entry.scores.get("DIM-01").copied(), Some(2.0));
        assert_eq!(entry.quantities[0].value, 45.0);
    }

    #[test]
    fn malformed_score_row_is_rejected() {
        let err = CorpusEntry::from_markdown(
            "---\nid: port:houston\nscale: national\n---\n\nscore: DIM-01 | not-a-number\n",
        )
        .expect_err("non-numeric score should fail");
        assert!(matches!(err, CorpusError::MalformedScore(_)));
    }
}
