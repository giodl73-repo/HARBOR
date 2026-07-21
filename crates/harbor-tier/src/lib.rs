use harbor_corpus::{CorpusEntry, DemandBasis, EvidenceLabel};
use harbor_network::{Network, NetworkError};
use harbor_score::{Dimension, Score, ScoreError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub const ALL: [Self; 4] = [Self::T1, Self::T2, Self::T3, Self::T4];

    pub fn key(self) -> &'static str {
        match self {
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
            Self::T4 => "T4",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TierError> {
        match value.trim().to_ascii_uppercase().as_str() {
            "T1" => Ok(Self::T1),
            "T2" => Ok(Self::T2),
            "T3" => Ok(Self::T3),
            "T4" => Ok(Self::T4),
            other => Err(TierError::UnknownTier(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sla {
    pub tier: Tier,
    pub name: String,
    pub throughput: String,
    pub dwell: String,
    pub connectivity: String,
    pub access: String,
    pub evidence_label: EvidenceLabel,
    pub rationale: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dim13Conformance {
    pub score: Score,
    pub demand_basis: DemandBasis,
    pub basis: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gap {
    pub entry_id: String,
    pub tier: Tier,
    pub dimension: Dimension,
    pub score: Score,
    pub reason: String,
    pub basis: String,
}

pub fn classify(entry: &CorpusEntry) -> Result<Tier, TierError> {
    let tier = entry.tier.as_deref().ok_or(TierError::MissingTier)?;
    Tier::parse(tier)
}

pub fn default_sla(tier: Tier) -> Sla {
    let (name, throughput, dwell, connectivity, access) = match tier {
        Tier::T1 => (
            "International Gateway Port",
            "massive peak-season TEU capacity",
            "managed dwell under surge",
            "broad international lane and hinterland reach",
            "deep all-tide gateway access",
        ),
        Tier::T2 => (
            "National Port",
            "high national TEU capacity",
            "bounded dwell under peak-season demand",
            "strong hinterland and coastal connectivity",
            "channel adequacy for national load-center service",
        ),
        Tier::T3 => (
            "Regional / Inland Port",
            "adequate regional throughput",
            "reliable turn time for feeder demand",
            "maintained gateway or inland corridor feed",
            "regional channel or intermodal access maintained",
        ),
        Tier::T4 => (
            "Local Terminal / Berth",
            "basic local terminal capacity",
            "basic reliability and turn time",
            "lifeline or local lane connectivity",
            "maintained berth or short-sea access",
        ),
    };
    Sla {
        tier,
        name: name.to_string(),
        throughput: throughput.to_string(),
        dwell: dwell.to_string(),
        connectivity: connectivity.to_string(),
        access: access.to_string(),
        evidence_label: EvidenceLabel::Planned,
        rationale: "Provisional maritime tier-SLA record pending calibrated operational evidence."
            .to_string(),
    }
}

pub fn conformance(entry: &CorpusEntry, network: &Network) -> Result<Dim13Conformance, TierError> {
    classify(entry)?;
    let required_teu = required_capacity_teu(entry).ok_or(TierError::MissingCapacityTeu)?;
    let demand_basis = demand_basis(entry).ok_or(TierError::MissingDemandBasis)?;
    let mut observed = 0_usize;
    let mut worst_score = 10.0_f64;

    for terminus in entry
        .termini
        .iter()
        .filter(|value| !value.trim().is_empty())
    {
        observed += 1;
        let incident_teu = network.incident_capacity_teu(terminus)?;
        let score = if required_teu <= 0.0 {
            10.0
        } else {
            (incident_teu / required_teu * 10.0).clamp(0.0, 10.0)
        };
        worst_score = worst_score.min(score);
    }

    if observed == 0 {
        return Err(TierError::MissingTerminus);
    }

    Ok(Dim13Conformance {
        score: Score::new(worst_score)?,
        demand_basis,
        basis: format!("tier SLA evaluated against {demand_basis} demand basis"),
    })
}

pub fn tier_sla_gap(entry: &CorpusEntry) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    let Some(raw_score) = entry.scores.get(Dimension::Dim13.key()).copied() else {
        return Ok(None);
    };
    gap_from_score(entry, tier, Score::new(raw_score)?, demand_basis(entry))
}

pub fn tier_sla_gap_with_network(
    entry: &CorpusEntry,
    network: &Network,
) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    let conformance = conformance(entry, network)?;
    gap_from_score(
        entry,
        tier,
        conformance.score,
        Some(conformance.demand_basis),
    )
}

fn gap_from_score(
    entry: &CorpusEntry,
    tier: Tier,
    score: Score,
    demand_basis: Option<DemandBasis>,
) -> Result<Option<Gap>, TierError> {
    if score.value() >= 10.0 {
        return Ok(None);
    }
    let demand_basis = demand_basis.ok_or(TierError::MissingDemandBasis)?;
    Ok(Some(Gap {
        entry_id: entry.id.clone().ok_or(TierError::MissingEntryIdForGap)?,
        tier,
        dimension: Dimension::Dim13,
        score,
        reason: "DIM-13 score below full tier-SLA conformance".to_string(),
        basis: format!("tier SLA shortfall evaluated on {demand_basis} demand basis"),
    }))
}

fn required_capacity_teu(entry: &CorpusEntry) -> Option<f64> {
    entry
        .quantities
        .iter()
        .find(|quantity| quantity.unit.eq_ignore_ascii_case("TEU"))
        .map(|quantity| quantity.value)
}

fn demand_basis(entry: &CorpusEntry) -> Option<DemandBasis> {
    entry
        .quantities
        .iter()
        .find(|quantity| quantity.unit.eq_ignore_ascii_case("TEU"))
        .and_then(|quantity| quantity.demand_basis)
        .or_else(|| {
            entry
                .quantities
                .iter()
                .find_map(|quantity| quantity.demand_basis)
        })
}

#[derive(Debug, Error, PartialEq)]
pub enum TierError {
    #[error("missing tier")]
    MissingTier,
    #[error("unknown tier: {0}")]
    UnknownTier(String),
    #[error("missing TEU capacity quantity")]
    MissingCapacityTeu,
    #[error("missing demand basis for DIM-13 conformance")]
    MissingDemandBasis,
    #[error("missing terminus for tier-SLA conformance")]
    MissingTerminus,
    #[error("missing entry id for tier-SLA gap")]
    MissingEntryIdForGap,
    #[error(transparent)]
    Network(#[from] NetworkError),
    #[error(transparent)]
    Score(#[from] ScoreError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use harbor_corpus::{Quantity, Scale};
    use harbor_network::{Lane, Port};

    fn port(id: &str) -> Port {
        Port {
            id: id.to_string(),
            name: format!("{id} port"),
            role: "gateway".to_string(),
        }
    }
    fn lane(id: &str, capacity_teu: f64) -> Lane {
        Lane {
            id: id.to_string(),
            capacity_teu,
            basis: DemandBasis::PeakSeason,
        }
    }
    fn entry_with_tier(tier: &str) -> CorpusEntry {
        CorpusEntry {
            id: Some("entry-1".to_string()),
            scale: Some(Scale::International),
            tier: Some(tier.to_string()),
            ..CorpusEntry::default()
        }
    }
    fn meshed_network() -> Network {
        let mut network = Network::new();
        network.add_port(port("a")).expect("port a accepted");
        network.add_port(port("b")).expect("port b accepted");
        network.add_port(port("c")).expect("port c accepted");
        network
            .add_lane("a", "b", lane("ab", 80.0))
            .expect("lane ab accepted");
        network
            .add_lane("a", "c", lane("ac", 60.0))
            .expect("lane ac accepted");
        network
            .add_lane("b", "c", lane("bc", 40.0))
            .expect("lane bc accepted");
        network
    }
    fn teu_quantity(value: f64, basis: DemandBasis) -> Quantity {
        Quantity {
            value,
            unit: "TEU".to_string(),
            label: Some(EvidenceLabel::Planned),
            source_id: None,
            demand_basis: Some(basis),
        }
    }

    #[test]
    fn classify_reads_declared_tier() {
        assert_eq!(classify(&entry_with_tier("T2")), Ok(Tier::T2));
    }

    #[test]
    fn default_sla_records_provisional_label() {
        let sla = default_sla(Tier::T1);
        assert_eq!(sla.name, "International Gateway Port");
        assert_eq!(sla.evidence_label, EvidenceLabel::Planned);
        assert!(sla.rationale.contains("Provisional"));
    }

    #[test]
    fn conformance_uses_teu_capacity_for_dim13_and_names_basis() {
        let mut entry = entry_with_tier("T1");
        entry.termini.push("a".to_string());
        entry
            .quantities
            .push(teu_quantity(100.0, DemandBasis::PeakSeason));
        let result = conformance(&entry, &meshed_network()).expect("entry conforms");
        assert_eq!(result.score.value(), 10.0);
        assert_eq!(result.demand_basis, DemandBasis::PeakSeason);
        assert!(result.basis.contains("peak-season"));
    }

    #[test]
    fn tier_sla_gap_reports_dim13_shortfall() {
        let mut entry = entry_with_tier("T3");
        entry
            .quantities
            .push(teu_quantity(120.0, DemandBasis::Average));
        entry.scores.insert("DIM-13".to_string(), 6.5);
        let gap = tier_sla_gap(&entry)
            .expect("gap evaluation succeeds")
            .expect("shortfall produces gap");
        assert_eq!(gap.entry_id, "entry-1");
        assert_eq!(gap.tier, Tier::T3);
        assert_eq!(gap.dimension, Dimension::Dim13);
        assert_eq!(gap.score.value(), 6.5);
        assert!(gap.basis.contains("average"));
    }

    #[test]
    fn full_dim13_score_has_no_gap() {
        let mut entry = entry_with_tier("T4");
        entry.scores.insert("DIM-13".to_string(), 10.0);
        assert_eq!(tier_sla_gap(&entry), Ok(None));
    }
}
