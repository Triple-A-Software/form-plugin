import type {
	RegisterHandlerReturn,
	SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";
import formBuilderItems from "../helper/formStructs";

export function createFormPage(
	formHandler: RegisterHandlerReturn,
): SettingsPage {
	const dataHandler = formHandler.toActionHandler("GET");
	const upsertHandler = formHandler.toActionHandler("POST");

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
					id: "send_confirmation_email",
					type: "checkbox",
					label: label()
						.en("Send confirmation email to visitor")
						.de("Bestätigungs Email an Besucher schicken")
						.build(),
				},
				{
					id: "confirmation_email_subject",
					type: "text",
					label: label()
						.en("Confirmation email subject")
						.de("Bestätigungsemailbetreff")
						.build(),
				},
				{
					id: "confirmation_email_content",
					type: "code",
					language: "html",
					label: label()
						.en("Confirmation email content")
						.de("Bestätigungsemailinhalt")
						.build(),
				},
				{
					label: label().en("Select Captcha").de("Captcha auswählen").build(),
					id: "captcha-select",
					type: "select",
					options: [
						{ label: { de: "Ohne Captcha", en: "None" }, value: "none" },
						{ label: { de: "reCAPTCHA", en: "reCAPTCHA" }, value: "recaptcha" },
						{ label: { de: "h-Captcha", en: "h-Captcha" }, value: "hcaptcha" },
					],
				},
				{
					label: label()
						.en("Activate Cleverreach")
						.de("Aktivere Cleverreach")
						.build(),
					id: "cleverreach-active",
					type: "checkbox",
				},
				{
					label: label().en("Group Ids").de("Group Ids").build(),
					id: "cleverreach_group_id",
					type: "text",
				},
				{
					label: label().en("CSS Classes").de("CSS Klassen").build(),
					id: "cssClasses",
					type: "text",
				},
				{
					id: "builder",
					type: "builder",
					label: label()
						.en("Build your Form")
						.de("Bearbeite dein Formular")
						.build(),

					items: formBuilderItems as [],
					previewTemplate: `<form 
    action=""
    class="{{ properties.cssClasses }}"
    method="POST"
>
    {{ render_builder properties.builder }}
</form>`,
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
