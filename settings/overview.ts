import type { SettingsPage } from "@cms-local/plugin-interface";

export function formOverviewPage(actionHandler): SettingsPage {
	return {
		label: { de: "Formulare", en: "Forms" },
		type: "list",
		getDataHandler: actionHandler.toActionHandler("GET"),
		columns: [
			{ key: "name", label: { de: "Name", en: "Name" } },
			{ key: "email", label: { de: "Empfänger", en: "Recipient" } },
			{ key: "actions", label: { de: "Aktionen", en: "Actions" } },
		],
		actions: [
			{
				icon: "i-tabler-edit",
				to: "/form-plugin/create?id={{record.id}}",
			},
			{
				icon: "i-tabler-trash",
				handler: {
					url: "/api/plugins/form-plugin/forms?id={{record.id}}",
					method: "DELETE",
				},
			},
		],
	};
}
