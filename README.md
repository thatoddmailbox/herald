# herald
Herald is a email notification processor. It takes automated reports sent from other mail servers, parses them, and stores them in a MySQL database for further analysis. Currently, it only processes DMARC reports, but at some point TLSRPT support will also be added.

Note that herald does not directly receive the reports&mdash;your mail server does. Herald logs into your mail server (over IMAP) and downloads the reports. (and does _not_ delete them&mdash;the original report emails are left in place)

Also note that this is still a work-in-progress. It's possible that breaking changes may be made at some point in the future.

## Usage
You should set up an email account on your mail server to receive reports, and update your domain's DNS records such that DMARC reports are sent to that account.

When you first run herald, it will create a config.toml file. Edit this file to configure your MySQL connection and email account information. Then, whenever you run herald, it will log in, check for new reports, and, if any are found, add them to the database.