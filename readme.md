# Form-plugin

This is a plugin for [neleto](https://neleto.io/) to create and manage forms and newsletters.

## Overview

### Forms

In the forms tab you can create forms and manage build their interface. A form can serve as a newsletter signup form, as a contact form or to submit to a survey.

### Emails

Here you can create and manage Email-templates, which can be sent when someone submits a form, either to you or the submitter themselves.

### Submissions

When a user submits a form, you can inspect the data they've entered here. You can also send them any of you predefined emails by just pressing a button.

### Settings

You can manage the settings of this form-plugin here. This is where you configure the cleverreach integration.


## File uploads and spam protection

- **File fields** accept multiple files (`multiple`), a list of allowed types (`accept`, also checked server side) and a max size per file (default 20 MB). A submission may contain at most 20 files / 250 MB.
- Uploaded files are stored in the plugin's private storage directory and are only downloadable from the submissions page in the admin. Turn off "Attach uploaded files" on a form if the files are too large for the notification email.
- Every form contains a hidden honeypot field. Submissions that fill it in are discarded silently.
- Each client IP may send at most 10 submissions and 500 MB per hour (in memory, per plugin process).
- Rejected submissions redirect back to the page with the form (from the `Referer`, falling back to the form's redirect target) with `?error=<code>&form=<id>`. The form shows a matching message (German/English) above its fields. Codes: `rate_limited`, `upload_rejected` (too large, too many files, wrong type), `captcha_missing`, `captcha_error`, `file_rejected` (virus scan), `server_error`. Messages can be styled via `.form-plugin-error`. Without referer and redirect target the matching HTTP status is returned. Files of rejected submissions are deleted immediately.

### Virus scanning (ClamAV)

Set the ClamAV address (`host:port` of a clamd instance, e.g. `clamav:3310`) in the plugin settings. Every uploaded file is then streamed to clamd before the submission is stored or emailed. The submission is rejected unless clamd reports every file as clean, which includes clamd being unreachable, timeouts and scan errors.

clamd scans the contents of ZIP and other archives itself. To also block files that cannot be fully checked, set these options in `clamd.conf`:

```
# encrypted archives and documents are reported as Heuristics.Encrypted.*
AlertEncrypted yes
# archives exceeding the scan limits (zip bombs, too deeply nested) are reported as Heuristics.Limits.Exceeded.*
AlertExceedsMax yes
# must be at least the largest allowed upload, otherwise clamd answers with an error and the upload is rejected
StreamMaxLength 30M
MaxFileSize 30M
MaxScanSize 200M
```

The official `clamav/clamav` image works; it needs to be on the same docker network as the neleto backend and needs about 1.5 GB RAM (more while signatures reload).
