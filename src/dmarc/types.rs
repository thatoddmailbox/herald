use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ReportDateRange {
	pub begin: u64,
	pub end: u64
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ReportMetadata {
	pub org_name: String,
	pub email: String,
	pub extra_contact_info: String,
	pub report_id: String,
	pub date_range: ReportDateRange,
	pub error: String
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Policy {
	pub domain: String,
	pub adkim: String,
	pub aspf: String,
	pub p: String,
	pub sp: String,
	pub pct: u8
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RecordRowPolicy {
	pub disposition: String,
	pub dkim: String,
	pub spf: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordRow {
	pub source_ip: String,
	pub count: u32,
	pub policy_evaluated: RecordRowPolicy
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RecordIdentifiers {
	pub header_from: String
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RecordDKIMResult {
	pub domain: String,
	pub result: String,
	pub selector: String
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RecordSPFResult {
	pub domain: String,
	pub scope: String,
	pub result: String
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RecordResults {
	pub dkim: Vec<RecordDKIMResult>,
	pub spf: Vec<RecordSPFResult>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Record {
	pub row: RecordRow,
	pub identifiers: RecordIdentifiers,
	pub auth_results: RecordResults
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Report {
	pub report_metadata: ReportMetadata,
	pub policy_published: Policy,
	pub record: Vec<Record>
}
