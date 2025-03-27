import type {
	RegisterHandlerReturn,
	SettingsPage,
} from "@aaa-soft/plugin-interface";
import { label } from "@aaa-soft/plugin-interface/builder";

export function formOverviewPage(
	actionHandler: RegisterHandlerReturn,
): SettingsPage {
	return {
		label: label().de("Formulare").en("Forms").build(),
		type: "list",
		getDataHandler: actionHandler.toActionHandler("GET"),
		columns: [
			{ key: "name", label: label().de("Name").en("Name").build() },
			{ key: "email", label: label().de("Empfänger").en("Recipient").build() },
			{ key: "actions", label: label().de("Aktionen").en("Actions").build() },
		],
		actions: [
			{
				icon: "i-tabler-edit",
				to: "/form-plugin/create?id={{ record.id }}",
			},
			{
				icon: "i-tabler-trash",
				handler: {
					url: `${
						actionHandler.toActionHandler("DELETE").url
					}?id={{ record.id }}`,
					method: "DELETE",
				},
			},
		],
	};
}
