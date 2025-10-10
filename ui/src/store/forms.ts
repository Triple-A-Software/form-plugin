import { defineStore } from "pinia";
import { useApiQuery } from "@/utils/api";

export const useForms = defineStore("forms", () => {
	const forms = useApiQuery("/api/forms", {
		method: "GET",
	});
	return { forms };
});
