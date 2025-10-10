import type * as T from "../../../../bindings";

export type API = FormsApi & EmailsApi & SubmissionsApi & SettingsApi;

type FormsApi = {
	"/api/forms": {
		GET: {
			response: Array<T.Form>;
		};
		POST: {
			response: T.Form;
			body: T.Form;
		};
		PUT: {
			response: T.StatusResponse;
			body: T.Form;
		};
		DELETE: {
			response: T.StatusResponse;
			body: T.DeleteFormBody;
		};
	};
	"/api/forms/preview": {
		POST: {
			response: {
				html: string | undefined;
			};
			body: T.Form;
		};
	};
};

type EmailsApi = {
	"/api/emails": {
		GET: {
			response: Array<T.Email>;
		};
		POST: {
			response: T.Email;
			body: T.Email;
		};
		PUT: {
			response: T.StatusResponse;
			body: T.Email;
		};
		DELETE: {
			response: T.StatusResponse;
			body: T.DeleteEmailBody;
		};
	};
};

type SubmissionsApi = {
	"/api/submissions": {
		GET: {
			response: Array<T.Submission>;
			query: T.ListSubmissionsQuery;
		};
		DELETE: {
			response: T.StatusResponse;
			body: T.DeleteSubmissionBody;
		};
	};
	"/api/submissions/count": {
		GET: {
			response: number;
			query: T.ListSubmissionsQuery;
		};
	};
	"/api/submissions/recent": {
		GET: {
			response: Array<T.Submission>;
		};
	};
	"/api/submissions/send-email": {
		POST: {
			response: T.StatusResponse;
			body: T.SendEmailToSubmissionBody;
		};
	};
	"/api/submissions/stats": {
		GET: {
			response: T.SubmissionStats;
		};
	};
};

type SettingsApi = {
	"/api/settings": {
		GET: {
			response: T.Settings;
		};
		PUT: {
			body: T.Settings;
			response: T.StatusResponse;
		};
	};
};
