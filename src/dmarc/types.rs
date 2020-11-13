use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ReportDateRange {
	begin: u64,
	end: u64
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ReportMetadata {
	org_name: String,
	email: String,
	extra_contact_info: String,
	report_id: String,
	date_range: ReportDateRange,
	error: String
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Policy {
	domain: String,
	adkim: String,
	aspf: String,
	p: String,
	sp: String,
	pct: u8
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordRowPolicy {
	disposition: String,
	dkim: String,
	spf: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordRow {
	source_ip: String,
	count: u32,
	policy_evaluated: RecordRowPolicy
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordIdentifiers {
	header_from: String
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordDKIMResult {
	domain: String,
	result: String,
	selector: String
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordSPFResult {
	domain: String,
	scope: String,
	result: String
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RecordResults {
	dkim: Vec<RecordDKIMResult>,
	spf: Vec<RecordSPFResult>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Record {
	row: RecordRow,
	identifiers: RecordIdentifiers,
	auth_results: RecordResults
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Report {
	report_metadata: ReportMetadata,
	policy_published: Policy,
	record: Vec<Record>
}
