import type { SettingsPage } from "@cms-local/plugin-interface";

export function formSubmissionsPage(submissionHandler): SettingsPage {
	return {
		label: { de: "Einsendungen", en: "Submissions" },
		type: "list",
		getDataHandler: submissionHandler.toActionHandler("GET"),
		columns: [
			{ key: "id", label: { de: "ID", en: "ID" } },
			{ key: "name", label: { de: "Formular", en: "Form" } },
			{ key: "recipient", label: { de: "Empfänger", en: "Recipient" } },
			{ key: "date", label: { de: "Gesendet am", en: "Recived at" } },
			{ key: "actions", label: { de: "Aktionen", en: "Actions" } },
		],
		actions: [
			{
				icon: "i-tabler-eye",
				showModal: true,
			},
			{
				icon: "i-tabler-trash",
				handler: {
					url: "/api/plugins/form-plugin/submit/{{record.id}}?formId={{record.formId}}",
					method: "DELETE",
				},
			},
		],
	};
}
