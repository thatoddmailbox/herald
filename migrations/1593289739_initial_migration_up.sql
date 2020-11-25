-- Description: Initial migration
-- Up migration

CREATE TABLE `dmarc_organizations` (
	`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`name` text NOT NULL,
	`email` text NOT NULL,
	`extra_contact_info` text NOT NULL
);

CREATE TABLE `dmarc_reports` (
	`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`organization_id` int NOT NULL,
	`report_id` text NOT NULL,
	`begin` int NOT NULL,
	`end` int NOT NULL,
	`policy_published` text NOT NULL,
	`message_id` text NOT NULL,
	`received_at` int NOT NULL,
	`processed_at` int NOT NULL,
	FOREIGN KEY (`organization_id`) REFERENCES `dmarc_organizations` (`id`)
);

CREATE TABLE `dmarc_records` (
	`id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,
	`source_ip` text NOT NULL,
	`count` int NOT NULL,
	`policy_evaluated` text NOT NULL,
	`identifiers` text NOT NULL,
	`auth_results` text NOT NULL,
	`report_id` int NOT NULL,
	FOREIGN KEY (`report_id`) REFERENCES `dmarc_reports` (`id`)
);