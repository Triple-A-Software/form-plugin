import type {
	RegisterHandlerReturn,
	SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";
import { formBuilderItems } from "../helper/formStructs";

export function createFormPage(
	formHandler: RegisterHandlerReturn,
	formCreateUpdateHandler: RegisterHandlerReturn,
): SettingsPage {
	const dataHandler = formHandler.toActionHandler("GET");
	const upsertHandler = formCreateUpdateHandler.toActionHandler("POST");

	return {
		type: "form",
		getDataHandler: {
			method: dataHandler.method,
			url: `${dataHandler.url}?id={{id}}`,
		},
		label: label().en("Create Form").de("Formular erstellen").build(),
		form: {
			label: label().en("Create Form").de("Formular erstellen").build(),
			properties: [
				{
					id: "name",
					type: "text",
					label: label()
						.en("Name of the form")
						.de("Name des Formulars")
						.build(),
				},
				{
					id: "email",
					type: "text",
					label: label()
						.en("Send submissions to this email")
						.de("Formularergebnisse an diese Email schicken")
						.build(),
				},
				{
					id: "builder",
					type: "builder",
					label: label()
						.en("Build your Form")
						.de("Bearbeite dein Formular")
						.build(),
					items: formBuilderItems,
				},
			],
		},
		actions: [
			{
				primary: true,
				label: label().en("Save").de("Speichern").build(),
				handler: {
					method: upsertHandler.method,
					url: `${upsertHandler.url}?id={{id}}`,
					redirect: "/plugins/form-plugin/overview",
				},
			},
		],
	};
}
