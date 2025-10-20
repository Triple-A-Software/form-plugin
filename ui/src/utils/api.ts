import { createApiToolkit } from "api-toolkit";
import { nullToUndefined, undefinedToNull } from "typescript-utils";
import type { ApiError } from "../../../bindings";
import type { API } from "./types/api";

function isError(v: unknown): v is ApiError {
    if (
        typeof v === "object" &&
        v &&
        "error_message" in v &&
        typeof v.error_message === "string"
    ) {
        return true;
    }
    return false;
}

export const { useApiMutation, useApiQuery, fetchApi } = createApiToolkit<
    API,
    ApiError
>({
    baseUrl: __API_BASE__,
    createError(s) {
        return {
            error_message: s,
        };
    },
    isError,
    transformer: {
        serialize: (data) => undefinedToNull(data),
        deserialize: (object) => nullToUndefined(object),
    },
    errorFromStatusCode: (statusCode: number, json: unknown) => {
        if (isError(json)) {
            return json;
        }
        return {
            error_message: JSON.stringify(json),
        } satisfies ApiError;
    },
});
