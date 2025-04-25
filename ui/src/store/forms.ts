import { useApiQuery } from "@/utils/api";
import { defineStore } from "pinia";

export const useForms = defineStore("forms", () => {
	const forms = useApiQuery(
		"/api/forms",
		{
			method: "GET",
		},
		{
			initialData: () => [],
		},
	);
	return { forms };
});
