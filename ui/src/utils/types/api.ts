import type * as T from "../../../../bindings";

export type API = FormsApi & EmailsApi;

type FormsApi = {
	"/api/forms": {
		GET: {
			response: Array<T.Form>;
		};
		POST: {
			response: T.StatusResponse;
			body: T.Form;
		};
		PUT: {
			response: T.StatusResponse;
			body: T.Form;
		};
		DELETE: {
			response: T.StatusResponse;
			query: T.DeleteFormQuery;
		};
	};
	"/api/forms/preview": {
		POST: {
			response: {
				html: string | null;
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
			response: T.StatusResponse;
			body: T.Email;
		};
		PUT: {
			response: T.StatusResponse;
			body: T.Email;
		};
		DELETE: {
			response: T.StatusResponse;
			query: T.DeleteEmailQuery;
		};
	};
};
