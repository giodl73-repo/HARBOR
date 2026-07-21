use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use harbor_corpus::{CorpusEntry, Scale, ValidationSeverity};
use harbor_gap::{find_gaps, null_result, GapRegion, NullResult};
use harbor_score::{Dimension, DimensionScorer, Rubric, StoredScoreScorer};
use harbor_tier::{classify, tier_sla_gap, Gap as TierGap, Tier};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "harbor")]
#[command(about = "HARBOR product CLI: corpus, score, tier-sla, and gap artifacts")]
struct Cli {
    /// Scale filter: international, national, regional, or local.
    #[arg(long, global = true, default_value = "national")]
    scale: Scale,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate corpus markdown while preserving evidence labels and scale.
    Corpus(ArtifactArgs),
    /// Emit DIM-01..13 score artifacts from corpus entries.
    Score(ArtifactArgs),
    /// Emit tier-SLA classification and shortfall artifacts.
    TierSla(ArtifactArgs),
    /// Emit scale-filtered gap analysis and null-result artifacts.
    Gap(ArtifactArgs),
}

#[derive(Debug, Clone, Parser)]
struct ArtifactArgs {
    /// Corpus markdown file or directory of .md files.
    #[arg(long)]
    input: PathBuf,
    /// Optional JSON artifact target. When omitted, JSON is written to stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run(Cli::parse()) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    let scale = cli.scale;
    match cli.command {
        Command::Corpus(args) => write_artifact(
            &args,
            corpus_artifact(&entries_for_corpus(&load_entries(&args.input)?, scale)),
        ),
        Command::Score(args) => {
            write_artifact(&args, score_artifact(&load_entries(&args.input)?, scale)?)
        }
        Command::TierSla(args) => write_artifact(
            &args,
            tier_sla_artifact(&load_entries(&args.input)?, scale)?,
        ),
        Command::Gap(args) => {
            write_artifact(&args, gap_artifact(&load_entries(&args.input)?, scale)?)
        }
    }
}

fn load_entries(input: &Path) -> Result<Vec<CorpusEntry>, CliError> {
    let mut paths = Vec::new();
    if input.is_dir() {
        for entry in fs::read_dir(input)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                paths.push(path);
            }
        }
        paths.sort();
    } else {
        paths.push(input.to_path_buf());
    }
    let mut entries = Vec::new();
    for path in paths {
        let body = fs::read_to_string(&path)?
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        let body = body.strip_prefix('\u{feff}').unwrap_or(&body);
        // Skip companion docs (e.g. SCHEMA.md) that are not corpus entries.
        if !body.trim_start().starts_with("---") {
            continue;
        }
        let mut entry = CorpusEntry::from_markdown(body)?;
        entry.scores.extend(parse_score_rows(body)?);
        entries.push(entry);
    }
    Ok(entries)
}

fn parse_score_rows(body: &str) -> Result<Vec<(String, f64)>, CliError> {
    let mut scores = Vec::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        let Some(row) = line.strip_prefix("score:") else {
            continue;
        };
        let parts = row.split('|').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(CliError::MalformedScore(line.to_string()));
        }
        scores.push((
            parts[0].to_string(),
            parts[1]
                .parse::<f64>()
                .map_err(|_| CliError::MalformedScore(line.to_string()))?,
        ));
    }
    Ok(scores)
}

fn write_artifact<T: Serialize>(args: &ArtifactArgs, artifact: T) -> Result<(), CliError> {
    let body = serde_json::to_string_pretty(&artifact)?;
    if let Some(output) = &args.output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output, format!("{body}\n"))?;
    } else {
        println!("{body}");
    }
    Ok(())
}

fn entries_for_scale(entries: &[CorpusEntry], scale: Scale) -> Vec<CorpusEntry> {
    entries
        .iter()
        .filter(|entry| entry.scale == Some(scale) || entry.cross_scale)
        .cloned()
        .collect()
}

fn entries_for_corpus(entries: &[CorpusEntry], scale: Scale) -> Vec<CorpusEntry> {
    entries
        .iter()
        .filter(|entry| entry.scale.is_none() || entry.scale == Some(scale) || entry.cross_scale)
        .cloned()
        .collect()
}

fn corpus_artifact(entries: &[CorpusEntry]) -> CorpusArtifact {
    let mut rows = Vec::new();
    for entry in entries {
        let report = entry.validate();
        rows.push(CorpusValidationRow {
            id: entry.id.clone(),
            scale: entry.scale,
            promotable: report.is_promotable(),
            held: report
                .issues
                .iter()
                .filter(|issue| issue.severity == ValidationSeverity::Held)
                .map(|issue| issue.reason.clone())
                .collect(),
            rejected: report
                .issues
                .iter()
                .filter(|issue| issue.severity == ValidationSeverity::Rejected)
                .map(|issue| issue.reason.clone())
                .collect(),
        });
    }
    CorpusArtifact {
        artifact: "corpus-validation-v0",
        entries: rows,
    }
}

fn score_artifact(entries: &[CorpusEntry], scale: Scale) -> Result<ScoreArtifact, CliError> {
    let scorer = StoredScoreScorer::default();
    let mut rows = Vec::new();
    for entry in entries_for_scale(entries, scale) {
        let mut scores = Vec::new();
        for dimension in Dimension::ALL {
            scores.push(DimensionScore {
                dimension: dimension.key().to_string(),
                score: scorer.score(&entry, dimension)?.value(),
            });
        }
        rows.push(ScoreRow {
            id: entry.id.clone().ok_or(CliError::MissingEntryId)?,
            scale: entry.scale,
            scores,
        });
    }
    Ok(ScoreArtifact {
        artifact: "score-v0",
        scale,
        rubric_version: Rubric::default_v0().version,
        entries: rows,
    })
}

