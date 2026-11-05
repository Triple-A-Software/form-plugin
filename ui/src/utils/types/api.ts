import type { CreateApi } from "@blechlawine/api-toolkit";
import type * as T from "../../../../bindings";

export type API = FormsApi & EmailsApi & SubmissionsApi & SettingsApi;

type FormsApi = CreateApi<{
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
    "/api/forms/stats": {
        GET: {
            response: Array<T.FormStats>;
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
}>;

type EmailsApi = CreateApi<{
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
    "/api/emails/preview": {
        POST: {
            response: T.PreviewEmailResponse;
            body: T.PreviewEmailBody;
        };
    };
}>;

type SubmissionsApi = CreateApi<{
    "/api/submissions": {
        GET: {
            response: Array<T.SubmissionApiResponse>;
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
}>;

type SettingsApi = CreateApi<{
    "/api/settings": {
        GET: {
            response: T.Settings;
        };
        PUT: {
            body: T.Settings;
            response: T.StatusResponse;
        };
    };
    "/api/settings/test-connection": {
        POST: {
            body: T.TestConnectionBody;
            response: T.TestConnectionResponse;
        };
    };
}>;
