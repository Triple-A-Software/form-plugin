import { useApiQuery } from "@/utils/api";

export function useForms() {
    return useApiQuery("/api/forms", {
        method: "GET",
    });
}
