import { type DataEntry, definePlugin } from "@aaa-soft/plugin-interface";
import dayjs from "dayjs";
import { formOverviewPage } from "./settings/overview";
import { formSubmissionsPage } from "./settings/submissions";
import { createFormPage } from "./settings/form";
import { label } from "@aaa-soft/plugin-interface/builder";
import type { BuilderItem } from "@aaa-soft/plugin-interface/types/builder";
import { HookName } from "@aaa-soft/cms-hooks";

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

	function getMailContent(title, body){
		return (
		`
		<html lang="de">
		<head>
			<meta charset="utf-8"/>
			<style>
        		table, th, td {
            		padding: 5px 10px;
            		text-align: left;
        		}
    		</style>
		</head>
		<body>
			<b>${title}</b>
			<br/>
			<table> 
				<thead>
					<tr>
						<th>Feld</th>
						<th>Inhalt</th>
					</tr>
				</thead>
				<tbody>
				${Object.entries(body).map(
					([key, value]) => `
					<tr>
						<td>
							${key}
						</td>
						<td>
							${value}
						</td>
					</tr>`
				).join("")}
				</tbody>
			</table>
			<br/>
		</body>
		</html>`
		)
	}

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
				await ctx.hooks.callHook(HookName.PAGE_CACHE_UPDATE_ALL);
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
				return new Response("FormID not given", {
					status: 400,
				});
			}
			return new Response(`Method not allowed ${req.method}`, {
				status: 405,
			});
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
				if (!res)
					return new Response("form not found", {
						status: 404,
					});
				if (res?.id) {
					return new Response("Deleted", { status: 200 });
				}
				return new Response(`Error on deleting record ${req.query.id}`, {
					status: 500,
				});
			}
			return new Response(`Method not allowed ${req.method}`, {
				status: 405,
			});
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
				const title = `[Neue Einsendung] ${form.data.form.name} vom ${dayjs(
					new Date(),
				).format("DD.MM.YYYY")}`
				if (form.data.form.email) {
					ctx.sendMail(
						title,
						form.data.form.email,
						getMailContent(title, req.body)
					);
				}
			} else {
				return new Response("Unknown FormID", {
					status: 404,
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

	const submissionsListHandler = ctx.registerHandler(
		["GET"],
		"/submission",
		async (req) => {
			// list all submissions
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
			return allSubmissions.sort((a,b) => (a.id > b.id ? -1 : 1));
		},
	);

	// The handler to list submissions and manage them from the admin-ui
	const submissionsDeleteHandler = ctx.registerHandler(
		["DELETE"],
		"/submission/:submissionId",
		async (req) => {
			if (!req.params?.submissionId) {
				return new Response("Submission ID is missing in the URL", {
					status: 400,
				});
			}
			const res = await ctx.storage.deleteOne(req.params.submissionId);
			if (res) {
				return new Response(null, { status: 200 });
			}
			return new Response("Error on Deleting Record", {
				status: 500,
			});
		},
		{
			allowedRoles: ["admin", "editor", "author", "developer"],
		},
	);

	// ctx.registerWidget(
	// 	"formSubmissionsWidget",
	// 	formSubmissionsPage(submissionsListHandler, submissionsDeleteHandler),
	// );

	ctx.registerSettingsPage(
		"/submissions",
		formSubmissionsPage(submissionsListHandler, submissionsDeleteHandler),
	);
	ctx.registerSettingsPage("/overview", formOverviewPage(formsHandler));
	ctx.registerSettingsPage("/create", createFormPage(formCreateUpdateHandler));

	ctx.registerElementType({
		name: "Form-Picker",
		icon: "i-tabler-forms",
		category: "misc",
		template: `<form 
    action="${submitHandler
			.toActionHandler("POST")
			.url.replace(":formId", "{{ properties.selectForm.id }}")}"
    class="{{ relations.selectForm.cssClasses }}"
    method="POST"
>
    {{ render_builder relations.selectForm.builder }}
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
					valueKey: "form",
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
