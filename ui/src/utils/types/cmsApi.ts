import type { CreateApi } from "@blechlawine/api-toolkit";

// this is the error type from the neleto api
export type CMSApiError =
    | { errorType: "unauthorized" }
    | { errorType: "database"; inner: string }
    | { errorType: "anyhow"; inner: string }
    | { errorType: "not-found" }
    | { errorType: "bad-request"; inner: string | undefined }
    | { errorType: "internal-server-error"; inner: string | undefined }
    | { errorType: "setup-already-finished" }
    | { errorType: "setup-not-finished" }
    | { errorType: "forbidden" }
    | { errorType: "invalid-path" }
    | { errorType: "invalid-size" }
    | { errorType: "invalid-mime-type" }
    | { errorType: "file-exists-already" }
    | { errorType: "local-file-can-not-be-imported" }
    | { errorType: "lsp"; inner: unknown }
    | { errorType: "renderer"; inner: Array<string> }
    | { errorType: "invalid-credentials" }
    | { errorType: "cannot-delete-yourself" }
    | { errorType: "passwords-do-not-match" }
    | { errorType: "invalid-query"; inner: string }
    | { errorType: "missing-files" }
    | { errorType: "failed-to-create-file" }
    | { errorType: "not-a-folder" }
    | { errorType: "file-too-big" };

export type CMSApi = CreateApi<{
    "/api/rest/settings": {
        GET: {
            query: {
                language: string;
            };
            response: {
                defaultLanguage: string;
                languages: Array<{ value: string; label: string }>;
                autoDetectBestLanguage: boolean;
                // there are more fields, but we don't care about them
            };
        };
    };
}>;
