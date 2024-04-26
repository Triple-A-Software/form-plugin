import type { SettingsPage } from "@cms-local/plugin-interface";
import { formBuilderItems } from "../helper/formStructs";

export function createFormPage(): SettingsPage {
	return {
		type: "form",
		getDataHandler: {
			method: "GET",
			url: "/api/plugins/form-plugin/form?id={{id}}",
		},
		label: {
			de: "Formular erstellen",
			en: "Create Form",
		},
		form: {
			label: {
				de: "Formular erstellen",
				en: "Create Form",
			},
			properties: [
				{
					id: "name",
					type: "text",
					label: {
						de: "Name des Formulars",
						en: "Name of the form",
					},
				},
				{
					id: "email",
					type: "text",
					label: {
						de: "Empfänger E-Mail",
						en: "Recipient",
					},
				},
				{
					id: "builder",
					type: "builder",
					label: {
						de: "Baue dein neues Formular",
						en: "Build your new Form",
					},
					items: formBuilderItems,
				},
			],
		},
		actions: [
			{
				primary: true,
				label: { de: "Speichern", en: "Save" },
				handler: {
					method: "POST",
					url: "/api/plugins/form-plugin/form?id={{id}}",
				},
			},
		],
	};
}
