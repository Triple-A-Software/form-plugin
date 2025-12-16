import { createApiToolkit } from "@blechlawine/api-toolkit";
import {
    nullToUndefined,
    undefinedToNull,
} from "@blechlawine/typescript-utils";
import type { ApiError } from "../../../bindings";
import type { API } from "./types/api";
import { useI18n } from "vue-i18n";

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

export const { useApiMutation, useApiQuery } = createApiToolkit<API, ApiError>({
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
    toastBuilder: () => {
        const toast = useToast();
        const { t, te } = useI18n();
        return {
            success: (path, method) => {
                const tKey = `toast.${path}.${String(method)}.success`;
                if (te(tKey)) {
                    toast.add({
                        title: t(tKey),
                        color: "success",
                        icon: "i-tabler-check",
                    });
                }
            },
            error: (path, method) => {
                const tKey = `toast.${path}.${String(method)}.error`;
                if (te(tKey)) {
                    toast.add({
                        title: t(tKey),
                        color: "error",
                        icon: "i-tabler-alert-circle",
                    });
                }
            },
        };
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
