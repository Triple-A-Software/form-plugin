import { DataEntry, definePlugin } from "@aaa-soft/plugin-interface";
import dayjs from "dayjs";
import { formOverviewPage } from "./settings/overview";
import { formSubmissionsPage } from "./settings/submissions";
import { createFormPage } from "./settings/form";
import { label } from "@aaa-soft/plugin-interface/builder";
import { BuilderItem } from "@aaa-soft/plugin-interface/types/builder";

type Form = {
	form: {
		name: string;
		email: string;
		builder: BuilderItem[];
	};
};
type Submission = {
	submission: {
		formId: string;
		data: unknown;
	};
};

export default definePlugin(async (ctx) => {
	const formCreateUpdateHandler = ctx.registerHandler(
		["POST", "GET"],
		"/form",
		async (req) => {
			if (req.method === "POST") {
				if (req.query.id) {
					await ctx.storage.updateOne<Form>(req.query.id, {
						form: {
							...req.body,
						},
					});
				} else {
					await ctx.storage.createOne<Form>({
						form: {
							...req.body,
						},
					});
				}
				// await hooks.trigger("render")
				return new Response(null, {
					status: 200,
				});
			}
			if (req.method === "GET") {
				if (req.query.id) {
					const dbData = await ctx.storage.getOne<Form>(req.query.id);
					if (!dbData)
						return new Response("Form with this id not found", {
							status: 404,
						});
					return dbData.data.form;
				}
				return {};
			}
			throw Error(`Method not allowed ${req.method}`);
		},
		{
			allowedRoles: ["admin", "editor", "author", "developer"],
		},
	);

	const formsHandler = ctx.registerHandler(
		["GET", "DELETE"],
		"/forms",
		async (req) => {
			if (req.method === "GET") {
				const dbData = await ctx.storage.getAllWithFilter<Form>("form");
				if (!dbData) return [];
				const allForms = dbData.map((item) => ({
					...item.data.form,
					id: item.id,
					pluginId: item.pluginId,
				}));
				return allForms;
			}
			if (req.method === "DELETE") {
				const res = await ctx.storage.deleteOne(req.query.id);
				if (res?.id) {
					return new Response("Deleted", { status: 200 });
				}
				throw Error("Error on Deleting Record", req.query.id);
			}
			throw Error(`Method not allowed ${req.method}`);
		},
		{
			allowedRoles: ["admin", "developer", "author", "editor"],
		},
	);

	// The handler to handle submissions from the website
	const submitHandler = ctx.registerHandler(
		["POST"],
		"/submit/:formId",
		async (req) => {
			if (!req.params?.formId) {
				return new Response("Form ID is missing in the URL", {
					status: 400,
				});
			}
			const form = await ctx.storage.getOne<Form | Submission>(
				req.params.formId,
			);
			if (dataentryIsForm(form)) {
				await ctx.storage.createOne({
					submission: {
						data: req.body,
						formId: req.params.formId,
					},
				});
				if (form.data.form.email) {
					ctx.sendMail(
						`[Neue Einsendung] ${form.data.form.name} vom ${dayjs(
							new Date(),
						).format("DD.MM.YYYY")}`,
						form.data.form.email,
						`<div style="padding:1rem;">
                            <table style="width:100%;border: solid 1px;padding:1rem">
                                <tr style="border-bottom:solid 1px;text-align:center">
                                    <th>Feld</th>
                                    <th></th>
                                </tr>
						${Object.entries(req.body).map(
							([key, value]) => `
						<tr>
                            <td style="border-right:solid 1px;padding:1rem">
                                ${String(key).slice(0, -1)}
                            </td>
                            <td style="padding:1rem">
                                ${value}
                            </td>
                        </tr>`,
						)}
                    </table>
                </div>`,
					);
				}
			} else {
				return new Response("Unknown FormID", {
					status: 400,
				});
			}

			return new Response(null, {
				status: 302, // statuscode for redirect
				headers: {
					location: "/",
				},
			});
		},
	);

	// The handler to list submissions and manage them from the admin-ui
	const submissionsHandler = ctx.registerHandler(
		["GET", "DELETE"],
		"/submission/:formId",
		async (req) => {
			if (!req.params?.formId) {
				return new Response("Form ID is missing in the URL", {
					status: 400,
				});
			}
			if (req.method === "DELETE") {
				const res = await ctx.storage.deleteOne(
					req.params.formId, // this is actually the submissionID
				);
				if (res) {
					return new Response(null, { status: 200 });
				}
				throw Error("Error on Deleting Record");
			}
			if (req.method === "GET") {
				const allForms = await ctx.storage.getAllWithFilter<Form>("form");
				const submissions =
					await ctx.storage.getAllWithFilter<Submission>("submission");
				const allSubmissions: Array<
					Submission["submission"] & {
						id: number;
						pluginId: string;
						name: string;
						recipient: string;
						date: string;
					}
				> = [];
				if (!submissions || !allForms) {
					return [];
				}
				for (const submission of submissions) {
					const matchingForm = allForms.find(
						(form) => form.id === Number(submission.data.submission.formId),
					);
					if (matchingForm) {
						allSubmissions.push({
							formId: submission.data.submission.formId,
							id: submission.id,
							pluginId: submission.pluginId,
							name: matchingForm.data.form.name,
							recipient: matchingForm.data.form.email,
							date: dayjs(submission.createdAt).format("HH:mm, DD.MM.YYYY"),
							data: submission.data.submission,
						});
					}
				}

				return allSubmissions;
			}
			throw new Error("Method not allowed");
		},
		{
			allowedRoles: ["admin", "editor", "author", "developer"],
		},
	);

	ctx.registerWidget(
		"formSubmissionsWidget",
		formSubmissionsPage(submissionsHandler),
	);

	ctx.registerSettingsPage(
		"/submissions",
		formSubmissionsPage(submissionsHandler),
	);
	ctx.registerSettingsPage("/overview", formOverviewPage(formsHandler));
	ctx.registerSettingsPage("/create", createFormPage(formCreateUpdateHandler));

	//TODO: Trigger hook to rerender form
	//ctx.hooks.on("rerender", () => {});

	ctx.registerElementType({
		name: "Form-Picker",
		icon: "i-tabler-forms",
		template: `<form 
    action="${submitHandler
			.toActionHandler("POST")
			.url.replace(":formId", "{{ properties.selectForm }}")}"
    class="{{ properties.cssClasses }}"
    method="POST"
>
    {{ render_builder properties.selectForm }}
</form>`,
		style: "",
		script: "",
		form: {
			label: label().en("Form-Picker").de("Form-Picker").build(),
			properties: [
				{
					label: label().en("Select Form").de("Formular Auswählen").build(),
					id: "selectForm",
					type: "pluginData",
					pluginId: ctx.pluginId(),
					filterByKey: "form",
					labelKey: "form.name",
					valueKey: "form.builder",
				},
				{
					label: label().en("CSS Classes").de("CSS Klassen").build(),
					id: "cssClasses",
					type: "text",
				},
			],
		},
	});
});

function dataentryIsForm(dataentry: unknown): dataentry is DataEntry<Form> {
	return (
		dataentry != null &&
		typeof dataentry === "object" &&
		"data" in dataentry &&
		dataentry.data != null &&
		typeof dataentry.data === "object" &&
		"form" in dataentry.data
	);
}
