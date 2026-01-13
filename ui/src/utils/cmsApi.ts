import { createApiToolkit } from "@blechlawine/api-toolkit";
import { nullToUndefined, undefinedToNull } from "@blechlawine/typescript-utils";
import type { CMSApi, CMSApiError } from "./types/cmsApi";

function isError(v: unknown): v is CMSApiError {
    if (typeof v === "object" && v && "errorType" in v) {
        return true;
    }
    return false;
}

export const { useApiQuery: useCmsApiQuery } = createApiToolkit<CMSApi, CMSApiError>({
    isError,
    createError: (s) => {
        return {
            errorType: "anyhow",
            inner: s,
        };
    },
    transformer: {
        serialize: (data) => undefinedToNull(data),
        deserialize: (object) => nullToUndefined(object),
    },
    errorFromStatusCode: (_, json) => {
        if (isError(json)) {
            return json;
        }
        return {
            errorType: "anyhow",
            inner: JSON.stringify(json),
        } satisfies CMSApiError;
    },
    // the baseUrl is an empty string in production, so the neleto api will be
    // called, not the plugin proxy endpoint. In development it points to a
    // local neleto instance
    baseUrl: __CMS_API_BASE__,
});
