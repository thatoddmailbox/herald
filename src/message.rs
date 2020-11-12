use super::types;
use imap_proto::types::BodyStructure;

/// Searches the given IMAP BodyStructure for a report file.
pub fn find_report(body_structure: &BodyStructure, prefix: String) -> Option<(String, types::ReportFileType)> {
	match body_structure {
		BodyStructure::Multipart { common: _, bodies, extension: _ } => {
			// unwrap the multipart message
			for (i, body) in bodies.iter().enumerate() {
				let mut actual_prefix = prefix.clone();
				if actual_prefix != "" {
					actual_prefix += ".";
				}
				let result = find_report(body, actual_prefix + &i.to_string());
				if result.is_some() {
					return result;
				}
			}

			None
		},

		BodyStructure::Basic { common, other: _, extension: _ } => {
			// this might be our attachment
			// check if it's a filetype we know about
			let filetype_option = match (
				common.ty.ty.to_lowercase().as_str(),
				common.ty.subtype.to_lowercase().as_str()
			) {
				("application", "gzip") => Some(types::ReportFileType::Gzip),
				("application", "zip") => Some(types::ReportFileType::Zip),
				_ => None,
			};

			let part_number = match prefix.as_str() {
				"" => "1".to_string(),
				_ => prefix,
			};

			match filetype_option {
				Some(filetype) => Some((part_number, filetype)),
				None => None,
			}
		},

		BodyStructure::Message { common: _, other: _, envelope: _, body: _, lines: _, extension: _, } => {
			// we shouldn't get this
			// ignore it

			None
		},
		BodyStructure::Text { common: _, other: _, lines: _, extension: _ } => {
			// don't care about text, it's probably just some human-readable message
			// ignore it

			None
		},
	}
}