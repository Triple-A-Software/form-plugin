import type {
	RegisterHandlerReturn,
	SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";

export function formSubmissionsPage(
	listHandler: RegisterHandlerReturn,
	deleteHandler: RegisterHandlerReturn,
): SettingsPage {
	return {
		label: label().de("Einsendungen").en("Submissions").build(),
		type: "list",
		getDataHandler: listHandler.toActionHandler("GET"),
		columns: [
			{ key: "id", label: label().de("ID").en("ID").build() },
			{ key: "name", label: label().de("Formular").en("Form").build() },
			{
				key: "recipient",
				label: label().de("Empfänger").en("Recipient").build(),
			},
			{
				key: "date",
				label: label().de("Gesendet am").en("Recived at").build(),
			},
			{ key: "actions", label: label().de("Aktionen").en("Actions").build() },
		],
		actions: [
			{
				icon: "i-tabler-eye",
				showModal: true,
			},
			{
				icon: "i-tabler-trash",
				handler: {
					url: deleteHandler
						.toActionHandler("DELETE")
						.url.replace(":submissionId", "{{ record.id }}"),
					method: "DELETE",
				},
			},
		],
	};
}