fn tier_sla_artifact(entries: &[CorpusEntry], scale: Scale) -> Result<TierSlaArtifact, CliError> {
    let mut rows = Vec::new();
    for entry in entries_for_scale(entries, scale) {
        let id = entry.id.clone().ok_or(CliError::MissingEntryId)?;
        rows.push(TierSlaRow {
            id,
            scale: entry.scale,
            tier: classify(&entry)?,
            gap: tier_sla_gap(&entry)?,
        });
    }
    Ok(TierSlaArtifact {
        artifact: "tier-sla-v0",
        scale,
        entries: rows,
    })
}

fn gap_artifact(entries: &[CorpusEntry], scale: Scale) -> Result<GapArtifact, CliError> {
    let rubric = Rubric::default_v0();
    let gaps = find_gaps(entries, &rubric, scale)?;
    let null = if gaps.is_empty() {
        null_result("near-optimal-region", entries, &rubric, scale)?
    } else {
        None
    };
    Ok(GapArtifact {
        artifact: "gap-v0",
        scale,
        gaps,
        null_result: null,
    })
}

#[derive(Debug, Serialize)]
struct CorpusArtifact {
    artifact: &'static str,
    entries: Vec<CorpusValidationRow>,
}
#[derive(Debug, Serialize)]
struct CorpusValidationRow {
    id: Option<String>,
    scale: Option<Scale>,
    promotable: bool,
    held: Vec<String>,
    rejected: Vec<String>,
}
#[derive(Debug, Serialize)]
struct ScoreArtifact {
    artifact: &'static str,
    scale: Scale,
    rubric_version: String,
    entries: Vec<ScoreRow>,
}
#[derive(Debug, Serialize)]
struct ScoreRow {
    id: String,
    scale: Option<Scale>,
    scores: Vec<DimensionScore>,
}
#[derive(Debug, Serialize)]
struct DimensionScore {
    dimension: String,
    score: f64,
}
#[derive(Debug, Serialize)]
struct TierSlaArtifact {
    artifact: &'static str,
    scale: Scale,
    entries: Vec<TierSlaRow>,
}
#[derive(Debug, Serialize)]
struct TierSlaRow {
    id: String,
    scale: Option<Scale>,
    tier: Tier,
    gap: Option<TierGap>,
}
#[derive(Debug, Serialize)]
struct GapArtifact {
    artifact: &'static str,
    scale: Scale,
    gaps: Vec<GapRegion>,
    null_result: Option<NullResult>,
}

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Corpus(#[from] harbor_corpus::CorpusError),
    #[error(transparent)]
    Score(#[from] harbor_score::ScoreError),
    #[error(transparent)]
    Tier(#[from] harbor_tier::TierError),
    #[error(transparent)]
    Gap(#[from] harbor_gap::GapError),
    #[error("score row is malformed: {0}")]
    MalformedScore(String),
    #[error("entry requires stable id for CLI artifact")]
    MissingEntryId,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_corpus() -> String {
        let mut body = String::from("---\nid: port:seed\ntype: port\nscale: national\nmarket: seed-market\ntermini: [seed-port]\ntier: T1\n---\nquantity: 50 | TEU | planned | - | peak-season\n");
        for dimension in Dimension::ALL {
            body.push_str(&format!("score: {} | 10\n", dimension.key()));
        }
        body
    }

    fn test_file(name: &str) -> PathBuf {
        let root = std::env::current_dir()
            .expect("current dir")
            .join("target")
            .join("harbor-cli-tests");
        fs::create_dir_all(&root).expect("test target dir");
        root.join(name)
    }

    #[test]
    fn corpus_score_tier_and_gap_artifacts_are_deterministic() {
        let input = test_file("seed.md");
        fs::write(&input, seed_corpus()).expect("seed corpus write");
        let entries = load_entries(&input).expect("entries load");
        let corpus = serde_json::to_string(&corpus_artifact(&entries)).expect("corpus json");
        let score = serde_json::to_string(
            &score_artifact(&entries, Scale::National).expect("score artifact"),
        )
        .expect("score json");
        let tier = serde_json::to_string(
            &tier_sla_artifact(&entries, Scale::National).expect("tier artifact"),
        )
        .expect("tier json");
        let gap = gap_artifact(&entries, Scale::National).expect("gap artifact");
        assert!(corpus.contains("corpus-validation-v0"));
        assert!(score.contains("\"DIM-13\""));
        assert!(tier.contains("tier-sla-v0"));
        assert!(gap.gaps.is_empty());
        assert_eq!(
            gap.null_result.expect("null result").region_id,
            "near-optimal-region"
        );
    }

    #[test]
    fn help_surface_lists_product_subcommands_and_scale_filter() {
        use clap::CommandFactory;
        let mut command = Cli::command();
        let help = command.render_long_help().to_string();
        let subcommands = command
            .get_subcommands()
            .map(|subcommand| subcommand.get_name().to_string())
            .collect::<Vec<_>>();
        assert!(help.contains("corpus"));
        assert!(help.contains("score"));
        assert!(help.contains("tier-sla"));
        assert!(help.contains("gap"));
        assert!(help.contains("--scale"));
        assert!(!subcommands.iter().any(|name| name == "work-package"));
        assert!(!subcommands.iter().any(|name| name == "prove"));
        assert!(!subcommands.iter().any(|name| name == "evidence"));
    }
}
