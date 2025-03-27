import type {
	RegisterHandlerReturn,
	SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";

export function cleverReachConfig(
	formHandler: RegisterHandlerReturn,
): SettingsPage {
	const dataHandler = formHandler.toActionHandler("GET");
	const upsertHandler = formHandler.toActionHandler("POST");

	return {
		type: "form",
		getDataHandler: {
			method: dataHandler.method,
			url: `${dataHandler.url}`,
		},
		label: label().en("Cleverreach Settings").de("Cleverreach Einstellungen").build(),
		form: {
			label: label().en("Cleverreach Settings").de("Cleverreach Einstellungen").build(),
			properties: [
				{
					id: "client_id",
					type: "text",
					label: label()
						.en("OAuth Client Id")
						.de("OAuth Client Id")
						.build(),
				},
				{
					label: label().en("OAuth Cleverreach password").de("OAuth Cleverreach Passwort").build(),
					id: "login_password",
					type: "password",
				}
			],
		},
		actions: [
			{
				primary: true,
				label: label().en("Save").de("Speichern").build(),
				handler: {
					method: upsertHandler.method,
					url: `${upsertHandler.url}`,
				},
			},
		],
	};
}